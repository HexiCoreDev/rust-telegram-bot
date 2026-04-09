//! Rate-limiting for Telegram Bot API requests.
//!
//! Port of `telegram.ext._baseratelimiter` and `telegram.ext._aioratelimiter`.
//! Uses a simple token-bucket algorithm implemented with `tokio::sync::Semaphore`
//! and `tokio::time` instead of the `aiolimiter` Python library.
//!
//! # Rate-limited request wrapping
//!
//! The [`RateLimitedRequest`] adapter wraps any [`BaseRequest`] implementation
//! and calls the rate limiter before each HTTP request is dispatched.  This is
//! the cleanest interception point -- it operates at the HTTP transport layer
//! so all API methods are rate-limited transparently without touching the
//! `Bot` / `ExtBot` call sites.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Notify, Semaphore};
use tracing::info;

use rust_tg_bot_raw::error::TelegramError;
use rust_tg_bot_raw::request::base::{BaseRequest, HttpMethod, TimeoutOverride};
use rust_tg_bot_raw::request::request_data::RequestData;

// ---------------------------------------------------------------------------
// BaseRateLimiter trait
// ---------------------------------------------------------------------------

/// The abstract rate-limiter interface. Mirrors Python's `BaseRateLimiter`.
///
/// Implementations must be `Send + Sync`.
///
/// The type parameter `RLArgs` allows callers to pass per-request rate-limit
/// hints (e.g. max retries, priority level).
pub trait BaseRateLimiter<RLArgs = i32>: Send + Sync {
    /// Initialize any resources used by this rate limiter.
    fn initialize(&self) -> impl Future<Output = ()> + Send;

    /// Shut down and release resources.
    fn shutdown(&self) -> impl Future<Output = ()> + Send;

    /// Process a single request through the rate limiter.
    ///
    /// The implementation **must** call `callback` at some point and return
    /// its result. *When* the callback is invoked is up to the implementation
    /// (it may delay the call to stay within rate limits).
    fn process_request<F, Fut>(
        &self,
        callback: F,
        endpoint: &str,
        data: &HashMap<String, serde_json::Value>,
        rate_limit_args: Option<RLArgs>,
    ) -> impl Future<Output = Result<serde_json::Value, TelegramError>> + Send
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<serde_json::Value, TelegramError>> + Send;
}

// ---------------------------------------------------------------------------
// DynRateLimiter -- object-safe trait for use as trait object
// ---------------------------------------------------------------------------

/// Boxed callback type for the dynamic rate limiter.
type BoxedCallback = Box<
    dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<serde_json::Value, TelegramError>> + Send>>
        + Send,
>;

/// Object-safe rate-limiter trait so we can store it as a trait object inside
/// [`ExtBot`](crate::ext_bot::ExtBot).
///
/// Unlike [`BaseRateLimiter`], `process_request` takes **owned** parameters
/// and a boxed callback, making the trait object-safe and avoiding lifetime
/// issues when the future outlives the caller's stack frame.
pub trait DynRateLimiter: Send + Sync + std::fmt::Debug {
    /// Initialize the rate limiter.
    fn initialize(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// Shut down the rate limiter.
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// Process a single request through the rate limiter.
    fn process_request(
        &self,
        callback: BoxedCallback,
        endpoint: String,
        data: HashMap<String, serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, TelegramError>> + Send + '_>>;
}

// ---------------------------------------------------------------------------
// RateLimitedRequest -- BaseRequest wrapper that applies rate limiting
// ---------------------------------------------------------------------------

/// A [`BaseRequest`] adapter that applies rate limiting before each HTTP call.
///
/// This wraps an inner [`BaseRequest`] implementation and a [`DynRateLimiter`].
/// Before each `do_request` or `do_request_json_bytes` call, it passes the
/// actual HTTP call as a callback to the rate limiter, which decides when to
/// invoke it.
///
/// This is the recommended way to wire rate limiting into the bot pipeline:
/// construct a `RateLimitedRequest`, then pass it to [`Bot::new`](rust_tg_bot_raw::bot::Bot::new)
/// as the request backend.
///
/// # Example
///
/// ```rust,ignore
/// let request = Arc::new(ReqwestRequest::new()?);
/// let limiter = Arc::new(AioRateLimiter::default_limits());
/// let rate_limited = RateLimitedRequest::new(request, limiter);
/// let bot = Bot::new("token", Arc::new(rate_limited));
/// ```
pub struct RateLimitedRequest {
    inner: Arc<dyn BaseRequest>,
    limiter: Arc<dyn DynRateLimiter>,
}

impl std::fmt::Debug for RateLimitedRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimitedRequest")
            .field("limiter", &self.limiter)
            .finish_non_exhaustive()
    }
}

impl RateLimitedRequest {
    /// Create a new rate-limited request wrapper.
    pub fn new(inner: Arc<dyn BaseRequest>, limiter: Arc<dyn DynRateLimiter>) -> Self {
        Self { inner, limiter }
    }
}

/// Convert request parameters from string-encoded JSON values (as returned by
/// `RequestData::json_parameters`) to `serde_json::Value` for the rate
/// limiter's `data` map. The limiter reads fields like `chat_id` from this
/// map to determine per-group throttling.
fn params_to_value_map(request_data: Option<&RequestData>) -> HashMap<String, serde_json::Value> {
    let Some(rd) = request_data else {
        return HashMap::new();
    };
    rd.json_parameters()
        .into_iter()
        .map(|(k, v)| {
            let value = serde_json::from_str(&v).unwrap_or_else(|_| serde_json::Value::String(v));
            (k, value)
        })
        .collect()
}

#[async_trait::async_trait]
impl BaseRequest for RateLimitedRequest {
    async fn initialize(&self) -> rust_tg_bot_raw::error::Result<()> {
        self.limiter.initialize().await;
        self.inner.initialize().await
    }

    async fn shutdown(&self) -> rust_tg_bot_raw::error::Result<()> {
        self.limiter.shutdown().await;
        self.inner.shutdown().await
    }

    fn default_read_timeout(&self) -> Option<Duration> {
        self.inner.default_read_timeout()
    }

    async fn do_request(
        &self,
        url: &str,
        method: HttpMethod,
        request_data: Option<&RequestData>,
        timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        let endpoint = url.rsplit('/').next().unwrap_or(url).to_owned();
        let data = params_to_value_map(request_data);

        let inner = self.inner.clone();
        let url_owned = url.to_owned();
        let request_data_clone = request_data.cloned();

        let result = self
            .limiter
            .process_request(
                Box::new(move || {
                    Box::pin(async move {
                        let rd_ref = request_data_clone.as_ref();
                        let (status, body) = inner
                            .do_request(&url_owned, method, rd_ref, timeouts)
                            .await?;
                        Ok(serde_json::json!({
                            "__status": status,
                            "__body": serde_json::Value::String(
                                base64_encode(&body)
                            ),
                        }))
                    })
                }),
                endpoint,
                data,
            )
            .await?;

        let status = result["__status"].as_u64().unwrap_or(200) as u16;
        let body_b64 = result["__body"].as_str().unwrap_or("");
        let body = base64_decode(body_b64);

        Ok((status, bytes::Bytes::from(body)))
    }

    async fn do_request_json_bytes(
        &self,
        url: &str,
        body: &[u8],
        timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        let endpoint = url.rsplit('/').next().unwrap_or(url).to_owned();
        let data: HashMap<String, serde_json::Value> =
            serde_json::from_slice(body).unwrap_or_default();

        let inner = self.inner.clone();
        let url_owned = url.to_owned();
        let body_owned = body.to_vec();

        let result = self
            .limiter
            .process_request(
                Box::new(move || {
                    Box::pin(async move {
                        let (status, resp_body) = inner
                            .do_request_json_bytes(&url_owned, &body_owned, timeouts)
                            .await?;
                        Ok(serde_json::json!({
                            "__status": status,
                            "__body": serde_json::Value::String(
                                base64_encode(&resp_body)
                            ),
                        }))
                    })
                }),
                endpoint,
                data,
            )
            .await?;

        let status = result["__status"].as_u64().unwrap_or(200) as u16;
        let body_b64 = result["__body"].as_str().unwrap_or("");
        let resp_body = base64_decode(body_b64);

        Ok((status, bytes::Bytes::from(resp_body)))
    }
}

// ---------------------------------------------------------------------------
// Base64 helpers for lossless binary round-trip through JSON Value
// ---------------------------------------------------------------------------

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut out = Vec::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(ALPHABET[((triple >> 18) & 0x3F) as usize]);
        out.push(ALPHABET[((triple >> 12) & 0x3F) as usize]);
        if chunk.len() > 1 {
            out.push(ALPHABET[((triple >> 6) & 0x3F) as usize]);
        } else {
            out.push(b'=');
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(triple & 0x3F) as usize]);
        } else {
            out.push(b'=');
        }
    }
    String::from_utf8(out).unwrap_or_default()
}

fn base64_decode(input: &str) -> Vec<u8> {
    fn char_val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }

    let bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'=' && b != b'\n' && b != b'\r')
        .collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);

    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 {
            break;
        }
        let a = char_val(chunk[0]).unwrap_or(0);
        let b = char_val(chunk[1]).unwrap_or(0);
        let c = if chunk.len() > 2 {
            char_val(chunk[2]).unwrap_or(0)
        } else {
            0
        };
        let d = if chunk.len() > 3 {
            char_val(chunk[3]).unwrap_or(0)
        } else {
            0
        };

        let triple = (a << 18) | (b << 12) | (c << 6) | d;

        out.push(((triple >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            out.push(((triple >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            out.push((triple & 0xFF) as u8);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Token-bucket implementation
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TokenBucket {
    semaphore: Arc<Semaphore>,
    max_rate: u32,
    #[allow(dead_code)]
    time_period: Duration,
    _replenish_handle: tokio::task::JoinHandle<()>,
}

impl TokenBucket {
    fn new(max_rate: u32, time_period: Duration) -> Self {
        let semaphore = Arc::new(Semaphore::new(max_rate as usize));
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move {
            let interval = time_period / max_rate;
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                if sem.available_permits() < max_rate as usize {
                    sem.add_permits(1);
                }
            }
        });
        Self {
            semaphore,
            max_rate,
            time_period,
            _replenish_handle: handle,
        }
    }

    async fn acquire(&self) {
        let permit = self.semaphore.acquire().await;
        if let Ok(permit) = permit {
            permit.forget();
        }
    }
}

// ---------------------------------------------------------------------------
// Shared throttling logic used by both AioRateLimiter and its DynRateLimiter impl
// ---------------------------------------------------------------------------

/// Helper: acquire rate-limit permits and invoke the callback.
///
/// Extracted so both `BaseRateLimiter::process_request` (which takes `&str` +
/// `&HashMap`) and `DynRateLimiter::process_request` (which takes owned
/// `String` + `HashMap`) can share the same throttling logic.
async fn throttle_and_call<F, Fut>(
    base_limiter: &Option<TokenBucket>,
    group_max_rate: u32,
    group_time_period: Duration,
    group_limiters: &Mutex<HashMap<GroupId, Arc<TokenBucket>>>,
    max_retries: u32,
    retry_after_notify: &Arc<Notify>,
    retry_after_active: &std::sync::atomic::AtomicBool,
    callback: F,
    data: &HashMap<String, serde_json::Value>,
) -> Result<serde_json::Value, TelegramError>
where
    F: FnOnce() -> Fut + Send,
    Fut: Future<Output = Result<serde_json::Value, TelegramError>> + Send,
{
    let chat_id_val = data.get("chat_id");
    let has_chat = chat_id_val.is_some();

    let group_id: Option<GroupId> = chat_id_val.and_then(|v| {
        if let Some(n) = v.as_i64() {
            if n < 0 {
                return Some(GroupId::Int(n));
            }
        }
        if let Some(s) = v.as_str() {
            if let Ok(n) = s.parse::<i64>() {
                if n < 0 {
                    return Some(GroupId::Int(n));
                }
            }
            return Some(GroupId::Str(s.to_owned()));
        }
        None
    });

    // Acquire per-group permit.
    if let Some(gid) = &group_id {
        if group_max_rate > 0 {
            let limiter = {
                let mut map = group_limiters.lock().await;
                if map.len() > 512 {
                    let gid_clone = gid.clone();
                    map.retain(|k, bucket| {
                        k == &gid_clone
                            || bucket.semaphore.available_permits() < bucket.max_rate as usize
                    });
                }
                map.entry(gid.clone())
                    .or_insert_with(|| {
                        Arc::new(TokenBucket::new(group_max_rate, group_time_period))
                    })
                    .clone()
            };
            limiter.acquire().await;
        }
    }

    // Acquire global permit.
    if has_chat {
        if let Some(ref bl) = base_limiter {
            bl.acquire().await;
        }
    }

    // Wait for any active retry-after window.
    if retry_after_active.load(std::sync::atomic::Ordering::Relaxed) {
        retry_after_notify.notified().await;
    }

    let result = callback().await;

    match result {
        Err(TelegramError::RetryAfter { retry_after }) if max_retries > 0 => {
            let sleep_dur = retry_after + Duration::from_millis(100);
            info!(
                "Rate limit hit. Retrying after {:.1}s",
                sleep_dur.as_secs_f64()
            );
            retry_after_active.store(true, std::sync::atomic::Ordering::Relaxed);
            tokio::time::sleep(sleep_dur).await;
            retry_after_active.store(false, std::sync::atomic::Ordering::Relaxed);
            retry_after_notify.notify_waiters();
            Err(TelegramError::RetryAfter { retry_after })
        }
        other => other,
    }
}

// ---------------------------------------------------------------------------
// AioRateLimiter (concrete implementation)
// ---------------------------------------------------------------------------

/// Rate limiter that applies two levels of throttling:
///
/// 1. **Overall**: limits the total number of requests per time period.
/// 2. **Per-group**: limits requests to any single group/channel.
///
/// Additionally, a `RetryAfter` error from Telegram will pause *all*
/// requests for the specified duration.
#[derive(Debug)]
pub struct AioRateLimiter {
    base_limiter: Option<TokenBucket>,
    group_max_rate: u32,
    group_time_period: Duration,
    group_limiters: Mutex<HashMap<GroupId, Arc<TokenBucket>>>,
    max_retries: u32,
    retry_after_notify: Arc<Notify>,
    retry_after_active: std::sync::atomic::AtomicBool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GroupId {
    Int(i64),
    Str(String),
}

/// Default messages per second (Telegram limit).
const DEFAULT_OVERALL_MAX_RATE: u32 = 30;
/// Default messages per minute per group (Telegram limit).
const DEFAULT_GROUP_MAX_RATE: u32 = 20;

impl AioRateLimiter {
    /// Create a new rate limiter with the given parameters.
    ///
    /// Pass `0` for a rate or period to disable that level of limiting.
    pub fn new(
        overall_max_rate: u32,
        overall_time_period: Duration,
        group_max_rate: u32,
        group_time_period: Duration,
        max_retries: u32,
    ) -> Self {
        let base_limiter = if overall_max_rate > 0 && !overall_time_period.is_zero() {
            Some(TokenBucket::new(overall_max_rate, overall_time_period))
        } else {
            None
        };

        Self {
            base_limiter,
            group_max_rate,
            group_time_period,
            group_limiters: Mutex::new(HashMap::new()),
            max_retries,
            retry_after_notify: Arc::new(Notify::new()),
            retry_after_active: false.into(),
        }
    }

    /// Create with Telegram's standard rate limits.
    pub fn default_limits() -> Self {
        Self::new(
            DEFAULT_OVERALL_MAX_RATE,
            Duration::from_secs(1),
            DEFAULT_GROUP_MAX_RATE,
            Duration::from_secs(60),
            0,
        )
    }
}

impl BaseRateLimiter<i32> for AioRateLimiter {
    async fn initialize(&self) {}
    async fn shutdown(&self) {}

    async fn process_request<F, Fut>(
        &self,
        callback: F,
        _endpoint: &str,
        data: &HashMap<String, serde_json::Value>,
        rate_limit_args: Option<i32>,
    ) -> Result<serde_json::Value, TelegramError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<serde_json::Value, TelegramError>> + Send,
    {
        let max_retries = rate_limit_args.map_or(self.max_retries, |n| n as u32);
        throttle_and_call(
            &self.base_limiter,
            self.group_max_rate,
            self.group_time_period,
            &self.group_limiters,
            max_retries,
            &self.retry_after_notify,
            &self.retry_after_active,
            callback,
            data,
        )
        .await
    }
}

impl DynRateLimiter for AioRateLimiter {
    fn initialize(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    fn shutdown(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    fn process_request(
        &self,
        callback: BoxedCallback,
        _endpoint: String,
        data: HashMap<String, serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, TelegramError>> + Send + '_>> {
        Box::pin(async move {
            throttle_and_call(
                &self.base_limiter,
                self.group_max_rate,
                self.group_time_period,
                &self.group_limiters,
                self.max_retries,
                &self.retry_after_notify,
                &self.retry_after_active,
                callback,
                &data,
            )
            .await
        })
    }
}

// ---------------------------------------------------------------------------
// No-op limiter
// ---------------------------------------------------------------------------

/// A rate limiter that does nothing (pass-through).
#[derive(Debug, Clone, Copy)]
pub struct NoRateLimiter;

impl BaseRateLimiter<i32> for NoRateLimiter {
    async fn initialize(&self) {}
    async fn shutdown(&self) {}

    async fn process_request<F, Fut>(
        &self,
        callback: F,
        _endpoint: &str,
        _data: &HashMap<String, serde_json::Value>,
        _rate_limit_args: Option<i32>,
    ) -> Result<serde_json::Value, TelegramError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<serde_json::Value, TelegramError>> + Send,
    {
        callback().await
    }
}

impl DynRateLimiter for NoRateLimiter {
    fn initialize(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    fn shutdown(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    fn process_request(
        &self,
        callback: BoxedCallback,
        _endpoint: String,
        _data: HashMap<String, serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, TelegramError>> + Send + '_>> {
        Box::pin(async move { callback().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn no_rate_limiter_passes_through() {
        let limiter = NoRateLimiter;
        BaseRateLimiter::initialize(&limiter).await;

        let result = BaseRateLimiter::process_request(
            &limiter,
            || async { Ok(serde_json::json!({"ok": true})) },
            "sendMessage",
            &HashMap::new(),
            None,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["ok"], true);
        BaseRateLimiter::shutdown(&limiter).await;
    }

    #[tokio::test]
    async fn aio_rate_limiter_basic() {
        let limiter =
            AioRateLimiter::new(10, Duration::from_secs(1), 5, Duration::from_secs(60), 0);
        BaseRateLimiter::initialize(&limiter).await;

        let result = BaseRateLimiter::process_request(
            &limiter,
            || async { Ok(serde_json::json!({"result": 42})) },
            "getMe",
            &HashMap::new(),
            None,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["result"], 42);
        BaseRateLimiter::shutdown(&limiter).await;
    }

    #[tokio::test]
    async fn aio_rate_limiter_group_detection() {
        let limiter =
            AioRateLimiter::new(100, Duration::from_secs(1), 100, Duration::from_secs(60), 0);

        let mut data = HashMap::new();
        data.insert(
            "chat_id".to_owned(),
            serde_json::Value::Number((-100i64).into()),
        );

        let result = BaseRateLimiter::process_request(
            &limiter,
            || async { Ok(serde_json::json!({"ok": true})) },
            "sendMessage",
            &data,
            None,
        )
        .await;

        assert!(result.is_ok());

        let groups = limiter.group_limiters.lock().await;
        assert!(groups.contains_key(&GroupId::Int(-100)));
    }

    #[tokio::test]
    async fn dyn_rate_limiter_no_op() {
        let limiter: Arc<dyn DynRateLimiter> = Arc::new(NoRateLimiter);
        DynRateLimiter::initialize(limiter.as_ref()).await;

        let result = DynRateLimiter::process_request(
            limiter.as_ref(),
            Box::new(|| {
                Box::pin(async { Ok(serde_json::json!({"ok": true})) })
                    as Pin<Box<dyn Future<Output = _> + Send>>
            }),
            "sendMessage".to_owned(),
            HashMap::new(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["ok"], true);
        DynRateLimiter::shutdown(limiter.as_ref()).await;
    }

    #[tokio::test]
    async fn dyn_rate_limiter_aio() {
        let limiter: Arc<dyn DynRateLimiter> = Arc::new(AioRateLimiter::default_limits());
        DynRateLimiter::initialize(limiter.as_ref()).await;

        let result = DynRateLimiter::process_request(
            limiter.as_ref(),
            Box::new(|| {
                Box::pin(async { Ok(serde_json::json!({"result": 99})) })
                    as Pin<Box<dyn Future<Output = _> + Send>>
            }),
            "getMe".to_owned(),
            HashMap::new(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["result"], 99);
        DynRateLimiter::shutdown(limiter.as_ref()).await;
    }

    #[test]
    fn base64_round_trip() {
        let original = b"hello world! \x00\xFF\xAB";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn base64_empty() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_decode(""), Vec::<u8>::new());
    }

    #[test]
    fn base64_json_body() {
        let json = br#"{"ok":true,"result":[]}"#;
        let encoded = base64_encode(json);
        let decoded = base64_decode(&encoded);
        assert_eq!(decoded, json.to_vec());
    }

    #[test]
    fn params_to_value_map_converts_strings() {
        use rust_tg_bot_raw::request::request_parameter::RequestParameter;

        let params = vec![
            RequestParameter::new("chat_id", serde_json::json!(-100)),
            RequestParameter::new("text", serde_json::json!("hello")),
        ];
        let rd = RequestData::from_parameters(params);
        let map = params_to_value_map(Some(&rd));

        assert_eq!(map.get("chat_id"), Some(&serde_json::json!(-100)));
        assert_eq!(map.get("text"), Some(&serde_json::json!("hello")));
    }

    #[test]
    fn params_to_value_map_none() {
        let map = params_to_value_map(None);
        assert!(map.is_empty());
    }
}

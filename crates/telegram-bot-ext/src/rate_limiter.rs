//! Rate-limiting for Telegram Bot API requests.
//!
//! Port of `telegram.ext._baseratelimiter` and `telegram.ext._aioratelimiter`.
//! Uses a simple token-bucket algorithm implemented with `tokio::sync::Semaphore`
//! and `tokio::time` instead of the `aiolimiter` Python library.

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Notify, Semaphore};
use tracing::info;

use rust_tg_bot_raw::error::TelegramError;

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
// Token-bucket implementation
// ---------------------------------------------------------------------------

/// A simple asynchronous token bucket that replenishes `max_rate` tokens
/// every `time_period`.
#[derive(Debug)]
struct TokenBucket {
    semaphore: Arc<Semaphore>,
    max_rate: u32,
    #[allow(dead_code)]
    time_period: Duration,
    /// Background replenish task handle.
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
        // Forget the permit so it is only returned by the replenish task.
        if let Ok(permit) = permit {
            permit.forget();
        } else {
            // Semaphore closed -- should not happen.
        }
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
    /// When a `RetryAfter` is in effect, waiters block on this `Notify`.
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

    async fn get_group_limiter(&self, group_id: GroupId) -> Arc<TokenBucket> {
        let mut map = self.group_limiters.lock().await;

        // Evict stale limiters when the map grows large.
        if map.len() > 512 {
            map.retain(|k, bucket| {
                k == &group_id || bucket.semaphore.available_permits() < bucket.max_rate as usize
            });
        }

        map.entry(group_id)
            .or_insert_with(|| {
                Arc::new(TokenBucket::new(
                    self.group_max_rate,
                    self.group_time_period,
                ))
            })
            .clone()
    }

    async fn wait_retry_after(&self) {
        if self
            .retry_after_active
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.retry_after_notify.notified().await;
        }
    }
}

impl BaseRateLimiter<i32> for AioRateLimiter {
    async fn initialize(&self) {
        // Nothing to do -- resources are created in `new`.
    }

    async fn shutdown(&self) {
        // Token-bucket replenish tasks will be cancelled when the
        // `JoinHandle` is dropped (when `Self` is dropped).
    }

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

        // Determine if this is a group/channel request.
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

        // Acquire rate-limit permits.
        if let Some(gid) = &group_id {
            if self.group_max_rate > 0 {
                let limiter = self.get_group_limiter(gid.clone()).await;
                limiter.acquire().await;
            }
        }
        if has_chat {
            if let Some(ref bl) = self.base_limiter {
                bl.acquire().await;
            }
        }

        // Wait for any active retry-after window.
        self.wait_retry_after().await;

        // We only have one shot at the callback since FnOnce.
        // For retries, we'd need FnMut or a cloneable factory. Since the
        // Python API also only calls callback once per process_request
        // invocation in the non-retry path, we match that behavior and handle
        // retries at a higher level. The retry logic below is illustrative for
        // a single attempt; a real multi-retry version would accept FnMut.
        let result = callback().await;

        match result {
            Err(TelegramError::RetryAfter { retry_after }) if max_retries > 0 => {
                let sleep_dur = retry_after + Duration::from_millis(100);
                info!(
                    "Rate limit hit. Retrying after {:.1}s",
                    sleep_dur.as_secs_f64()
                );
                self.retry_after_active
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                tokio::time::sleep(sleep_dur).await;
                self.retry_after_active
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                self.retry_after_notify.notify_waiters();
                // We cannot retry a FnOnce callback; propagate the error so
                // the caller can retry at a higher level.
                Err(TelegramError::RetryAfter { retry_after })
            }
            other => other,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn no_rate_limiter_passes_through() {
        let limiter = NoRateLimiter;
        limiter.initialize().await;

        let result = limiter
            .process_request(
                || async { Ok(serde_json::json!({"ok": true})) },
                "sendMessage",
                &HashMap::new(),
                None,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["ok"], true);
        limiter.shutdown().await;
    }

    #[tokio::test]
    async fn aio_rate_limiter_basic() {
        let limiter =
            AioRateLimiter::new(10, Duration::from_secs(1), 5, Duration::from_secs(60), 0);
        limiter.initialize().await;

        let result = limiter
            .process_request(
                || async { Ok(serde_json::json!({"result": 42})) },
                "getMe",
                &HashMap::new(),
                None,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["result"], 42);
        limiter.shutdown().await;
    }

    #[tokio::test]
    async fn aio_rate_limiter_group_detection() {
        let limiter =
            AioRateLimiter::new(100, Duration::from_secs(1), 100, Duration::from_secs(60), 0);
        limiter.initialize().await;

        let mut data = HashMap::new();
        data.insert(
            "chat_id".to_owned(),
            serde_json::Value::Number((-100i64).into()),
        );

        let result = limiter
            .process_request(
                || async { Ok(serde_json::json!({"ok": true})) },
                "sendMessage",
                &data,
                None,
            )
            .await;

        assert!(result.is_ok());

        // Verify a group limiter was created.
        let groups = limiter.group_limiters.lock().await;
        assert!(groups.contains_key(&GroupId::Int(-100)));
        drop(groups);

        limiter.shutdown().await;
    }
}

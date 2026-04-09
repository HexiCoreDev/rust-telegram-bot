//! Typestate builder for [`Application`](crate::application::Application).

use std::marker::PhantomData;
use std::sync::Arc;

use rust_tg_bot_raw::bot::Bot;
use rust_tg_bot_raw::request::base::BaseRequest;

#[cfg(feature = "persistence")]
use crate::application::DynPersistence;
use crate::application::{Application, ApplicationConfig, LifecycleHook};
use crate::context_types::ContextTypes;
use crate::defaults::Defaults;
use crate::ext_bot::ExtBot;
#[cfg(feature = "job-queue")]
use crate::job_queue::JobQueue;
#[cfg(feature = "rate-limiter")]
use crate::rate_limiter::{DynRateLimiter, RateLimitedRequest};
use crate::update_processor;

// ---------------------------------------------------------------------------
// Typestate markers
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct NoToken;

#[derive(Debug)]
pub struct HasToken;

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

pub struct ApplicationBuilder<State = NoToken> {
    token: Option<String>,
    request: Option<Arc<dyn BaseRequest>>,
    base_url: Option<String>,
    base_file_url: Option<String>,
    defaults: Option<Defaults>,
    arbitrary_callback_data: Option<usize>,
    #[cfg(feature = "rate-limiter")]
    rate_limiter: Option<Arc<dyn DynRateLimiter>>,
    #[cfg(not(feature = "rate-limiter"))]
    rate_limiter: Option<()>,
    context_types: Option<ContextTypes>,
    concurrent_updates: usize,
    post_init: Option<LifecycleHook>,
    post_stop: Option<LifecycleHook>,
    post_shutdown: Option<LifecycleHook>,
    #[cfg(feature = "persistence")]
    persistence: Option<Box<dyn DynPersistence>>,
    #[cfg(feature = "job-queue")]
    job_queue: Option<Arc<JobQueue>>,
    _marker: PhantomData<State>,
}

impl Default for ApplicationBuilder<NoToken> {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationBuilder<NoToken> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            token: None,
            request: None,
            base_url: None,
            base_file_url: None,
            defaults: None,
            arbitrary_callback_data: None,
            rate_limiter: None,
            context_types: None,
            concurrent_updates: 1,
            post_init: None,
            post_stop: None,
            post_shutdown: None,
            #[cfg(feature = "persistence")]
            persistence: None,
            #[cfg(feature = "job-queue")]
            job_queue: None,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn token(self, token: impl Into<String>) -> ApplicationBuilder<HasToken> {
        ApplicationBuilder {
            token: Some(token.into()),
            request: self.request,
            base_url: self.base_url,
            base_file_url: self.base_file_url,
            defaults: self.defaults,
            arbitrary_callback_data: self.arbitrary_callback_data,
            rate_limiter: self.rate_limiter,
            context_types: self.context_types,
            concurrent_updates: self.concurrent_updates,
            post_init: self.post_init,
            post_stop: self.post_stop,
            post_shutdown: self.post_shutdown,
            #[cfg(feature = "persistence")]
            persistence: self.persistence,
            #[cfg(feature = "job-queue")]
            job_queue: self.job_queue,
            _marker: PhantomData,
        }
    }
}

// Methods available in *any* state.
impl<S> ApplicationBuilder<S> {
    #[must_use]
    pub fn request(mut self, request: Arc<dyn BaseRequest>) -> Self {
        self.request = Some(request);
        self
    }

    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn base_file_url(mut self, url: impl Into<String>) -> Self {
        self.base_file_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn defaults(mut self, defaults: Defaults) -> Self {
        self.defaults = Some(defaults);
        self
    }

    #[must_use]
    pub fn arbitrary_callback_data(mut self, maxsize: usize) -> Self {
        self.arbitrary_callback_data = Some(maxsize);
        self
    }

    /// Sets the rate limiter for the application.
    ///
    /// When set, all API calls will be throttled through the provided limiter.
    /// Requires the `rate-limiter` feature.
    #[cfg(feature = "rate-limiter")]
    #[must_use]
    pub fn rate_limiter(mut self, rl: Arc<dyn DynRateLimiter>) -> Self {
        self.rate_limiter = Some(rl);
        self
    }

    /// Sets the rate-limiter placeholder (feature disabled).
    #[cfg(not(feature = "rate-limiter"))]
    #[must_use]
    pub fn rate_limiter(mut self, _rl: ()) -> Self {
        self.rate_limiter = Some(());
        self
    }

    #[must_use]
    pub fn context_types(mut self, ct: ContextTypes) -> Self {
        self.context_types = Some(ct);
        self
    }

    #[must_use]
    pub fn concurrent_updates(mut self, n: usize) -> Self {
        self.concurrent_updates = if n == 0 { 1 } else { n };
        self
    }

    #[must_use]
    pub fn post_init(mut self, hook: LifecycleHook) -> Self {
        self.post_init = Some(hook);
        self
    }

    #[must_use]
    pub fn post_stop(mut self, hook: LifecycleHook) -> Self {
        self.post_stop = Some(hook);
        self
    }

    #[must_use]
    pub fn post_shutdown(mut self, hook: LifecycleHook) -> Self {
        self.post_shutdown = Some(hook);
        self
    }

    /// Sets the persistence backend.
    ///
    /// Requires the `persistence` feature.
    #[cfg(feature = "persistence")]
    #[must_use]
    pub fn persistence(mut self, p: Box<dyn DynPersistence>) -> Self {
        self.persistence = Some(p);
        self
    }

    /// Sets the job queue.
    ///
    /// Requires the `job-queue` feature.
    #[cfg(feature = "job-queue")]
    #[must_use]
    pub fn job_queue(mut self, jq: Arc<JobQueue>) -> Self {
        self.job_queue = Some(jq);
        self
    }
}

impl ApplicationBuilder<HasToken> {
    #[must_use]
    pub fn build(self) -> Arc<Application> {
        let token = self.token.expect("HasToken state guarantees a token");

        let request: Arc<dyn BaseRequest> = self.request.unwrap_or_else(|| {
            Arc::new(
                rust_tg_bot_raw::request::reqwest_impl::ReqwestRequest::new()
                    .expect("Failed to create default ReqwestRequest"),
            )
        });

        // Wrap the request backend in a RateLimitedRequest if a limiter is set.
        #[cfg(feature = "rate-limiter")]
        let (effective_request, rate_limiter) = if let Some(ref rl) = self.rate_limiter {
            let wrapped: Arc<dyn BaseRequest> =
                Arc::new(RateLimitedRequest::new(request, rl.clone()));
            (wrapped, self.rate_limiter)
        } else {
            (request, None)
        };

        #[cfg(not(feature = "rate-limiter"))]
        let (effective_request, rate_limiter) = (request, self.rate_limiter);

        let bot_raw = Bot::new(&token, effective_request);

        let ext_bot = Arc::new(ExtBot::new(
            bot_raw,
            self.defaults,
            self.arbitrary_callback_data,
            rate_limiter,
        ));

        let context_types = self.context_types.unwrap_or_default();

        let update_processor = Arc::new(
            update_processor::simple_processor(self.concurrent_updates)
                .expect("concurrent_updates validated by builder"),
        );

        let mut config = ApplicationConfig::new(ext_bot, context_types, update_processor);
        config.post_init = self.post_init;
        config.post_stop = self.post_stop;
        config.post_shutdown = self.post_shutdown;
        #[cfg(feature = "persistence")]
        {
            config.persistence = self.persistence;
        }
        #[cfg(feature = "job-queue")]
        {
            config.job_queue = self.job_queue;
        }

        Application::new(config)
    }
}

impl<S> std::fmt::Debug for ApplicationBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationBuilder")
            .field("has_token", &self.token.is_some())
            .field("concurrent_updates", &self.concurrent_updates)
            .field("has_rate_limiter", &self.rate_limiter.is_some())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext_bot::test_support::mock_request;

    #[test]
    fn builder_typestate_enforces_token() {
        let app = ApplicationBuilder::new()
            .token("test_token")
            .request(mock_request())
            .build();
        assert_eq!(app.bot().token(), "test_token");
    }

    #[test]
    fn builder_with_defaults() {
        let defaults = Defaults::builder().parse_mode("HTML").build();
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .defaults(defaults)
            .build();
        assert_eq!(app.bot().defaults().unwrap().parse_mode(), Some("HTML"));
    }

    #[test]
    fn builder_concurrent_updates() {
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .concurrent_updates(8)
            .build();
        assert_eq!(app.concurrent_updates(), 8);
    }

    #[test]
    fn builder_zero_concurrent_updates_defaults_to_one() {
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .concurrent_updates(0)
            .build();
        assert_eq!(app.concurrent_updates(), 1);
    }

    #[test]
    fn builder_arbitrary_callback_data() {
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .arbitrary_callback_data(512)
            .build();
        assert!(app.bot().has_callback_data_cache());
    }

    #[test]
    fn builder_custom_context_types() {
        let ct = ContextTypes::default();
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .context_types(ct)
            .build();
        assert_eq!(app.bot().token(), "t");
    }

    #[test]
    fn builder_with_lifecycle_hooks() {
        let hook: LifecycleHook = Arc::new(|_app| Box::pin(async {}));
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .post_init(hook.clone())
            .post_stop(hook.clone())
            .post_shutdown(hook)
            .build();
        assert_eq!(app.bot().token(), "t");
    }

    #[cfg(feature = "job-queue")]
    #[test]
    fn builder_with_job_queue() {
        let jq = Arc::new(JobQueue::new());
        let app = ApplicationBuilder::new()
            .token("t")
            .request(mock_request())
            .job_queue(jq)
            .build();
        assert!(app.job_queue().is_some());
    }

    #[cfg(feature = "rate-limiter")]
    #[test]
    fn builder_with_rate_limiter() {
        use crate::rate_limiter::NoRateLimiter;

        let limiter: Arc<dyn DynRateLimiter> = Arc::new(NoRateLimiter);
        let app = ApplicationBuilder::new()
            .token("rl_app")
            .request(mock_request())
            .rate_limiter(limiter)
            .build();
        assert_eq!(app.bot().token(), "rl_app");
        assert!(app.bot().has_rate_limiter());
    }

    #[test]
    fn builder_debug() {
        let b = ApplicationBuilder::new();
        let s = format!("{b:?}");
        assert!(s.contains("ApplicationBuilder"));
        assert!(s.contains("has_token"));
    }

    #[test]
    fn default_builder_is_no_token() {
        let b = ApplicationBuilder::default();
        let _b2 = b.token("tok");
    }
}

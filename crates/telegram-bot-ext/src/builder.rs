//! Typestate builder for [`Application`](crate::application::Application).

use std::marker::PhantomData;
use std::sync::Arc;

use telegram_bot_raw::bot::Bot;
use telegram_bot_raw::request::base::BaseRequest;

use crate::application::{Application, ApplicationConfig, DynPersistence, LifecycleHook};
use crate::context_types::ContextTypes;
use crate::defaults::Defaults;
use crate::ext_bot::ExtBot;
use crate::job_queue::JobQueue;
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
    rate_limiter: Option<()>,
    context_types: Option<ContextTypes>,
    concurrent_updates: usize,
    post_init: Option<LifecycleHook>,
    post_stop: Option<LifecycleHook>,
    post_shutdown: Option<LifecycleHook>,
    persistence: Option<Box<dyn DynPersistence>>,
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
            persistence: None,
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
            persistence: self.persistence,
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

    #[must_use]
    pub fn rate_limiter(mut self, rl: ()) -> Self {
        self.rate_limiter = Some(rl);
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
    #[must_use]
    pub fn persistence(mut self, p: Box<dyn DynPersistence>) -> Self {
        self.persistence = Some(p);
        self
    }

    /// Sets the job queue.
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
                telegram_bot_raw::request::reqwest_impl::ReqwestRequest::new()
                    .expect("Failed to create default ReqwestRequest"),
            )
        });

        let bot_raw = Bot::new(&token, request);

        let ext_bot = Arc::new(ExtBot::new(
            bot_raw,
            self.defaults,
            self.arbitrary_callback_data,
            self.rate_limiter,
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
        config.persistence = self.persistence;
        config.job_queue = self.job_queue;

        Application::new(config)
    }
}

impl<S> std::fmt::Debug for ApplicationBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationBuilder")
            .field("has_token", &self.token.is_some())
            .field("concurrent_updates", &self.concurrent_updates)
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

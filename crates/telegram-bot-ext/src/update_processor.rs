//! Semaphore-based concurrent update processing.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_baseupdateprocessor.py`.
//!
//! Provides [`BaseUpdateProcessor`] (the async trait) and [`SimpleUpdateProcessor`] (the default
//! implementation that immediately awaits each coroutine under a semaphore).

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::sync::Semaphore;

use telegram_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that may occur during update processing.
#[derive(Debug, thiserror::Error)]
pub enum UpdateProcessorError {
    /// `max_concurrent_updates` was not a positive integer.
    #[error("`max_concurrent_updates` must be a positive integer")]
    InvalidConcurrency,

    /// An inner handler returned an error.
    #[error("Handler error: {0}")]
    Handler(Box<dyn std::error::Error + Send + Sync>),
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// An abstract base for update processors.
///
/// Implementations control *how* update coroutines are driven (e.g. immediately awaited,
/// batched, prioritised, etc.).
///
/// The [`process_update`](BaseUpdateProcessor::process_update) method is *final* -- it
/// acquires the internal semaphore and then delegates to
/// [`do_process_update`](BaseUpdateProcessor::do_process_update).
#[async_trait::async_trait]
pub trait UpdateProcessor: Send + Sync {
    /// Custom implementation of how to process an update.  Must be implemented by the
    /// concrete type.
    ///
    /// **Warning**: This method is called by
    /// [`process_update`](BaseUpdateProcessor::process_update).  It should *not* be called
    /// manually.
    async fn do_process_update(
        &self,
        update: Update,
        coroutine: Pin<Box<dyn Future<Output = ()> + Send>>,
    );

    /// Called once before the processor starts handling updates.
    async fn initialize(&self) {}

    /// Called once when the processor is shutting down.
    async fn shutdown(&self) {}
}

// ---------------------------------------------------------------------------
// BaseUpdateProcessor -- semaphore wrapper
// ---------------------------------------------------------------------------

/// Wraps any [`UpdateProcessor`] with a semaphore to bound concurrency.
pub struct BaseUpdateProcessor {
    inner: Box<dyn UpdateProcessor>,
    semaphore: Arc<Semaphore>,
    max_concurrent_updates: usize,
    /// Tracks how many permits are currently held so we can report
    /// `current_concurrent_updates`.
    active: AtomicUsize,
}

impl std::fmt::Debug for BaseUpdateProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseUpdateProcessor")
            .field("max_concurrent_updates", &self.max_concurrent_updates)
            .field("active", &self.active.load(Ordering::Relaxed))
            .finish()
    }
}

impl BaseUpdateProcessor {
    /// Creates a new `BaseUpdateProcessor`.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateProcessorError::InvalidConcurrency`] if
    /// `max_concurrent_updates` is zero.
    pub fn new(
        inner: Box<dyn UpdateProcessor>,
        max_concurrent_updates: usize,
    ) -> Result<Self, UpdateProcessorError> {
        if max_concurrent_updates == 0 {
            return Err(UpdateProcessorError::InvalidConcurrency);
        }
        Ok(Self {
            inner,
            semaphore: Arc::new(Semaphore::new(max_concurrent_updates)),
            max_concurrent_updates,
            active: AtomicUsize::new(0),
        })
    }

    /// The maximum number of updates that can be processed concurrently.
    #[must_use]
    pub fn max_concurrent_updates(&self) -> usize {
        self.max_concurrent_updates
    }

    /// A snapshot of the number of updates currently being processed.
    #[must_use]
    pub fn current_concurrent_updates(&self) -> usize {
        self.active.load(Ordering::Relaxed)
    }

    /// Acquires the semaphore and then delegates to [`UpdateProcessor::do_process_update`].
    pub async fn process_update(
        &self,
        update: Update,
        coroutine: Pin<Box<dyn Future<Output = ()> + Send>>,
    ) {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .expect("semaphore should not be closed");
        self.active.fetch_add(1, Ordering::Relaxed);
        self.inner.do_process_update(update, coroutine).await;
        self.active.fetch_sub(1, Ordering::Relaxed);
    }

    /// Delegates to the inner processor's `initialize`.
    pub async fn initialize(&self) {
        self.inner.initialize().await;
    }

    /// Delegates to the inner processor's `shutdown`.
    pub async fn shutdown(&self) {
        self.inner.shutdown().await;
    }
}

// ---------------------------------------------------------------------------
// SimpleUpdateProcessor
// ---------------------------------------------------------------------------

/// Default [`UpdateProcessor`] that immediately awaits the coroutine.
///
/// This is used when `ApplicationBuilder.concurrent_updates` is set to an integer -- the
/// semaphore in [`BaseUpdateProcessor`] provides the actual bounding.
#[derive(Debug, Default)]
pub struct SimpleUpdateProcessor;

#[async_trait::async_trait]
impl UpdateProcessor for SimpleUpdateProcessor {
    async fn do_process_update(
        &self,
        _update: Update,
        coroutine: Pin<Box<dyn Future<Output = ()> + Send>>,
    ) {
        coroutine.await;
    }
}

/// Convenience constructor that builds a [`BaseUpdateProcessor`] wrapping a
/// [`SimpleUpdateProcessor`] with the given concurrency limit.
///
/// # Errors
///
/// Returns [`UpdateProcessorError::InvalidConcurrency`] if `max_concurrent_updates` is zero.
pub fn simple_processor(
    max_concurrent_updates: usize,
) -> Result<BaseUpdateProcessor, UpdateProcessorError> {
    BaseUpdateProcessor::new(Box::new(SimpleUpdateProcessor), max_concurrent_updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_update() -> Update {
        serde_json::from_value(serde_json::json!({"update_id": 0})).unwrap()
    }

    #[tokio::test]
    async fn simple_processor_runs_coroutine() {
        let proc = simple_processor(1).unwrap();
        proc.initialize().await;

        let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag2 = flag.clone();

        let fut: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            flag2.store(true, Ordering::Relaxed);
        });

        proc.process_update(dummy_update(), fut).await;
        assert!(flag.load(Ordering::Relaxed));

        proc.shutdown().await;
    }

    #[test]
    fn zero_concurrency_rejected() {
        assert!(simple_processor(0).is_err());
    }

    #[tokio::test]
    async fn concurrent_updates_tracking() {
        let proc = simple_processor(4).unwrap();
        assert_eq!(proc.max_concurrent_updates(), 4);
        assert_eq!(proc.current_concurrent_updates(), 0);
    }

    #[tokio::test]
    async fn concurrent_processing_bounded() {
        let proc = Arc::new(simple_processor(2).unwrap());
        let counter = Arc::new(AtomicUsize::new(0));
        let max_seen = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();

        for _ in 0..10 {
            let p = proc.clone();
            let c = counter.clone();
            let m = max_seen.clone();

            handles.push(tokio::spawn(async move {
                let cc = c.clone();
                let mm = m.clone();
                let fut: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
                    let current = cc.fetch_add(1, Ordering::SeqCst) + 1;
                    // Record the maximum concurrent count observed.
                    mm.fetch_max(current, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                    cc.fetch_sub(1, Ordering::SeqCst);
                });
                p.process_update(dummy_update(), fut).await;
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        // The semaphore should have bounded concurrency to 2.
        assert!(max_seen.load(Ordering::SeqCst) <= 2);
    }
}

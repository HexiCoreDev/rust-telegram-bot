//! A retry loop for network-oriented operations against the Telegram API.
//!
//! Port of `telegram.ext._utils.networkloop.network_retry_loop`.
//! This is library-internal and not part of the public API stability guarantee.

use std::future::Future;
use std::time::Duration;

use tokio::sync::watch;
use tracing::{debug, error};

use telegram_bot_raw::error::TelegramError;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Parameters for [`network_retry_loop`].
pub struct NetworkLoopConfig<'a, A, E> {
    /// The async action to attempt on each iteration.
    pub action_cb: A,
    /// Optional error callback invoked when a `TelegramError` is caught.
    pub on_err_cb: Option<E>,
    /// Human-readable label used in log messages.
    pub description: &'a str,
    /// Base interval between attempts (seconds).
    pub interval: f64,
    /// A watch receiver whose value, when `true`, signals the loop to stop.
    /// Pass `None` if the loop should only be controlled by `is_running` and
    /// `max_retries`.
    pub stop_rx: Option<watch::Receiver<bool>>,
    /// Predicate checked at the top of every iteration. Returning `false`
    /// exits the loop.
    pub is_running: Option<Box<dyn Fn() -> bool + Send + Sync + 'a>>,
    /// Maximum retry count.
    /// - negative: retry indefinitely.
    /// - 0: no retries (single attempt).
    /// - positive: up to N retries.
    pub max_retries: i32,
    /// If `true`, the action is repeated after a successful call.
    pub repeat_on_success: bool,
}

// ---------------------------------------------------------------------------
// Loop implementation
// ---------------------------------------------------------------------------

/// Run `action_cb` in a loop, retrying on `TelegramError` according to the
/// back-off / retry policy described by `config`.
///
/// # Errors
///
/// Returns the last `TelegramError` if retries are exhausted, or propagates
/// an `InvalidToken` error immediately.
pub async fn network_retry_loop<'a, A, AF, E>(
    config: NetworkLoopConfig<'a, A, E>,
) -> Result<(), TelegramError>
where
    A: Fn() -> AF,
    AF: Future<Output = Result<(), TelegramError>>,
    E: Fn(&TelegramError),
{
    let NetworkLoopConfig {
        action_cb,
        on_err_cb,
        description,
        interval,
        mut stop_rx,
        is_running,
        max_retries,
        repeat_on_success,
    } = config;

    let log_prefix = format!("Network Retry Loop ({description}):");
    let effective_is_running = is_running.unwrap_or_else(|| Box::new(|| true));

    debug!("{log_prefix} Starting");

    let mut cur_interval = interval;
    let mut retries: i32 = 0;

    while effective_is_running() {
        // Execute the action, racing against the stop signal if one exists.
        let action_result = match stop_rx.as_mut() {
            Some(rx) => {
                tokio::select! {
                    biased;
                    _ = wait_for_stop(rx) => {
                        debug!("{log_prefix} Cancelled via stop signal");
                        return Ok(());
                    }
                    res = action_cb() => res,
                }
            }
            None => action_cb().await,
        };

        match action_result {
            Ok(()) => {
                if !repeat_on_success {
                    debug!("{log_prefix} Action succeeded. Stopping loop.");
                    return Ok(());
                }
                cur_interval = interval;
            }
            Err(TelegramError::RetryAfter { retry_after }) => {
                let slack = Duration::from_millis(500);
                cur_interval = (retry_after + slack).as_secs_f64();
                if check_max_retries(retries, max_retries, &log_prefix) {
                    return Err(TelegramError::RetryAfter { retry_after });
                }
            }
            Err(TelegramError::TimedOut(_)) => {
                cur_interval = 0.0;
                if check_max_retries(retries, max_retries, &log_prefix) {
                    return Err(TelegramError::TimedOut("timed out".into()));
                }
            }
            Err(TelegramError::InvalidToken(msg)) => {
                error!("{log_prefix} Invalid token. Aborting retry loop.");
                return Err(TelegramError::InvalidToken(msg));
            }
            Err(ref e) => {
                if let Some(ref cb) = on_err_cb {
                    cb(e);
                }
                if check_max_retries(retries, max_retries, &log_prefix) {
                    // Move out of the ref to return ownership.
                    return Err(action_result.unwrap_err());
                }
                // Exponential back-off up to 30 seconds.
                cur_interval = if cur_interval == 0.0 {
                    1.0
                } else {
                    (1.5 * cur_interval).min(30.0)
                };
            }
        }

        retries += 1;

        if cur_interval > 0.0 {
            tokio::time::sleep(Duration::from_secs_f64(cur_interval)).await;
        }
    }

    Ok(())
}

/// Wait until the watch channel yields `true`.
async fn wait_for_stop(rx: &mut watch::Receiver<bool>) {
    while !*rx.borrow_and_update() {
        if rx.changed().await.is_err() {
            // Sender dropped -- treat as stop.
            return;
        }
    }
}

/// Returns `true` if we should abort (max retries reached).
fn check_max_retries(current: i32, max: i32, prefix: &str) -> bool {
    if max < 0 || current < max {
        debug!(
            "{prefix} Failed run {current} of {max}. Retrying.",
        );
        false
    } else {
        error!(
            "{prefix} Failed run {current} of {max}. Aborting.",
        );
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn succeeds_on_first_try() {
        let result = network_retry_loop(NetworkLoopConfig {
            action_cb: || async { Ok(()) },
            on_err_cb: None::<fn(&TelegramError)>,
            description: "test",
            interval: 0.0,
            stop_rx: None,
            is_running: None,
            max_retries: 0,
            repeat_on_success: false,
        })
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn retries_and_succeeds() {
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();
        let result = network_retry_loop(NetworkLoopConfig {
            action_cb: move || {
                let c = c.clone();
                async move {
                    let n = c.fetch_add(1, Ordering::SeqCst);
                    if n < 2 {
                        Err(TelegramError::Network("fail".into()))
                    } else {
                        Ok(())
                    }
                }
            },
            on_err_cb: None::<fn(&TelegramError)>,
            description: "retry-test",
            interval: 0.0,
            stop_rx: None,
            is_running: None,
            max_retries: -1, // indefinite
            repeat_on_success: false,
        })
        .await;
        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn aborts_after_max_retries() {
        let result = network_retry_loop(NetworkLoopConfig {
            action_cb: || async {
                Err::<(), _>(TelegramError::Network("always fail".into()))
            },
            on_err_cb: None::<fn(&TelegramError)>,
            description: "abort-test",
            interval: 0.0,
            stop_rx: None,
            is_running: None,
            max_retries: 2,
            repeat_on_success: false,
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn invalid_token_aborts_immediately() {
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();
        let result = network_retry_loop(NetworkLoopConfig {
            action_cb: move || {
                let c = c.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Err::<(), _>(TelegramError::InvalidToken("bad".into()))
                }
            },
            on_err_cb: None::<fn(&TelegramError)>,
            description: "token-test",
            interval: 0.0,
            stop_rx: None,
            is_running: None,
            max_retries: -1,
            repeat_on_success: false,
        })
        .await;
        assert!(result.is_err());
        // Should abort on the first attempt, no retries.
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn stop_signal_cancels_loop() {
        let (tx, rx) = watch::channel(false);
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();

        // Spawn the loop; it will repeat on success indefinitely.
        let handle = tokio::spawn(async move {
            network_retry_loop(NetworkLoopConfig {
                action_cb: move || {
                    let c = c.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                },
                on_err_cb: None::<fn(&TelegramError)>,
                description: "stop-test",
                interval: 0.01,
                stop_rx: Some(rx),
                is_running: None,
                max_retries: -1,
                repeat_on_success: true,
            })
            .await
        });

        // Let it run a few iterations then signal stop.
        tokio::time::sleep(Duration::from_millis(80)).await;
        tx.send(true).unwrap();
        let result = handle.await.unwrap();
        assert!(result.is_ok());
        // It should have run at least once.
        assert!(counter.load(Ordering::SeqCst) >= 1);
    }
}

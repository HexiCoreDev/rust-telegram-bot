//! Scheduled job execution for Telegram bots.
//!
//! Port of `telegram.ext._jobqueue`. Uses `tokio` timers directly instead
//! of APScheduler, with `tokio::sync::watch<bool>` for cancellation.
//!
//! # Example
//!
//! ```rust,ignore
//! let jq = JobQueue::new();
//! jq.start().await;
//!
//! // Builder API -- no trailing None arguments:
//! let job = jq.once(callback, Duration::from_secs(5))
//!     .name("greeting")
//!     .chat_id(12345)
//!     .start()
//!     .await;
//! ```

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::{Datelike, NaiveTime, Timelike, Utc, Weekday};
use tokio::sync::{watch, Mutex, RwLock};
use tracing::{debug, error};

// ---------------------------------------------------------------------------
// Job ID
// ---------------------------------------------------------------------------

static NEXT_JOB_ID: AtomicU64 = AtomicU64::new(1);

fn next_job_id() -> u64 {
    NEXT_JOB_ID.fetch_add(1, Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Hooks -- called by the Application after each job execution
// ---------------------------------------------------------------------------

/// Hook invoked after every job callback completes (success or failure).
///
/// The Application wires this to call `update_persistence()`, mirroring
/// Python's `application._mark_for_persistence_update(job=self)`.
pub type PostJobHook = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Hook invoked when a job callback returns an error.
///
/// The Application wires this to call `process_error(None, error)`, mirroring
/// Python's `application.process_error(None, exc, job=self)`.
pub type JobErrorHook = Arc<
    dyn Fn(Box<dyn std::error::Error + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

// ---------------------------------------------------------------------------
// JobCallback
// ---------------------------------------------------------------------------

/// A job callback is an async function that receives an opaque context value.
///
/// Returning `Err` routes the error through the Application's error handlers,
/// matching Python-Telegram-Bot's exception handling in `Job._run()`.
pub type JobCallbackFn = Arc<
    dyn Fn(
            JobContext,
        ) -> Pin<
            Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>,
        > + Send
        + Sync,
>;

/// Opaque context handed to job callbacks. In a full application this would
/// carry a reference to the `Application`; here it carries the minimal info
/// that the callback needs.
#[derive(Debug, Clone)]
pub struct JobContext {
    pub job_name: String,
    pub chat_id: Option<i64>,
    pub user_id: Option<i64>,
    pub data: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Job
// ---------------------------------------------------------------------------

/// A handle to a scheduled job. Can be used to check status or cancel.
#[derive(Clone)]
pub struct Job {
    pub id: u64,
    pub name: String,
    pub chat_id: Option<i64>,
    pub user_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    callback: JobCallbackFn,
    /// Sending `true` cancels the job's timer loop.
    cancel_tx: watch::Sender<bool>,
    /// `true` once the job has been removed / finished.
    removed: Arc<std::sync::atomic::AtomicBool>,
    /// `true` while the job is enabled (can be paused/resumed).
    enabled: Arc<std::sync::atomic::AtomicBool>,
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Job")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("chat_id", &self.chat_id)
            .field("user_id", &self.user_id)
            .field("removed", &self.is_removed())
            .field("enabled", &self.is_enabled())
            .finish()
    }
}

impl Job {
    fn new(
        name: String,
        callback: JobCallbackFn,
        chat_id: Option<i64>,
        user_id: Option<i64>,
        data: Option<serde_json::Value>,
    ) -> Self {
        let (cancel_tx, _) = watch::channel(false);
        Self {
            id: next_job_id(),
            name,
            chat_id,
            user_id,
            data,
            callback,
            cancel_tx,
            removed: Arc::new(false.into()),
            enabled: Arc::new(true.into()),
        }
    }

    /// Run the callback immediately (bypass the schedule).
    ///
    /// After the callback completes (success or failure), the post-job hooks
    /// are invoked to match Python-Telegram-Bot's `Job._run()` behavior:
    ///
    /// - On error: `on_job_error` is called (routes to `Application.process_error`)
    /// - Always: `on_job_complete` is called (triggers `Application.update_persistence`)
    pub async fn run(
        &self,
        on_job_complete: &Option<PostJobHook>,
        on_job_error: &Option<JobErrorHook>,
    ) {
        if !self.is_enabled() {
            debug!("Job {} is disabled, skipping run", self.name);
            return;
        }
        let ctx = JobContext {
            job_name: self.name.clone(),
            chat_id: self.chat_id,
            user_id: self.user_id,
            data: self.data.clone(),
        };

        let result = (self.callback)(ctx).await;

        // Mirror PTB: on error, route through application.process_error
        if let Err(err) = result {
            if let Some(ref hook) = on_job_error {
                hook(err).await;
            } else {
                error!(
                    "Job '{}' raised an error with no error hook set: {}",
                    self.name, err
                );
            }
        }

        // Mirror PTB: always call application._mark_for_persistence_update
        if let Some(ref hook) = on_job_complete {
            hook().await;
        }
    }

    /// Cancel this job. It will not fire again.
    pub fn schedule_removal(&self) {
        self.removed.store(true, Ordering::Relaxed);
        let _ = self.cancel_tx.send(true);
    }

    pub fn is_removed(&self) -> bool {
        self.removed.load(Ordering::Relaxed)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    fn cancel_rx(&self) -> watch::Receiver<bool> {
        self.cancel_tx.subscribe()
    }
}

// ---------------------------------------------------------------------------
// JobQueue
// ---------------------------------------------------------------------------

/// Manages scheduling and cancellation of [`Job`]s using tokio timers.
pub struct JobQueue {
    jobs: RwLock<HashMap<u64, Job>>,
    running: std::sync::atomic::AtomicBool,
    /// Master shutdown signal: setting this to `true` cancels all spawned job
    /// loops.
    shutdown_tx: watch::Sender<bool>,
    #[allow(dead_code)]
    shutdown_rx: Mutex<watch::Receiver<bool>>,

    /// Hook called after every job execution (success or failure) to flush
    /// persistence. Set by the Application during initialization.
    on_job_complete: RwLock<Option<PostJobHook>>,

    /// Hook called when a job callback returns an error. Set by the Application
    /// during initialization to route errors through `process_error`.
    on_job_error: RwLock<Option<JobErrorHook>>,
}

impl fmt::Debug for JobQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JobQueue")
            .field("running", &self.running.load(Ordering::Relaxed))
            .finish()
    }
}

struct RepeatingJobSpec {
    name: String,
    interval: Duration,
    first: Option<Duration>,
    last: Option<Duration>,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

struct DailyJobSpec {
    name: String,
    time: NaiveTime,
    days: Vec<u8>,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

struct MonthlyJobSpec {
    name: String,
    time: NaiveTime,
    day: i32,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl JobQueue {
    pub fn new() -> Self {
        let (tx, rx) = watch::channel(false);
        Self {
            jobs: RwLock::new(HashMap::new()),
            running: false.into(),
            shutdown_tx: tx,
            shutdown_rx: Mutex::new(rx),
            on_job_complete: RwLock::new(None),
            on_job_error: RwLock::new(None),
        }
    }

    /// Set the post-job-completion hook. Called after every job callback
    /// finishes (regardless of success/failure), mirroring PTB's
    /// `application._mark_for_persistence_update(job=self)`.
    pub async fn set_on_job_complete(&self, hook: PostJobHook) {
        *self.on_job_complete.write().await = Some(hook);
    }

    /// Set the job-error hook. Called when a job callback returns an error,
    /// mirroring PTB's `application.process_error(None, exc, job=self)`.
    pub async fn set_on_job_error(&self, hook: JobErrorHook) {
        *self.on_job_error.write().await = Some(hook);
    }

    /// Mark the queue as running. In a full application this would wire up
    /// the APScheduler equivalent; here it simply flips a flag.
    pub async fn start(&self) {
        self.running.store(true, Ordering::Relaxed);
        debug!("JobQueue started");
    }

    /// Shut down the queue and cancel all pending jobs.
    pub async fn stop(&self) {
        if !self.running.load(Ordering::Relaxed) {
            return;
        }
        debug!("JobQueue stopping");
        self.running.store(false, Ordering::Relaxed);
        let _ = self.shutdown_tx.send(true);

        // Mark all jobs as removed.
        let jobs = self.jobs.read().await;
        for job in jobs.values() {
            job.schedule_removal();
        }
        debug!("JobQueue stopped");
    }

    /// Return all currently scheduled (non-removed) jobs.
    pub async fn jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        jobs.values().filter(|j| !j.is_removed()).cloned().collect()
    }

    /// Return all scheduled jobs whose name matches a regex pattern.
    pub async fn jobs_by_pattern(&self, pattern: &str) -> Vec<Job> {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let jobs = self.jobs.read().await;
        jobs.values()
            .filter(|j| !j.is_removed() && re.is_match(&j.name))
            .cloned()
            .collect()
    }

    /// Return all scheduled jobs with the given exact name.
    pub async fn get_jobs_by_name(&self, name: &str) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        jobs.values()
            .filter(|j| !j.is_removed() && j.name == name)
            .cloned()
            .collect()
    }

    // -----------------------------------------------------------------------
    // Internal: snapshot hooks for use inside spawned tasks
    // -----------------------------------------------------------------------

    /// Take a snapshot of the current hooks so they can be moved into a
    /// spawned task without holding the `RwLock` across `.await` points.
    async fn snapshot_hooks(&self) -> (Option<PostJobHook>, Option<JobErrorHook>) {
        let complete = self.on_job_complete.read().await.clone();
        let error = self.on_job_error.read().await.clone();
        (complete, error)
    }

    // -----------------------------------------------------------------------
    // Builder entry points (public API)
    // -----------------------------------------------------------------------

    /// Returns a builder for scheduling a one-shot job.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let job = jq.once(callback, Duration::from_secs(5))
    ///     .name("greeting")
    ///     .chat_id(12345)
    ///     .start()
    ///     .await;
    /// ```
    pub fn once(&self, callback: JobCallbackFn, delay: Duration) -> RunOnceBuilder<'_> {
        RunOnceBuilder {
            queue: self,
            name: "once_job".to_owned(),
            delay,
            callback,
            chat_id: None,
            user_id: None,
            data: None,
        }
    }

    /// Returns a builder for scheduling a repeating job.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let job = jq.repeating(callback, Duration::from_secs(60))
    ///     .name("heartbeat")
    ///     .first(Duration::from_secs(5))
    ///     .start()
    ///     .await;
    /// ```
    pub fn repeating(
        &self,
        callback: JobCallbackFn,
        interval: Duration,
    ) -> RunRepeatingBuilder<'_> {
        RunRepeatingBuilder {
            queue: self,
            name: "repeating_job".to_owned(),
            interval,
            first: None,
            last: None,
            callback,
            chat_id: None,
            user_id: None,
            data: None,
        }
    }

    /// Returns a builder for scheduling a daily job.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let job = jq.daily(callback, NaiveTime::from_hms_opt(9, 0, 0).unwrap(), &[1, 2, 3, 4, 5])
    ///     .name("morning_report")
    ///     .chat_id(12345)
    ///     .start()
    ///     .await;
    /// ```
    pub fn daily<'a>(
        &'a self,
        callback: JobCallbackFn,
        time: NaiveTime,
        days: &[u8],
    ) -> RunDailyBuilder<'a> {
        RunDailyBuilder {
            queue: self,
            name: "daily_job".to_owned(),
            time,
            days: days.to_vec(),
            callback,
            chat_id: None,
            user_id: None,
            data: None,
        }
    }

    /// Returns a builder for scheduling a monthly job.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let job = jq.monthly(callback, NaiveTime::from_hms_opt(12, 0, 0).unwrap(), 1)
    ///     .name("monthly_report")
    ///     .start()
    ///     .await;
    /// ```
    pub fn monthly(
        &self,
        callback: JobCallbackFn,
        time: NaiveTime,
        day: i32,
    ) -> RunMonthlyBuilder<'_> {
        RunMonthlyBuilder {
            queue: self,
            name: "monthly_job".to_owned(),
            time,
            day,
            callback,
            chat_id: None,
            user_id: None,
            data: None,
        }
    }

    /// Returns a builder for scheduling a custom one-shot job via `Arc<JobQueue>`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let job = jq.custom(callback, Duration::from_secs(120))
    ///     .name("custom_task")
    ///     .data(serde_json::json!({"key": "val"}))
    ///     .start();
    /// ```
    pub fn custom(
        self: &Arc<Self>,
        callback: JobCallbackFn,
        trigger: Duration,
    ) -> RunCustomBuilder {
        RunCustomBuilder {
            queue: Arc::clone(self),
            name: "custom_job".to_owned(),
            trigger,
            callback,
            chat_id: None,
            user_id: None,
            data: None,
        }
    }

    // -----------------------------------------------------------------------
    // Raw scheduling methods (crate-internal)
    // -----------------------------------------------------------------------

    /// Schedule a job that runs once after `delay`.
    pub(crate) async fn run_once(
        &self,
        name: impl Into<String>,
        delay: Duration,
        callback: JobCallbackFn,
        chat_id: Option<i64>,
        user_id: Option<i64>,
        data: Option<serde_json::Value>,
    ) -> Job {
        let job = Job::new(name.into(), callback, chat_id, user_id, data);
        let handle = job.clone();
        self.register(job.clone()).await;

        let mut cancel_rx = job.cancel_rx();
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let (hook_complete, hook_error) = self.snapshot_hooks().await;

        tokio::spawn(async move {
            tokio::select! {
                biased;
                _ = wait_cancel(&mut cancel_rx) => {}
                _ = wait_cancel(&mut shutdown_rx) => {}
                _ = tokio::time::sleep(delay) => {
                    job.run(&hook_complete, &hook_error).await;
                    job.schedule_removal();
                }
            }
        });

        handle
    }

    /// Schedule a repeating job with a fixed interval.
    ///
    /// - `first`: optional initial delay before the first run.
    /// - `last`: optional deadline after which the job stops.
    async fn run_repeating(&self, spec: RepeatingJobSpec) -> Job {
        let RepeatingJobSpec {
            name,
            interval,
            first,
            last,
            callback,
            chat_id,
            user_id,
            data,
        } = spec;
        let job = Job::new(name, callback, chat_id, user_id, data);
        let handle = job.clone();
        self.register(job.clone()).await;

        let mut cancel_rx = job.cancel_rx();
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let start = tokio::time::Instant::now();
        let (hook_complete, hook_error) = self.snapshot_hooks().await;

        tokio::spawn(async move {
            // Initial delay.
            if let Some(first_delay) = first {
                tokio::select! {
                    biased;
                    _ = wait_cancel(&mut cancel_rx) => return,
                    _ = wait_cancel(&mut shutdown_rx) => return,
                    _ = tokio::time::sleep(first_delay) => {}
                }
            }

            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    biased;
                    _ = wait_cancel(&mut cancel_rx) => return,
                    _ = wait_cancel(&mut shutdown_rx) => return,
                    _ = ticker.tick() => {
                        if let Some(deadline) = last {
                            if start.elapsed() >= deadline {
                                job.schedule_removal();
                                return;
                            }
                        }
                        job.run(&hook_complete, &hook_error).await;
                    }
                }
            }
        });

        handle
    }

    /// Schedule a job that runs daily at a specific time on the given days
    /// of the week.
    ///
    /// `days`: 0 = Sunday .. 6 = Saturday (matching PTB convention).
    async fn run_daily(&self, spec: DailyJobSpec) -> Job {
        let DailyJobSpec {
            name,
            time,
            days,
            callback,
            chat_id,
            user_id,
            data,
        } = spec;
        let job = Job::new(name, callback, chat_id, user_id, data);
        let handle = job.clone();
        self.register(job.clone()).await;

        let mut cancel_rx = job.cancel_rx();
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let allowed_days = days;
        let (hook_complete, hook_error) = self.snapshot_hooks().await;

        tokio::spawn(async move {
            loop {
                let delay = duration_until_next_daily(&time, &allowed_days);
                tokio::select! {
                    biased;
                    _ = wait_cancel(&mut cancel_rx) => return,
                    _ = wait_cancel(&mut shutdown_rx) => return,
                    _ = tokio::time::sleep(delay) => {
                        job.run(&hook_complete, &hook_error).await;
                    }
                }
            }
        });

        handle
    }

    /// Schedule a job that runs monthly on a specific day at a specific time.
    ///
    /// If `day == -1`, runs on the last day of each month.
    /// If a month has fewer days than `day`, the job is skipped that month.
    async fn run_monthly(&self, spec: MonthlyJobSpec) -> Job {
        let MonthlyJobSpec {
            name,
            time,
            day,
            callback,
            chat_id,
            user_id,
            data,
        } = spec;
        let job = Job::new(name, callback, chat_id, user_id, data);
        let handle = job.clone();
        self.register(job.clone()).await;

        let mut cancel_rx = job.cancel_rx();
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let (hook_complete, hook_error) = self.snapshot_hooks().await;

        tokio::spawn(async move {
            loop {
                let delay = duration_until_next_monthly(&time, day);
                tokio::select! {
                    biased;
                    _ = wait_cancel(&mut cancel_rx) => return,
                    _ = wait_cancel(&mut shutdown_rx) => return,
                    _ = tokio::time::sleep(delay) => {
                        job.run(&hook_complete, &hook_error).await;
                    }
                }
            }
        });

        handle
    }

    /// Schedule a one-shot job that fires after an arbitrary `trigger` duration.
    ///
    /// Unlike [`JobQueue::run_once`] this method takes ownership of the queue via
    /// `Arc<JobQueue>` so that registration can happen inside the spawned future.
    pub(crate) fn run_custom(
        self: &Arc<Self>,
        callback: JobCallbackFn,
        trigger: Duration,
        name: Option<String>,
        data: Option<serde_json::Value>,
        chat_id: Option<i64>,
        user_id: Option<i64>,
    ) -> Job {
        let job_name = name.unwrap_or_else(|| "custom_job".to_owned());
        let job = Job::new(job_name, callback, chat_id, user_id, data);
        let handle = job.clone();

        let mut cancel_rx = job.cancel_rx();
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let queue = Arc::clone(self);

        tokio::spawn(async move {
            queue.register(job.clone()).await;
            let (hook_complete, hook_error) = queue.snapshot_hooks().await;
            tokio::select! {
                biased;
                _ = wait_cancel(&mut cancel_rx) => {}
                _ = wait_cancel(&mut shutdown_rx) => {}
                _ = tokio::time::sleep(trigger) => {
                    job.run(&hook_complete, &hook_error).await;
                    job.schedule_removal();
                }
            }
        });

        handle
    }

    // -----------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------

    async fn register(&self, job: Job) {
        self.jobs.write().await.insert(job.id, job);
    }
}

// ---------------------------------------------------------------------------
// Job scheduling builders
// ---------------------------------------------------------------------------

/// Builder for scheduling a one-shot job via [`JobQueue::once`].
pub struct RunOnceBuilder<'a> {
    queue: &'a JobQueue,
    name: String,
    delay: Duration,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl<'a> fmt::Debug for RunOnceBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunOnceBuilder")
            .field("name", &self.name)
            .field("delay", &self.delay)
            .finish()
    }
}

impl<'a> RunOnceBuilder<'a> {
    /// Sets the job name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the chat ID forwarded to the callback context.
    #[must_use]
    pub fn chat_id(mut self, id: i64) -> Self {
        self.chat_id = Some(id);
        self
    }

    /// Sets the user ID forwarded to the callback context.
    #[must_use]
    pub fn user_id(mut self, id: i64) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets arbitrary JSON data forwarded to the callback context.
    #[must_use]
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Schedules the job and returns a handle.
    pub async fn start(self) -> Job {
        self.queue
            .run_once(
                self.name,
                self.delay,
                self.callback,
                self.chat_id,
                self.user_id,
                self.data,
            )
            .await
    }
}

/// Builder for scheduling a repeating job via [`JobQueue::repeating`].
pub struct RunRepeatingBuilder<'a> {
    queue: &'a JobQueue,
    name: String,
    interval: Duration,
    first: Option<Duration>,
    last: Option<Duration>,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl<'a> fmt::Debug for RunRepeatingBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunRepeatingBuilder")
            .field("name", &self.name)
            .field("interval", &self.interval)
            .finish()
    }
}

impl<'a> RunRepeatingBuilder<'a> {
    /// Sets the job name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets an initial delay before the first run.
    #[must_use]
    pub fn first(mut self, delay: Duration) -> Self {
        self.first = Some(delay);
        self
    }

    /// Sets a deadline after which the job stops repeating.
    #[must_use]
    pub fn last(mut self, deadline: Duration) -> Self {
        self.last = Some(deadline);
        self
    }

    /// Sets the chat ID forwarded to the callback context.
    #[must_use]
    pub fn chat_id(mut self, id: i64) -> Self {
        self.chat_id = Some(id);
        self
    }

    /// Sets the user ID forwarded to the callback context.
    #[must_use]
    pub fn user_id(mut self, id: i64) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets arbitrary JSON data forwarded to the callback context.
    #[must_use]
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Schedules the job and returns a handle.
    pub async fn start(self) -> Job {
        self.queue
            .run_repeating(RepeatingJobSpec {
                name: self.name,
                interval: self.interval,
                first: self.first,
                last: self.last,
                callback: self.callback,
                chat_id: self.chat_id,
                user_id: self.user_id,
                data: self.data,
            })
            .await
    }
}

/// Builder for scheduling a daily job via [`JobQueue::daily`].
pub struct RunDailyBuilder<'a> {
    queue: &'a JobQueue,
    name: String,
    time: NaiveTime,
    days: Vec<u8>,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl<'a> fmt::Debug for RunDailyBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunDailyBuilder")
            .field("name", &self.name)
            .field("time", &self.time)
            .finish()
    }
}

impl<'a> RunDailyBuilder<'a> {
    /// Sets the job name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the chat ID forwarded to the callback context.
    #[must_use]
    pub fn chat_id(mut self, id: i64) -> Self {
        self.chat_id = Some(id);
        self
    }

    /// Sets the user ID forwarded to the callback context.
    #[must_use]
    pub fn user_id(mut self, id: i64) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets arbitrary JSON data forwarded to the callback context.
    #[must_use]
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Schedules the job and returns a handle.
    pub async fn start(self) -> Job {
        self.queue
            .run_daily(DailyJobSpec {
                name: self.name,
                time: self.time,
                days: self.days,
                callback: self.callback,
                chat_id: self.chat_id,
                user_id: self.user_id,
                data: self.data,
            })
            .await
    }
}

/// Builder for scheduling a monthly job via [`JobQueue::monthly`].
pub struct RunMonthlyBuilder<'a> {
    queue: &'a JobQueue,
    name: String,
    time: NaiveTime,
    day: i32,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl<'a> fmt::Debug for RunMonthlyBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunMonthlyBuilder")
            .field("name", &self.name)
            .field("day", &self.day)
            .finish()
    }
}

impl<'a> RunMonthlyBuilder<'a> {
    /// Sets the job name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the chat ID forwarded to the callback context.
    #[must_use]
    pub fn chat_id(mut self, id: i64) -> Self {
        self.chat_id = Some(id);
        self
    }

    /// Sets the user ID forwarded to the callback context.
    #[must_use]
    pub fn user_id(mut self, id: i64) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets arbitrary JSON data forwarded to the callback context.
    #[must_use]
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Schedules the job and returns a handle.
    pub async fn start(self) -> Job {
        self.queue
            .run_monthly(MonthlyJobSpec {
                name: self.name,
                time: self.time,
                day: self.day,
                callback: self.callback,
                chat_id: self.chat_id,
                user_id: self.user_id,
                data: self.data,
            })
            .await
    }
}

/// Builder for scheduling a custom one-shot job via [`JobQueue::custom`].
///
/// Unlike the other builders, this one takes an `Arc<JobQueue>` because the
/// registration happens inside a spawned future.
pub struct RunCustomBuilder {
    queue: Arc<JobQueue>,
    name: String,
    trigger: Duration,
    callback: JobCallbackFn,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    data: Option<serde_json::Value>,
}

impl fmt::Debug for RunCustomBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunCustomBuilder")
            .field("name", &self.name)
            .field("trigger", &self.trigger)
            .finish()
    }
}

impl RunCustomBuilder {
    /// Sets the job name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the chat ID forwarded to the callback context.
    #[must_use]
    pub fn chat_id(mut self, id: i64) -> Self {
        self.chat_id = Some(id);
        self
    }

    /// Sets the user ID forwarded to the callback context.
    #[must_use]
    pub fn user_id(mut self, id: i64) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets arbitrary JSON data forwarded to the callback context.
    #[must_use]
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Schedules the job and returns a handle.
    #[must_use]
    pub fn start(self) -> Job {
        self.queue.run_custom(
            self.callback,
            self.trigger,
            Some(self.name),
            self.data,
            self.chat_id,
            self.user_id,
        )
    }
}

// ---------------------------------------------------------------------------
// Time helpers
// ---------------------------------------------------------------------------

/// Wait until the watch channel receives `true`.
async fn wait_cancel(rx: &mut watch::Receiver<bool>) {
    while !*rx.borrow_and_update() {
        if rx.changed().await.is_err() {
            return;
        }
    }
}

/// Compute how long to sleep until the next occurrence of `time` on one of
/// the given `days` (0=Sun, 6=Sat).
fn duration_until_next_daily(time: &NaiveTime, days: &[u8]) -> Duration {
    let now = Utc::now();
    let today_dow = chrono_weekday_to_ptb(now.weekday());

    for offset in 0u32..8 {
        let candidate_dow = (today_dow + offset as u8) % 7;
        if !days.contains(&candidate_dow) {
            continue;
        }
        let candidate_date = (now + chrono::Duration::days(i64::from(offset))).date_naive();
        // Safety: hour/minute/second from a NaiveTime are always valid
        let candidate_dt = candidate_date
            .and_hms_opt(time.hour(), time.minute(), time.second())
            .expect("valid time components from NaiveTime");
        let candidate_utc = candidate_dt.and_utc();
        if candidate_utc > now {
            return (candidate_utc - now)
                .to_std()
                .unwrap_or(Duration::from_secs(1));
        }
    }
    // Fallback: 24 hours.
    Duration::from_secs(86400)
}

/// Compute how long to sleep until the next occurrence of a monthly job.
fn duration_until_next_monthly(time: &NaiveTime, day: i32) -> Duration {
    let now = Utc::now();

    for month_offset in 0u32..13 {
        let candidate_month = now.month() + month_offset;
        let year_add = (candidate_month - 1) / 12;
        let month = ((candidate_month - 1) % 12) + 1;
        let year = now.year() + year_add as i32;

        let target_day = if day == -1 {
            last_day_of_month(year, month)
        } else {
            day as u32
        };

        let candidate_date = match chrono::NaiveDate::from_ymd_opt(year, month, target_day) {
            Some(d) => d,
            None => continue, // month doesn't have this day
        };
        // Safety: hour/minute/second from a NaiveTime are always valid
        let candidate_dt = candidate_date
            .and_hms_opt(time.hour(), time.minute(), time.second())
            .expect("valid time components from NaiveTime");
        let candidate_utc = candidate_dt.and_utc();
        if candidate_utc > now {
            return (candidate_utc - now)
                .to_std()
                .unwrap_or(Duration::from_secs(1));
        }
    }
    Duration::from_secs(86400)
}

/// Map `chrono::Weekday` to PTB convention: 0 = Sunday, 6 = Saturday.
fn chrono_weekday_to_ptb(wd: Weekday) -> u8 {
    match wd {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    // The first day of the *next* month minus 1 day.
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    // Safety: month is always 1-12 and year is a valid calendar year
    chrono::NaiveDate::from_ymd_opt(y, m, 1)
        .expect("valid month for next-month calculation")
        .pred_opt()
        .expect("first day of a month always has a valid predecessor")
        .day()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    fn make_callback(counter: Arc<AtomicU32>) -> JobCallbackFn {
        Arc::new(move |_ctx| {
            let c = counter.clone();
            Box::pin(async move {
                c.fetch_add(1, Ordering::Relaxed);
                Ok(())
            })
        })
    }

    #[tokio::test]
    async fn run_once_fires_after_delay() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .once(make_callback(counter.clone()), Duration::from_millis(50))
            .name("test")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(120)).await;
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn run_once_cancelled_before_fire() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .once(make_callback(counter.clone()), Duration::from_millis(200))
            .name("cancel-test")
            .start()
            .await;
        job.schedule_removal();

        tokio::time::sleep(Duration::from_millis(300)).await;
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn run_repeating_fires_multiple_times() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(30))
            .name("repeat")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(150)).await;
        job.schedule_removal();
        let count = counter.load(Ordering::Relaxed);
        assert!(count >= 3, "expected at least 3 runs, got {count}");
    }

    #[tokio::test]
    async fn stop_cancels_all_jobs() {
        let jq = Arc::new(JobQueue::new());
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(20))
            .name("stop-test")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(80)).await;
        jq.stop().await;
        let count_at_stop = counter.load(Ordering::Relaxed);

        tokio::time::sleep(Duration::from_millis(80)).await;
        let count_after = counter.load(Ordering::Relaxed);
        // Should not have increased (much) after stop.
        assert!(count_after <= count_at_stop + 1);
    }

    #[tokio::test]
    async fn get_jobs_by_name_filters() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _a = jq
            .once(make_callback(counter.clone()), Duration::from_secs(600))
            .name("alpha")
            .start()
            .await;
        let _b = jq
            .once(make_callback(counter.clone()), Duration::from_secs(600))
            .name("beta")
            .start()
            .await;
        let _c = jq
            .once(make_callback(counter), Duration::from_secs(600))
            .name("alpha")
            .start()
            .await;

        let alphas = jq.get_jobs_by_name("alpha").await;
        assert_eq!(alphas.len(), 2);

        let all = jq.jobs().await;
        assert_eq!(all.len(), 3);

        jq.stop().await;
    }

    #[tokio::test]
    async fn run_custom_fires_after_trigger() {
        let jq = Arc::new(JobQueue::new());
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .custom(make_callback(counter.clone()), Duration::from_millis(50))
            .name("custom")
            .start();

        tokio::time::sleep(Duration::from_millis(150)).await;
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn once_builder_with_chat_id() {
        let jq = JobQueue::new();
        jq.start().await;

        let received_chat = Arc::new(Mutex::new(None));
        let rc = received_chat.clone();
        let cb: JobCallbackFn = Arc::new(move |ctx| {
            let rc = rc.clone();
            Box::pin(async move {
                *rc.lock().await = ctx.chat_id;
                Ok(())
            })
        });

        let _job = jq
            .once(cb, Duration::from_millis(10))
            .name("chat-test")
            .chat_id(42)
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(60)).await;
        assert_eq!(*received_chat.lock().await, Some(42));
        jq.stop().await;
    }

    #[tokio::test]
    async fn on_job_complete_hook_called() {
        let jq = JobQueue::new();
        jq.start().await;

        let hook_called = Arc::new(AtomicU32::new(0));
        let hc = hook_called.clone();
        jq.set_on_job_complete(Arc::new(move || {
            let hc = hc.clone();
            Box::pin(async move {
                hc.fetch_add(1, Ordering::Relaxed);
            })
        }))
        .await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .once(make_callback(counter), Duration::from_millis(10))
            .name("hook-test")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(80)).await;
        assert_eq!(hook_called.load(Ordering::Relaxed), 1);
        jq.stop().await;
    }

    #[tokio::test]
    async fn on_job_error_hook_called_on_failure() {
        let jq = JobQueue::new();
        jq.start().await;

        let error_seen = Arc::new(Mutex::new(String::new()));
        let es = error_seen.clone();
        jq.set_on_job_error(Arc::new(move |err| {
            let es = es.clone();
            Box::pin(async move {
                *es.lock().await = err.to_string();
            })
        }))
        .await;

        let complete_called = Arc::new(AtomicU32::new(0));
        let cc = complete_called.clone();
        jq.set_on_job_complete(Arc::new(move || {
            let cc = cc.clone();
            Box::pin(async move {
                cc.fetch_add(1, Ordering::Relaxed);
            })
        }))
        .await;

        // A callback that always fails.
        let failing_cb: JobCallbackFn = Arc::new(|_ctx| {
            Box::pin(async {
                Err(
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, "job failed"))
                        as Box<dyn std::error::Error + Send + Sync>,
                )
            })
        });

        let _job = jq
            .once(failing_cb, Duration::from_millis(10))
            .name("fail-test")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(80)).await;
        assert_eq!(*error_seen.lock().await, "job failed");
        // on_job_complete is called even after errors (mirrors PTB's finally block).
        assert_eq!(complete_called.load(Ordering::Relaxed), 1);
        jq.stop().await;
    }

    #[tokio::test]
    async fn on_job_complete_called_on_repeating() {
        let jq = JobQueue::new();
        jq.start().await;

        let hook_called = Arc::new(AtomicU32::new(0));
        let hc = hook_called.clone();
        jq.set_on_job_complete(Arc::new(move || {
            let hc = hc.clone();
            Box::pin(async move {
                hc.fetch_add(1, Ordering::Relaxed);
            })
        }))
        .await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(30))
            .name("repeat-hook")
            .start()
            .await;

        tokio::time::sleep(Duration::from_millis(150)).await;
        job.schedule_removal();

        let runs = counter.load(Ordering::Relaxed);
        let hooks = hook_called.load(Ordering::Relaxed);
        // Hook should be called once per run.
        assert_eq!(
            runs, hooks,
            "hook calls ({hooks}) should match job runs ({runs})"
        );
        jq.stop().await;
    }

    #[test]
    fn last_day_of_month_february_leap() {
        assert_eq!(last_day_of_month(2024, 2), 29);
        assert_eq!(last_day_of_month(2023, 2), 28);
        assert_eq!(last_day_of_month(2024, 12), 31);
        assert_eq!(last_day_of_month(2024, 4), 30);
    }

    // =====================================================================
    // R2: Additional edge-case tests
    // =====================================================================

    #[tokio::test]
    async fn zero_delay_once_job_fires_immediately() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .once(make_callback(counter.clone()), Duration::from_secs(0))
            .name("zero-delay")
            .start()
            .await;

        // Give the spawned task time to execute
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(counter.load(Ordering::Relaxed), 1);
        jq.stop().await;
    }

    #[tokio::test]
    async fn concurrent_job_scheduling() {
        let jq = Arc::new(JobQueue::new());
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));

        // Schedule 10 jobs concurrently
        let mut handles = Vec::new();
        for i in 0..10 {
            let jq = jq.clone();
            let counter = counter.clone();
            handles.push(tokio::spawn(async move {
                jq.once(make_callback(counter), Duration::from_millis(10))
                    .name(format!("concurrent-{i}"))
                    .start()
                    .await
            }));
        }

        // Wait for all to be scheduled
        for h in handles {
            let _ = h.await;
        }

        // Wait for them to fire
        tokio::time::sleep(Duration::from_millis(100)).await;

        assert_eq!(
            counter.load(Ordering::Relaxed),
            10,
            "all 10 concurrently scheduled jobs should fire"
        );
        jq.stop().await;
    }

    #[tokio::test]
    async fn disabled_job_does_not_run() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .once(make_callback(counter.clone()), Duration::from_millis(10))
            .name("disabled")
            .start()
            .await;

        // Disable the job before it fires
        job.set_enabled(false);

        tokio::time::sleep(Duration::from_millis(80)).await;
        assert_eq!(
            counter.load(Ordering::Relaxed),
            0,
            "disabled job should not execute"
        );
        jq.stop().await;
    }

    #[tokio::test]
    async fn job_enable_disable_toggle() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(20))
            .name("toggle")
            .start()
            .await;

        // Let it run a bit
        tokio::time::sleep(Duration::from_millis(60)).await;
        let count_before_disable = counter.load(Ordering::Relaxed);
        assert!(count_before_disable > 0);

        // Disable
        job.set_enabled(false);
        tokio::time::sleep(Duration::from_millis(60)).await;
        let count_after_disable = counter.load(Ordering::Relaxed);

        // Re-enable
        job.set_enabled(true);
        tokio::time::sleep(Duration::from_millis(60)).await;
        let count_after_reenable = counter.load(Ordering::Relaxed);

        // After re-enable, the counter should have increased again
        assert!(
            count_after_reenable > count_after_disable,
            "re-enabled job should continue running"
        );

        job.schedule_removal();
        jq.stop().await;
    }

    #[tokio::test]
    async fn repeating_with_first_delay() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(30))
            .name("first-delay")
            .first(Duration::from_millis(80))
            .start()
            .await;

        // At 50ms, the first delay hasn't elapsed yet
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(
            counter.load(Ordering::Relaxed),
            0,
            "job should not fire before first delay"
        );

        // At 150ms, the first delay has passed and the repeating interval has ticked
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(
            counter.load(Ordering::Relaxed) >= 1,
            "job should have fired at least once after first delay"
        );

        job.schedule_removal();
        jq.stop().await;
    }

    #[tokio::test]
    async fn repeating_with_last_deadline() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _job = jq
            .repeating(make_callback(counter.clone()), Duration::from_millis(20))
            .name("deadline")
            .last(Duration::from_millis(80))
            .start()
            .await;

        // Let it run past the deadline
        tokio::time::sleep(Duration::from_millis(150)).await;
        let count_at_deadline = counter.load(Ordering::Relaxed);

        // Wait more -- count should not increase
        tokio::time::sleep(Duration::from_millis(80)).await;
        let count_after = counter.load(Ordering::Relaxed);
        assert_eq!(
            count_at_deadline, count_after,
            "job should stop after last deadline"
        );

        jq.stop().await;
    }

    #[test]
    fn chrono_weekday_mapping() {
        assert_eq!(chrono_weekday_to_ptb(Weekday::Sun), 0);
        assert_eq!(chrono_weekday_to_ptb(Weekday::Mon), 1);
        assert_eq!(chrono_weekday_to_ptb(Weekday::Sat), 6);
    }

    #[test]
    fn duration_until_next_daily_returns_positive() {
        // All days enabled -- the next daily occurrence must be positive
        let time = NaiveTime::from_hms_opt(0, 0, 1).unwrap();
        let all_days: Vec<u8> = (0..7).collect();
        let dur = duration_until_next_daily(&time, &all_days);
        assert!(dur.as_secs() > 0 || dur.as_millis() > 0);
    }

    #[test]
    fn duration_until_next_monthly_returns_positive() {
        let time = NaiveTime::from_hms_opt(0, 0, 1).unwrap();
        let dur = duration_until_next_monthly(&time, 1);
        assert!(dur.as_secs() > 0);
    }

    #[test]
    fn duration_until_next_monthly_last_day() {
        let time = NaiveTime::from_hms_opt(0, 0, 1).unwrap();
        let dur = duration_until_next_monthly(&time, -1);
        assert!(dur.as_secs() > 0);
    }

    #[tokio::test]
    async fn jobs_by_pattern_works() {
        let jq = JobQueue::new();
        jq.start().await;

        let counter = Arc::new(AtomicU32::new(0));
        let _a = jq
            .once(make_callback(counter.clone()), Duration::from_secs(600))
            .name("daily_report")
            .start()
            .await;
        let _b = jq
            .once(make_callback(counter.clone()), Duration::from_secs(600))
            .name("daily_cleanup")
            .start()
            .await;
        let _c = jq
            .once(make_callback(counter), Duration::from_secs(600))
            .name("weekly_report")
            .start()
            .await;

        let daily_jobs = jq.jobs_by_pattern("^daily_").await;
        assert_eq!(daily_jobs.len(), 2);

        // Invalid regex returns empty vec (not panic)
        let bad_pattern = jq.jobs_by_pattern("[invalid").await;
        assert!(bad_pattern.is_empty());

        jq.stop().await;
    }
}

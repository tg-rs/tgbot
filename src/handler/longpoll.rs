use std::{cmp::max, collections::HashSet, sync::Arc, time::Duration};

use async_stream::stream;
use futures_util::{pin_mut, stream::StreamExt};
use log::error;
use tokio::{
    spawn,
    sync::mpsc::{Receiver, Sender, channel},
    time::sleep,
};

use crate::{
    api::{Client, ExecuteError},
    handler::UpdateHandler,
    types::{AllowedUpdate, GetUpdates, Integer},
};

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_ERROR_TIMEOUT: Duration = Duration::from_secs(5);

/// Allows receiving incoming updates from the Telegram Bot API using long polling.
pub struct LongPoll<H> {
    client: Client,
    handler: Arc<H>,
    options: LongPollOptions,
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl<H> LongPoll<H> {
    /// Creates a new `LongPoll`.
    ///
    /// # Arguments
    ///
    /// * `client` - Telegram Bot API Client.
    /// * `handler` - Updates Handler.
    pub fn new(client: Client, handler: H) -> Self {
        let (sender, receiver) = channel(1);
        Self {
            client,
            handler: Arc::new(handler),
            options: LongPollOptions::default(),
            sender,
            receiver,
        }
    }

    /// Sets a new polling options
    ///
    /// # Arguments
    ///
    /// * `value` - Polling options to be set.
    pub fn with_options(mut self, options: LongPollOptions) -> Self {
        self.options = options;
        self
    }
}

impl<H> LongPoll<H>
where
    H: UpdateHandler + Send + Sync + 'static,
{
    /// Returns a handle allowing control over the polling loop.
    #[must_use]
    pub fn get_handle(&self) -> LongPollHandle {
        LongPollHandle {
            sender: self.sender.clone(),
        }
    }

    /// Starts the polling loop.
    pub async fn run(self) {
        let LongPollOptions {
            mut offset,
            limit,
            poll_timeout,
            error_timeout,
            allowed_updates,
        } = self.options;
        let client = self.client.clone();
        let mut receiver = self.receiver;
        let s = stream! {
            loop {
                if receiver.try_recv().is_ok() {
                    receiver.close();
                    break;
                }
                let method = GetUpdates::default()
                    .with_allowed_updates(allowed_updates.clone())
                    .with_limit(limit)
                    .with_offset(offset + 1)
                    .with_timeout(poll_timeout);
                let updates = match client.execute(method).await {
                    Ok(updates) => updates,
                    Err(err) => {
                        error!("An error has occurred while getting updates: {err}");
                        let error_timeout = get_error_timeout(err, error_timeout);
                        sleep(error_timeout).await;
                        continue
                    }
                };
                for update in updates {
                    offset = max(offset, update.id);
                    yield update
                }
            }
        };
        pin_mut!(s);
        while let Some(update) = s.next().await {
            let handler = self.handler.clone();
            spawn(async move { handler.handle(update).await });
        }
    }
}

/// Allows to control a polling loop.
pub struct LongPollHandle {
    sender: Sender<()>,
}

impl LongPollHandle {
    /// Stops the associated polling loop.
    pub async fn shutdown(self) {
        let _ = self.sender.send(()).await;
    }
}

fn get_error_timeout(err: ExecuteError, default_timeout: Duration) -> Duration {
    match err {
        ExecuteError::Response(err) => err.retry_after().map(Duration::from_secs).unwrap_or(default_timeout),
        _ => default_timeout,
    }
}

/// Represents options for configuring long polling behavior.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LongPollOptions {
    offset: Integer,
    limit: Integer,
    poll_timeout: Duration,
    error_timeout: Duration,
    allowed_updates: HashSet<AllowedUpdate>,
}

impl LongPollOptions {
    /// Adds a type of updates that you want your bot to receive.
    ///
    /// # Arguments
    ///
    /// * `value` - A type of update to be allowed.
    pub fn with_allowed_update(mut self, value: AllowedUpdate) -> Self {
        self.allowed_updates.insert(value);
        self
    }

    /// Sets a new error timeout.
    ///
    /// # Arguments
    ///
    /// * `value` - Timeout in seconds when an error has occurred; default - 5.
    pub fn with_error_timeout(mut self, value: u64) -> Self {
        self.error_timeout = Duration::from_secs(value);
        self
    }

    /// Sets a new limit for the number of updates to be retrieved.
    ///
    /// # Arguments
    ///
    /// * `value` - Limit of the number of updates to be retrieved; 1—100; default - 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = value;
        self
    }

    /// Sets a new timeout for long polling.
    ///
    /// # Arguments
    ///
    /// * `value` - Timeout for long polling in seconds;
    ///   0 - usual short polling;
    ///   default - 10.
    ///
    /// Should be positive, short polling should be used for testing purposes only.
    pub fn with_poll_timeout(mut self, value: Duration) -> Self {
        self.poll_timeout = value;
        self
    }
}

impl Default for LongPollOptions {
    fn default() -> Self {
        LongPollOptions {
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: DEFAULT_ERROR_TIMEOUT,
            allowed_updates: HashSet::new(),
        }
    }
}

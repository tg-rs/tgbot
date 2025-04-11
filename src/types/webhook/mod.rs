use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{AllowedUpdate, Integer},
};

#[cfg(test)]
mod tests;

/// Represents a current status of a webhook.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct WebhookInfo {
    /// Indicates whether a custom certificate was provided for webhook certificate checks.
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery.
    pub pending_update_count: Integer,
    /// Webhook URL, may be empty if webhook is not set up.
    pub url: String,
    /// A list of update types the bot is subscribed to.
    ///
    /// Defaults to all update types.
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    /// Currently used webhook IP address.
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened
    /// when trying to deliver an update via webhook.
    pub last_error_date: Option<Integer>,
    /// Error message in human-readable format for the most recent error
    /// that happened when trying to deliver an update via webhook.
    pub last_error_message: Option<String>,
    /// Unix time of the most recent error that happened
    /// when trying to synchronize available updates with Telegram datacenters.
    pub last_synchronization_error_date: Option<Integer>,
    /// Maximum allowed number of simultaneous HTTPS connections
    /// to the webhook for update delivery.
    pub max_connections: Option<Integer>,
}

impl WebhookInfo {
    /// Creates a new `WebhookInfo`.
    ///
    /// # Arguments
    ///
    /// * `url` - Webhook URL.
    /// * `has_custom_certificate` - Indicates whether a custom certificate was provided.
    /// * `pending_update_count` - Number of updates awaiting delivery.
    pub fn new<T>(url: T, has_custom_certificate: bool, pending_update_count: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            has_custom_certificate,
            pending_update_count,
            url: url.into(),
            allowed_updates: None,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
        }
    }

    /// Sets a new list of allowed updates.
    ///
    /// # Arguments
    ///
    /// * `value` - List of allowed updates.
    pub fn with_allowed_updates<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = AllowedUpdate>,
    {
        self.allowed_updates = Some(value.into_iter().collect());
        self
    }

    /// Sets a new IP address.
    ///
    /// # Arguments
    ///
    /// * `value` - IP address.
    pub fn with_ip_address<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.ip_address = Some(value.into());
        self
    }

    /// Sets a new last error date.
    ///
    /// # Arguments
    ///
    /// * `value` - Last error date.
    pub fn with_last_error_date(mut self, value: Integer) -> Self {
        self.last_error_date = Some(value);
        self
    }

    /// Sets a new last error message.
    ///
    /// # Arguments
    ///
    /// * `value` - Last error message.
    pub fn with_last_error_message<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_error_message = Some(value.into());
        self
    }

    /// Sets a new last synchronization error date.
    ///
    /// # Arguments
    ///
    /// * `value` - Last synchronization error date.
    pub fn with_last_synchronization_error_date(mut self, value: Integer) -> Self {
        self.last_synchronization_error_date = Some(value);
        self
    }

    /// Sets a new number of max connections.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of max connections.
    pub fn with_max_connections(mut self, value: Integer) -> Self {
        self.max_connections = Some(value);
        self
    }
}

/// Removes a webhook integration if you decide to switch back to [`crate::types::GetUpdates`].
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct DeleteWebhook {
    drop_pending_updates: Option<bool>,
}

impl DeleteWebhook {
    /// Sets a new value for the `drop_pending_updates` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to drop all pending updates.
    pub fn with_drop_pending_updates(mut self, value: bool) -> Self {
        self.drop_pending_updates = Some(value);
        self
    }
}

impl Method for DeleteWebhook {
    type Response = bool;

    fn into_payload(self) -> Payload {
        if self.drop_pending_updates.is_some() {
            Payload::json("deleteWebhook", self)
        } else {
            Payload::empty("deleteWebhook")
        }
    }
}

/// Returns current webhook status.
#[derive(Clone, Copy, Debug)]
pub struct GetWebhookInfo;

impl Method for GetWebhookInfo {
    type Response = WebhookInfo;

    fn into_payload(self) -> Payload {
        Payload::empty("getWebhookInfo")
    }
}

/// Specifies an url and returns incoming updates via an outgoing webhook.
///
/// Whenever there is an update for the bot, we will send an HTTPS POST request
/// to the specified url, containing a JSON-serialized Update.
/// In case of an unsuccessful request, we will give up after a reasonable amount of attempts.
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g. `https://www.example.com/<token>`
/// Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetWebhook {
    url: String,
    allowed_updates: Option<HashSet<AllowedUpdate>>,
    certificate: Option<String>,
    drop_pending_updates: Option<bool>,
    ip_address: Option<String>,
    max_connections: Option<Integer>,
    secret_token: Option<String>,
}

impl SetWebhook {
    /// Creates a new `SetWebhook`.
    ///
    /// # Arguments
    ///
    /// * `url` - HTTPS url to send updates to; use an empty string to remove webhook integration.
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            allowed_updates: None,
            certificate: None,
            drop_pending_updates: None,
            ip_address: None,
            max_connections: None,
            secret_token: None,
        }
    }

    /// Adds a type of update you want your bot to receive.
    ///
    /// # Arguments
    ///
    /// * `value` - A type to add.
    pub fn add_allowed_update(mut self, value: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(value);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(value);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }

    /// Sets a new list of allowed update types.
    ///
    /// # Arguments
    ///
    /// * `value` - List of types you want your bot to receive.
    ///
    /// For example, specify `[AllowedUpdate::Message]`
    /// to only receive updates of these types.
    /// See [`AllowedUpdate`] for a complete list of available update types.
    /// Specify an empty list to receive all updates regardless of type (default).
    /// If not specified, the previous setting will be used.
    /// Please note that this parameter doesn't affect
    /// updates created before the call to the [`SetWebhook`],
    /// so unwanted updates may be received for a short period of time.
    pub fn with_allowed_updates(mut self, value: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(value);
        self
    }

    /// Sets a new certificate.
    ///
    /// # Arguments
    ///
    /// * `value` - Public key certificate; so that the root certificate in use can be checked.
    pub fn with_certificate<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.certificate = Some(value.into());
        self
    }

    /// Sets a new value for the `drop_pending_updates` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to drop all pending updates.
    pub fn with_drop_pending_updates(mut self, value: bool) -> Self {
        self.drop_pending_updates = Some(value);
        self
    }

    /// Sets a new IP address.
    ///
    /// # Arguments
    ///
    /// * `value` - The fixed IP address which will be used to send webhook requests
    ///   instead of the IP address resolved through DNS.
    pub fn with_ip_address<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.ip_address = Some(value.into());
        self
    }

    /// Sets a new number of max connections.
    ///
    /// # Arguments
    ///
    /// * `value` - Maximum allowed number of simultaneous HTTPS connections
    ///   to the webhook for update delivery; 1-100; default - 40.
    ///
    /// Use lower values to limit the load on your bot‘s server,
    /// and higher values to increase your bot’s throughput.
    pub fn with_max_connections(mut self, value: Integer) -> Self {
        self.max_connections = Some(value);
        self
    }

    /// Sets a new secret token.
    ///
    /// # Arguments
    ///
    /// * `value` - A secret token to be sent in a header.
    ///
    /// “X-Telegram-Bot-Api-Secret-Token” in every webhook request; 1-256 characters
    ///
    /// Only characters A-Z, a-z, 0-9, _ and - are allowed.
    /// The header is useful to ensure that the request comes from a webhook set by you.
    pub fn with_secret_token<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.secret_token = Some(value.into());
        self
    }
}

impl Method for SetWebhook {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setWebhook", self)
    }
}

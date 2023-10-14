use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{AllowedUpdate, Integer},
};

#[cfg(test)]
mod tests;

/// Information about the current status of a webhook
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: Integer,
    /// Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///  Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<Integer>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<Integer>,
    /// A list of update types the bot is subscribed to
    /// Defaults to all update types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

/// Remove webhook integration if you decide to switch back to getUpdates
///
/// Returns True on success
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct DeleteWebhook {
    drop_pending_updates: Option<bool>,
}

impl DeleteWebhook {
    /// Pass true to drop all pending updates
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}

impl Method for DeleteWebhook {
    type Response = bool;

    fn into_request(self) -> Request {
        if self.drop_pending_updates.is_some() {
            Request::json("deleteWebhook", self)
        } else {
            Request::empty("deleteWebhook")
        }
    }
}

/// Get current webhook status
#[derive(Clone, Copy, Debug)]
pub struct GetWebhookInfo;

impl Method for GetWebhookInfo {
    type Response = WebhookInfo;

    fn into_request(self) -> Request {
        Request::empty("getWebhookInfo")
    }
}

/// Specify a url and receive incoming updates via an outgoing webhook
///
/// Whenever there is an update for the bot, we will send an HTTPS POST request
/// to the specified url, containing a JSON-serialized Update
/// In case of an unsuccessful request, we will give up after a reasonable amount of attempts
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g. `https://www.example.com/<token>`
/// Since nobody else knows your bot‘s token, you can be pretty sure it’s us
#[derive(Clone, Debug, Serialize)]
pub struct SetWebhook {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}

impl SetWebhook {
    /// Creates a new SetWebhook with given url
    ///
    /// # Arguments
    ///
    /// * url - HTTPS url to send updates to
    ///         Use an empty string to remove webhook integration
    pub fn new<S: Into<String>>(url: S) -> Self {
        SetWebhook {
            url: url.into(),
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
        }
    }

    /// Upload your public key certificate so that the root certificate in use can be checked
    pub fn certificate<C: Into<String>>(mut self, certificate: C) -> Self {
        self.certificate = Some(certificate.into());
        self
    }

    /// The fixed IP address which will be used to send webhook requests
    /// instead of the IP address resolved through DNS
    pub fn ip_address<A: Into<String>>(mut self, ip_address: A) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100
    ///
    /// Defaults to 40
    /// Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput
    pub fn max_connections(mut self, max_connections: Integer) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”]
    /// to only receive updates of these types
    /// See Update for a complete list of available update types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the setWebhook,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }

    /// Pass true to drop all pending updates
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}

impl Method for SetWebhook {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setWebhook", self)
    }
}

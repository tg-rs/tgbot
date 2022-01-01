use crate::{
    methods::Method,
    request::Request,
    types::{AllowedUpdate, Integer},
};
use serde::Serialize;
use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_webhook() {
        let request = SetWebhook::new("url").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
        match request.into_body() {
            RequestBody::Json(data) => {
                assert_eq!(data.unwrap(), r#"{"url":"url"}"#);
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let mut updates = HashSet::new();
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::EditedMessage);
        updates.insert(AllowedUpdate::ChannelPost);
        updates.insert(AllowedUpdate::EditedChannelPost);
        updates.insert(AllowedUpdate::ChosenInlineResult);
        let request = SetWebhook::new("url")
            .certificate("cert")
            .ip_address("127.0.0.1")
            .max_connections(10)
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::InlineQuery)
            .add_allowed_update(AllowedUpdate::CallbackQuery)
            .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
            .add_allowed_update(AllowedUpdate::ShippingQuery)
            .drop_pending_updates(true)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(data["certificate"], "cert");
                assert_eq!(data["ip_address"], "127.0.0.1");
                assert_eq!(data["max_connections"], 10);
                assert!(data["drop_pending_updates"].as_bool().unwrap());
                let mut updates: Vec<&str> = data["allowed_updates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect();
                updates.sort_unstable();
                assert_eq!(
                    updates,
                    vec![
                        "callback_query",
                        "channel_post",
                        "chosen_inline_result",
                        "edited_channel_post",
                        "edited_message",
                        "inline_query",
                        "message",
                        "pre_checkout_query",
                        "shipping_query",
                    ]
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let method = SetWebhook::new("url").add_allowed_update(AllowedUpdate::Message);
        assert_eq!(method.allowed_updates.unwrap().len(), 1);
    }
}

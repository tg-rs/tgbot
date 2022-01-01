use crate::{methods::Method, request::Request};
use serde::Serialize;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn delete_webhook() {
        let request = DeleteWebhook::default().into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteWebhook"
        );
        match request.into_body() {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }

        let request = DeleteWebhook::default().drop_pending_updates(false).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteWebhook"
        );
        match request.into_body() {
            RequestBody::Json(data) => {
                assert_eq!(data.unwrap(), r#"{"drop_pending_updates":false}"#);
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}

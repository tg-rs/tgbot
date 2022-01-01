use crate::{methods::Method, request::Request, types::WebhookInfo};

/// Get current webhook status
#[derive(Clone, Copy, Debug)]
pub struct GetWebhookInfo;

impl Method for GetWebhookInfo {
    type Response = WebhookInfo;

    fn into_request(self) -> Request {
        Request::empty("getWebhookInfo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_webhook_info() {
        let request = GetWebhookInfo.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getWebhookInfo"
        );
        match request.into_body() {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}

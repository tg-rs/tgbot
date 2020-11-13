use crate::{methods::Method, request::Request};

/// Close the bot instance before moving it from one local server to another
///
/// You need to delete the webhook before calling this method to ensure
/// that the bot isn't launched again after server restart.
///
/// The method will return error 429 in the first 10 minutes after the bot is launched.
#[derive(Clone, Copy, Debug)]
pub struct Close;

impl Method for Close {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::empty("close")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_me() {
        let request = Close.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/close");
        if let RequestBody::Empty = request.into_body() {
        } else {
            panic!("Unexpected request body");
        }
    }
}

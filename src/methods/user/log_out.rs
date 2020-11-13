use crate::{methods::Method, request::Request};

/// Log out from the cloud Bot API
///
/// You must log out the bot before running it locally,
/// otherwise there is no guarantee that the bot will receive updates.
/// After a successful call, you can immediately log in on a local server,
/// but will not be able to log in back to the cloud Bot API server for 10 minutes.
#[derive(Clone, Copy, Debug)]
pub struct LogOut;

impl Method for LogOut {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::empty("logOut")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_me() {
        let request = LogOut.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/logOut");
        if let RequestBody::Empty = request.into_body() {
        } else {
            panic!("Unexpected request body");
        }
    }
}

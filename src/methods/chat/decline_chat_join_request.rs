use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Decline a chat join request
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_invite_users administrator right.
#[derive(Clone, Debug, Serialize)]
pub struct DeclineChatJoinRequest {
    chat_id: ChatId,
    user_id: Integer,
}

impl DeclineChatJoinRequest {
    /// Creates a new DeclineChatJoinRequest
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for DeclineChatJoinRequest {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("declineChatJoinRequest", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn approve_chat_join_request() {
        let request = DeclineChatJoinRequest::new(1, 1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/declineChatJoinRequest"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "user_id": 1,
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

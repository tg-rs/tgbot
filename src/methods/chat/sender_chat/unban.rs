use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Unban a previously banned channel chat in a supergroup or channel
///
/// The bot must be an administrator for this to work and must have
/// the appropriate administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatSenderChat {
    chat_id: ChatId,
    sender_chat_id: Integer,
}

impl UnbanChatSenderChat {
    /// Creates a new UnbanChatSenderChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target
    /// * sender_chat_id - Unique identifier of the target sender chat
    pub fn new<C: Into<ChatId>>(chat_id: C, sender_chat_id: Integer) -> Self {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }
}

impl Method for UnbanChatSenderChat {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unbanChatSenderChat", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn ban_chat_sender_chat() {
        let request = UnbanChatSenderChat::new(1, 1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unbanChatSenderChat"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "sender_chat_id": 1,
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

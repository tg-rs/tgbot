use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Ban a channel chat in a supergroup or a channel
///
/// Until the chat is unbanned, the owner of the banned chat won't be able to send messages
/// on behalf of any of their channels. The bot must be an administrator in the supergroup or
/// channel for this to work and must have the appropriate administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct BanChatSenderChat {
    chat_id: ChatId,
    sender_chat_id: Integer,
}

impl BanChatSenderChat {
    /// Creates a new BanChatSenderChat
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

impl Method for BanChatSenderChat {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("banChatSenderChat", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn ban_chat_sender_chat() {
        let request = BanChatSenderChat::new(1, 1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/banChatSenderChat"
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

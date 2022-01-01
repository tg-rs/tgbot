use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Remove a message from the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
}

impl UnpinChatMessage {
    /// Creates a new UnpinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinChatMessage {
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Identifier of a message to unpin
    ///
    /// If not specified, the most recent pinned message (by sending date) will be unpinned
    pub fn message_id(mut self, message_id: Integer) -> Self {
        self.message_id = Some(message_id);
        self
    }
}

impl Method for UnpinChatMessage {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unpinChatMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn unpin_chat_message_full() {
        let request = UnpinChatMessage::new(1).message_id(2).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unpinChatMessage"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn unpin_chat_message_partial() {
        let request = UnpinChatMessage::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unpinChatMessage"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{"chat_id":1}"#);
        } else {
            panic!("Unexpected request body");
        }
    }
}

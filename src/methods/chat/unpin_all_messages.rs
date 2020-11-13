use crate::{methods::Method, request::Request, types::ChatId};
use serde::Serialize;

/// Clear the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllChatMessages {
    chat_id: ChatId,
}

impl UnpinAllChatMessages {
    /// Creates a new UnpinAllChatMessages
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinAllChatMessages {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for UnpinAllChatMessages {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unpinAllChatMessages", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn unpin_all_chat_messages() {
        let request = UnpinAllChatMessages::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unpinAllChatMessages"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}

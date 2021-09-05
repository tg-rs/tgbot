use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Get the number of members in a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMemberCount {
    chat_id: ChatId,
}

impl GetChatMemberCount {
    /// Creates a new GetChatMemberCount
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatMemberCount {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatMemberCount {
    type Response = Integer;

    fn into_request(self) -> Request {
        Request::json("getChatMemberCount", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_chat_members_count() {
        let request = GetChatMemberCount::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getChatMemberCount"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}

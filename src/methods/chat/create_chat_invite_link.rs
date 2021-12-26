use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, ChatInviteLink, Integer},
};
use serde::Serialize;

/// Create an additional invite link for a chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// The link can be revoked using the method `RevokeChatInviteLink`.
/// Returns the new invite link as `ChatInviteLink` object.
#[derive(Clone, Debug, Serialize)]
pub struct CreateChatInviteLink {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl CreateChatInviteLink {
    /// Creates a new CreateChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    /// Sets point in time (Unix timestamp) when the link will expire
    pub fn expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    pub fn member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// True, if users joining the chat via the link need to be approved by chat administrators.
    /// If True, member_limit can't be specified
    pub fn creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }
}

impl Method for CreateChatInviteLink {
    type Response = ChatInviteLink;

    fn into_request(self) -> Request {
        Request::json("createChatInviteLink", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn create_chat_invite_link() {
        let request = CreateChatInviteLink::new(1)
            .expire_date(0)
            .member_limit(1)
            .creates_join_request(false)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/createChatInviteLink"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "expire_date": 0,
                    "member_limit": 1,
                    "creates_join_request": false
                })
            );
        } else {
            panic!("Unexpected request body");
        }

        let request = CreateChatInviteLink::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/createChatInviteLink"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data, serde_json::json!({"chat_id": 1}));
        } else {
            panic!("Unexpected request body");
        }
    }
}

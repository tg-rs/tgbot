use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, ChatInviteLink},
};
use serde::Serialize;

/// Revoke an invite link created by the bot
///
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights.
/// Returns the revoked invite link as ChatInviteLink object.
#[derive(Clone, Debug, Serialize)]
pub struct RevokeChatInviteLink {
    chat_id: ChatId,
    invite_link: String,
}

impl RevokeChatInviteLink {
    /// Creates a new RevokeChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * invite_link - The invite link to revoke
    pub fn new<C, I>(chat_id: C, invite_link: I) -> Self
    where
        C: Into<ChatId>,
        I: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }
}

impl Method for RevokeChatInviteLink {
    type Response = ChatInviteLink;

    fn into_request(self) -> Request {
        Request::json("revokeChatInviteLink", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn create_chat_invite_link() {
        let request = RevokeChatInviteLink::new(1, "test").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/revokeChatInviteLink"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data, serde_json::json!({"chat_id": 1, "invite_link": "test"}));
        } else {
            panic!("Unexpected request body");
        }
    }
}

use crate::types::{primitive::Integer, user::User};
use serde::Deserialize;

/// Represents an invite link for a chat.
#[derive(Clone, Debug, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link
    ///
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<Integer>,
    /// Maximum number of users that can be members
    /// of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    pub member_limit: Option<Integer>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat_invite_link_partial() {
        let data: ChatInviteLink = serde_json::from_value(serde_json::json!({
            "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
            "creator": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            },
            "creates_join_request": false,
            "is_primary": true,
            "is_revoked": false
        }))
        .unwrap();
        assert_eq!(data.invite_link, "https://t.me/joinchat/o8oIBrbCI3U2OGJi");
        assert_eq!(data.creator.id, 1);
        assert!(!data.creates_join_request);
        assert!(data.is_primary);
        assert!(!data.is_revoked);
        assert!(data.expire_date.is_none());
        assert!(data.member_limit.is_none());
    }

    #[test]
    fn deserialize_chat_invite_link_full() {
        let data: ChatInviteLink = serde_json::from_value(serde_json::json!({
            "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
            "creator": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            },
            "creates_join_request": true,
            "is_primary": true,
            "is_revoked": false,
            "name": "test",
            "expire_date": 0,
            "member_limit": 10,
            "pending_join_request_count": 0
        }))
        .unwrap();
        assert_eq!(data.invite_link, "https://t.me/joinchat/o8oIBrbCI3U2OGJi");
        assert_eq!(data.creator.id, 1);
        assert!(data.creates_join_request);
        assert!(data.is_primary);
        assert!(!data.is_revoked);
        assert_eq!(data.name, Some(String::from("test")));
        assert_eq!(data.expire_date, Some(0));
        assert_eq!(data.member_limit, Some(10));
        assert_eq!(data.pending_join_request_count, Some(0));
    }
}

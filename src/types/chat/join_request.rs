use crate::types::{
    chat::{invite_link::ChatInviteLink, Chat},
    primitive::Integer,
    User,
};
use serde::Deserialize;

/// Represents a join request sent to a chat
#[derive(Clone, Debug, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: Integer,
    /// Bio of the user
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat_join_request_partial() {
        let data: ChatJoinRequest = serde_json::from_value(serde_json::json!({
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            },
            "date": 0
        }))
        .unwrap();
        assert_eq!(data.chat.get_id(), 1);
        assert_eq!(data.from.id, 1);
        assert_eq!(data.date, 0);
    }

    #[test]
    fn deserialize_chat_join_request_full() {
        let data: ChatJoinRequest = serde_json::from_value(serde_json::json!({
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            },
            "date": 0,
            "bio": "bio",
            "invite_link": {
                "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
                "creator": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "firstname"
                },
                "creates_join_request": false,
                "is_primary": true,
                "is_revoked": false
            }
        }))
        .unwrap();
        assert_eq!(data.chat.get_id(), 1);
        assert_eq!(data.from.id, 1);
        assert_eq!(data.date, 0);
        assert_eq!(data.bio, Some(String::from("bio")));
        assert_eq!(
            data.invite_link.unwrap().invite_link,
            "https://t.me/joinchat/o8oIBrbCI3U2OGJi"
        );
    }
}

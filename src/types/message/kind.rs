use crate::types::{
    chat::{ChannelChat, GroupChat, PrivateChat, SupergroupChat},
    user::User,
};

/// Contains chat-specific data
#[derive(Clone, Debug)]
pub enum MessageKind {
    /// Channel chat
    Channel {
        /// Channel chat
        chat: ChannelChat,
        /// Author signature, if exists
        author_signature: Option<String>,
    },
    /// Group chat
    Group {
        /// Group chat
        chat: GroupChat,
        /// Sender
        from: User,
        /// Custom title of an anonymous group administrator
        author_signature: Option<String>,
    },
    /// Private chat
    Private {
        /// Private chat
        chat: PrivateChat,
        /// Sender
        from: User,
    },
    /// Supergroup chat
    Supergroup {
        /// Supergroup chat
        chat: SupergroupChat,
        /// Sender
        from: User,
        /// Custom title of an anonymous group administrator
        author_signature: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, MessageData, Text};
    use serde_json::json;

    #[test]
    fn deserialize_channel_chat() {
        let input = json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "text": "test"
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        assert_eq!(msg.get_chat_id(), 1);
        assert!(msg.get_user().is_none());
        assert!(!msg.is_edited());
        if let MessageKind::Channel {
            chat: ChannelChat { id, title, .. },
            author_signature,
        } = msg.kind
        {
            assert_eq!(id, 1);
            assert_eq!(title, "channeltitle");
            assert_eq!(author_signature, None);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert!(entities.is_none());
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }

    #[test]
    fn deserialize_group_chat() {
        let input = json!({
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "text": "test",
            "edit_date": 1
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        assert_eq!(msg.get_chat_id(), 1);
        assert_eq!(msg.get_user().map(|u| u.id), Some(1));
        assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
        assert!(msg.is_edited());
        if let MessageKind::Group {
            chat,
            from,
            author_signature,
        } = msg.kind
        {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
            assert!(author_signature.is_none());
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert!(entities.is_none());
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = json!({
            "message_id": 1, "date": 0, "text": "test",
            "chat": {"id": 1, "type": "group", "title": "grouptitle"}
        });
        let err = serde_json::from_value::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), "missing field `from`");
    }

    #[test]
    fn deserialize_private_chat() {
        let input = json!({
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "private",
                "first_name": "firstname"
            },
            "text": "test"
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        assert_eq!(msg.get_chat_id(), 1);
        assert_eq!(msg.get_user().map(|u| u.id), Some(1));
        assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
        if let MessageKind::Private { chat, from } = msg.kind {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.first_name, "firstname");
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert!(entities.is_none());
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = json!({
            "message_id": 1, "date": 0, "text": "test",
            "chat": {"id": 1, "type": "private", "first_name": "firstname"}
        });
        let err = serde_json::from_value::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), "missing field `from`");
    }

    #[test]
    fn deserialize_supergroup_chat() {
        let input = json!({
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "supergroup",
                "title": "supergrouptitle",
                "username": "supergroupusername"
            },
            "text": "test"
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        assert_eq!(msg.get_chat_id(), 1);
        assert_eq!(msg.get_chat_username().unwrap(), "supergroupusername");
        assert_eq!(msg.get_user().map(|u| u.id), Some(1));
        assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
        if let MessageKind::Supergroup {
            chat,
            from,
            author_signature,
        } = msg.kind
        {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
            assert!(author_signature.is_none());
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert!(entities.is_none());
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = json!({
            "message_id": 1, "date": 0,
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        });
        let err = serde_json::from_value::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), "missing field `from`");
    }
}

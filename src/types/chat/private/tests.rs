use crate::types::{
    tests::assert_json_eq,
    Chat,
    ChatPhoto,
    Message,
    MessageData,
    MessageSender,
    PrivateChat,
    Text,
    User,
};

#[test]
fn private_chat() {
    let expected_struct = Chat::Private(PrivateChat {
        id: 1,
        first_name: String::from("John"),
        last_name: Some(String::from("Doe")),
        username: Some(String::from("john_doe")),
        photo: Some(ChatPhoto {
            small_file_id: String::from("small-file-id"),
            small_file_unique_id: String::from("small-file-unique-id"),
            big_file_id: String::from("big-file-id"),
            big_file_unique_id: String::from("big-file-unique-id"),
        }),
        bio: Some(String::from("Bio")),
        pinned_message: Some(Box::new(Message {
            id: 1,
            date: 0,
            edit_date: Some(0),
            sender: MessageSender::User(User {
                id: 1,
                is_bot: false,
                first_name: String::from("John"),
                last_name: Some(String::from("Doe")),
                username: Some(String::from("john_doe")),
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            chat: Chat::Private(PrivateChat {
                id: 1,
                first_name: String::from("John"),
                last_name: Some(String::from("Doe")),
                username: Some(String::from("john_doe")),
                photo: None,
                bio: None,
                pinned_message: None,
                has_private_forwards: None,
                message_auto_delete_time: None,
                has_restricted_voice_and_video_messages: None,
                active_usernames: None,
                emoji_status_custom_emoji_id: None,
            }),
            author_signature: None,
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            has_media_spoiler: None,
            data: MessageData::Text(Text {
                data: String::from("message-text"),
                entities: None,
            }),
        })),
        has_private_forwards: Some(true),
        message_auto_delete_time: Some(86400),
        has_restricted_voice_and_video_messages: Some(true),
        active_usernames: Some(vec![String::from("john_doe")]),
        emoji_status_custom_emoji_id: Some(String::from("emoji-id")),
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "john_doe");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "private",
            "username": "john_doe",
            "first_name": "John",
            "last_name": "Doe",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "bio": "Bio",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe"
                },
                "text": "message-text",
                "edit_date": 0,
                "has_protected_content": false,
                "is_automatic_forward": false,
            },
            "has_private_forwards": true,
            "message_auto_delete_time": 86400,
            "has_restricted_voice_and_video_messages": true,
            "active_usernames": ["john_doe"],
            "emoji_status_custom_emoji_id": "emoji-id"
        }),
    );
    let expected_struct = Chat::Private(PrivateChat {
        id: 1,
        first_name: String::from("John"),
        last_name: None,
        username: None,
        photo: None,
        bio: None,
        pinned_message: None,
        has_private_forwards: None,
        message_auto_delete_time: None,
        has_restricted_voice_and_video_messages: None,
        active_usernames: None,
        emoji_status_custom_emoji_id: None,
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "private",
            "first_name": "John",
        }),
    );
}

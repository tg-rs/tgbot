use crate::{
    tests::assert_json_eq,
    types::{ChannelChat, Chat, ChatPhoto, Message, MessageData, MessageSender, Text, User},
};

#[test]
fn channel_chat() {
    let expected_struct = Chat::Channel(ChannelChat {
        id: 1,
        title: String::from("Channel"),
        username: Some(String::from("channel_username")),
        photo: Some(ChatPhoto {
            small_file_id: String::from("small-file-id"),
            small_file_unique_id: String::from("small-file-unique-id"),
            big_file_id: String::from("big-file-id"),
            big_file_unique_id: String::from("big-file-unique-id"),
        }),
        description: Some(String::from("Description")),
        invite_link: Some(String::from("example.com/join/channel")),
        pinned_message: Some(Box::new(Message {
            id: 1,
            date: 0,
            edit_date: None,
            sender: MessageSender::User(User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
            }),
            chat: Chat::Channel(ChannelChat {
                id: 1,
                title: String::from("Channel"),
                username: Some(String::from("channel_username")),
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                linked_chat_id: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            author_signature: None,
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("text"),
                entities: None,
            }),
        })),
        linked_chat_id: Some(2),
        has_protected_content: Some(true),
        message_auto_delete_time: Some(86400),
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(
        expected_struct.get_username().unwrap(),
        String::from("channel_username")
    );
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "Channel",
            "username": "channel_username",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "description": "Description",
            "invite_link": "example.com/join/channel",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "linked_chat_id": 2,
            "has_protected_content": true,
            "message_auto_delete_time": 86400
        }),
    );

    let expected_struct = Chat::Channel(ChannelChat {
        id: 1,
        title: String::from("Channel"),
        username: None,
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        linked_chat_id: None,
        has_protected_content: None,
        message_auto_delete_time: None,
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "Channel"
        }),
    );
}

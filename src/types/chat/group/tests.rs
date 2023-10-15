use crate::types::{
    tests::assert_json_eq,
    Chat,
    ChatPermissions,
    ChatPhoto,
    GroupChat,
    Message,
    MessageData,
    MessageSender,
    Text,
    User,
};

#[test]
fn group_chat() {
    let expected_struct = Chat::Group(GroupChat {
        id: 1,
        title: String::from("Group"),
        photo: Some(ChatPhoto {
            small_file_id: String::from("small-file-id"),
            small_file_unique_id: String::from("small-file-unique-id"),
            big_file_id: String::from("big-file-id"),
            big_file_unique_id: String::from("big-file-unique-id"),
        }),
        invite_link: Some(String::from("example.com/join/group")),
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
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("Group"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
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
            data: MessageData::Text(Text {
                data: String::from("text"),
                entities: None,
            }),
        })),
        permissions: Some(ChatPermissions::allowed()),
        has_protected_content: Some(true),
        message_auto_delete_time: Some(86400),
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "invite_link": "example.com/join/group",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "permissions": {
                "can_send_messages": true,
                "can_send_media_messages": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
            },
            "has_protected_content": true,
            "message_auto_delete_time": 86400
        }),
    );
    assert_json_eq(
        Chat::Group(GroupChat {
            id: 1,
            title: String::from("Group"),
            photo: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            has_protected_content: None,
            message_auto_delete_time: None,
        }),
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
        }),
    );
}

use crate::types::{
    tests::assert_json_eq,
    ChannelChat,
    Chat,
    Forward,
    ForwardFrom,
    Message,
    MessageData,
    MessageSender,
    SupergroupChat,
    Text,
    User,
};

fn create_message_struct() -> Message {
    Message {
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
        chat: Chat::Supergroup(SupergroupChat {
            id: 1,
            title: String::from("Chat"),
            username: None,
            photo: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            linked_chat_id: None,
            location: None,
            has_protected_content: None,
            join_to_send_messages: None,
            join_by_request: None,
            is_forum: None,
            active_usernames: None,
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
            data: String::from("test"),
            entities: None,
        }),
    }
}

fn create_message_value() -> serde_json::Value {
    serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {
            "id": 1,
            "first_name": "User",
            "is_bot": false
        },
        "chat": {
            "id": 1,
            "type": "supergroup",
            "title": "Chat"
        },
        "text": "test",
        "has_protected_content": false,
        "is_automatic_forward": false
    })
}

#[test]
fn forward_from_user() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.forward = Some(Forward {
        from: ForwardFrom::User(User {
            id: 2,
            is_bot: false,
            first_name: String::from("firstname"),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
        }),
        date: 0,
    });
    expected_value["forward_from"] = serde_json::json!({
        "id": 2,
        "first_name": "firstname",
        "is_bot": false
    });
    expected_value["forward_date"] = serde_json::json!(0);

    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn deserialize_forward_from_hidden_user() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.forward = Some(Forward {
        date: 0,
        from: ForwardFrom::HiddenUser(String::from("Hidden User")),
    });
    expected_value["forward_sender_name"] = serde_json::json!("Hidden User");
    expected_value["forward_date"] = serde_json::json!(0);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn deserialize_forward_from_channel() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.forward = Some(Forward {
        date: 0,
        from: ForwardFrom::Channel {
            chat: ChannelChat {
                id: 1,
                title: String::from("test"),
                username: None,
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                linked_chat_id: None,
                has_protected_content: None,
                message_auto_delete_time: None,
                active_usernames: None,
            },
            message_id: 1,
            signature: Some(String::from("test")),
        },
    });
    expected_value["forward_from_chat"] = serde_json::json!({
        "id": 1,
        "title": "test"
    });
    expected_value["forward_from_message_id"] = serde_json::json!(1);
    expected_value["forward_signature"] = serde_json::json!("test");
    expected_value["forward_date"] = serde_json::json!(0);
    assert_json_eq(expected_struct, expected_value);
}

use crate::types::{
    tests::assert_json_eq,
    ChannelChat,
    Forward,
    ForwardFrom,
    Message,
    MessageData,
    SupergroupChat,
    Text,
    User,
};

fn create_message_struct() -> Message {
    Message::new(
        1,
        0,
        SupergroupChat::new(1, "Chat"),
        MessageData::Text(Text::from("test")),
        User::new(1, "User", false),
    )
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

    expected_struct.forward = Some(Forward::new(0, ForwardFrom::User(User::new(1, "User", false))));
    expected_value["forward_from"] = serde_json::json!({
        "id": 1,
        "first_name": "User",
        "is_bot": false
    });
    expected_value["forward_date"] = serde_json::json!(0);

    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn deserialize_forward_from_hidden_user() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.forward = Some(Forward::new(0, ForwardFrom::HiddenUser(String::from("Hidden User"))));
    expected_value["forward_sender_name"] = serde_json::json!("Hidden User");
    expected_value["forward_date"] = serde_json::json!(0);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn deserialize_forward_from_channel() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.forward = Some(Forward::new(
        0,
        ForwardFrom::Channel {
            chat: ChannelChat::new(1, "test"),
            message_id: 1,
            signature: Some(String::from("test")),
        },
    ));
    expected_value["forward_from_chat"] = serde_json::json!({
        "id": 1,
        "title": "test"
    });
    expected_value["forward_from_message_id"] = serde_json::json!(1);
    expected_value["forward_signature"] = serde_json::json!("test");
    expected_value["forward_date"] = serde_json::json!(0);
    assert_json_eq(expected_struct, expected_value);
}

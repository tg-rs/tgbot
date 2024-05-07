use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        BackgroundType,
        ChannelChat,
        Chat,
        ChatBackground,
        Document,
        GetChat,
        GroupChat,
        LeaveChat,
        PrivateChat,
        SetChatDescription,
        SetChatTitle,
        SupergroupChat,
    },
};

#[test]
fn channel_chat() {
    let expected_struct = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(&expected_struct.get_username().unwrap(), &"channel_username");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "Channel",
            "username": "channel_username",
        }),
    );

    let expected_struct = Chat::Channel(ChannelChat::new(1, "Channel"));
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

#[test]
fn group_chat() {
    let expected_struct = Chat::Group(GroupChat::new(1, "Group"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(&expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
        }),
    );
}

#[test]
fn private_chat() {
    let expected_struct = Chat::Private(
        PrivateChat::new(1, "John")
            .with_last_name("Doe")
            .with_username("john_doe"),
    );
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
        }),
    );
    let expected_struct = Chat::Private(PrivateChat::new(1, "John"));
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

#[test]
fn supergroup_chat() {
    let expected_struct = Chat::Supergroup(
        SupergroupChat::new(1, "Supergroup Chat")
            .with_is_forum(true)
            .with_username("supergroup_chat"),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "supergroup_chat");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat",
            "is_forum": true,
            "username": "supergroup_chat",
        }),
    );

    let expected_struct = Chat::Supergroup(SupergroupChat::new(1, "Supergroup Chat"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat"
        }),
    );
}

#[test]
fn chat_background() {
    assert_json_eq(
        ChatBackground::from(BackgroundType::Wallpaper {
            dark_theme_dimming: 100,
            document: Document::new("file-id", "file-unique-id"),
            is_blurred: Some(true),
            is_moving: Some(false),
        }),
        serde_json::json!({
            "type": {
                "type": "wallpaper",
                "dark_theme_dimming": 100,
                "document": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                },
                "is_blurred": true,
                "is_moving": false,
            }
        }),
    );
}

#[test]
fn get_chat() {
    assert_payload_eq(
        Payload::json(
            "getChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChat::new(1),
    );
}

#[test]
fn leave_chat() {
    assert_payload_eq(
        Payload::json(
            "leaveChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        LeaveChat::new(1),
    );
}

#[test]
fn set_chat_description() {
    let method = SetChatDescription::new(1);
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1,
                "description": "Description"
            }),
        ),
        method.with_description("Description"),
    );
}

#[test]
fn set_chat_title() {
    assert_payload_eq(
        Payload::json(
            "setChatTitle",
            serde_json::json!({
                "chat_id": 1,
                "title": "Chat"
            }),
        ),
        SetChatTitle::new(1, "Chat"),
    );
}

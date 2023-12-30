use crate::types::{
    tests::assert_json_eq,
    ChannelChat,
    GroupChat,
    MessageOrigin,
    MessageOriginChannel,
    MessageOriginChat,
    MessageOriginHiddenUser,
    MessageOriginUser,
    User,
};

#[test]
fn message_origin_channel() {
    assert_json_eq(
        MessageOrigin::from(MessageOriginChannel::new(ChannelChat::new(1, "test"), 0, 1)),
        serde_json::json!({
            "type": "channel",
            "chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            },
            "date": 0,
            "message_id": 1
        }),
    );
}

#[test]
fn message_origin_chat() {
    assert_json_eq(
        MessageOrigin::from(MessageOriginChat::new(0, GroupChat::new(1, "test"))),
        serde_json::json!({
            "type": "chat",
            "sender_chat": {
                "type": "group",
                "id": 1,
                "title": "test"
            },
            "date": 0
        }),
    );
}

#[test]
fn message_origin_hidden_user() {
    assert_json_eq(
        MessageOrigin::from(MessageOriginHiddenUser::new(0, "test")),
        serde_json::json!({
            "type": "hidden_user",
            "date": 0,
            "sender_user_name": "test"
        }),
    );
}

#[test]
fn message_origin_user() {
    assert_json_eq(
        MessageOrigin::from(MessageOriginUser::new(0, User::new(1, "test", false))),
        serde_json::json!({
            "type": "user",
            "date": 0,
            "sender_user": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );
}

use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        CloseForumTopic,
        CreateForumTopic,
        DeleteForumTopic,
        EditForumTopic,
        ForumTopic,
        ForumTopicClosed,
        ForumTopicCreated,
        ForumTopicReopened,
        GetForumTopicIconStickers,
        ReopenForumTopic,
        UnpinAllForumTopicMessages,
    },
};

#[test]
fn forum_topic() {
    assert_json_eq(
        ForumTopic {
            message_thread_id: 1,
            name: String::from("topic-name"),
            icon_color: 0,
            icon_custom_emoji_id: Some(String::from("emoji-id")),
        },
        serde_json::json!({
            "message_thread_id": 1,
            "name": "topic-name",
            "icon_color": 0,
            "icon_custom_emoji_id": "emoji-id",
        }),
    );
}

#[test]
fn forum_topic_closed() {
    assert_json_eq(ForumTopicClosed::default(), serde_json::json!({}));
}

#[test]
fn forum_topic_created() {
    assert_json_eq(
        ForumTopicCreated {
            name: String::from("topic-name"),
            icon_color: 0,
            icon_custom_emoji_id: Some(String::from("emoji-id")),
        },
        serde_json::json!({
            "name": "topic-name",
            "icon_color": 0,
            "icon_custom_emoji_id": "emoji-id"
        }),
    );
    assert_json_eq(
        ForumTopicCreated {
            name: String::from("topic-name"),
            icon_color: 0,
            icon_custom_emoji_id: None,
        },
        serde_json::json!({
            "name": "topic-name",
            "icon_color": 0,
        }),
    );
}

#[test]
fn forum_topic_reopened() {
    assert_json_eq(ForumTopicReopened::default(), serde_json::json!({}));
}

#[test]
fn close_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "closeForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        CloseForumTopic::new(1, 1),
    );
}

#[test]
fn create_forum_topic() {
    let method = CreateForumTopic::new(1, "topic-name");
    assert_payload_eq(
        Payload::json(
            "createForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "name": "topic-name"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "name": "topic-name",
                "icon_color": 0,
                "icon_custom_emoji_id": "emoji-id"
            }),
        ),
        method.icon_color(0).icon_custom_emoji_id("emoji-id"),
    );
}

#[test]
fn delete_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "deleteForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        DeleteForumTopic::new(1, 1),
    );
}

#[test]
fn edit_forum_topic() {
    let method = EditForumTopic::new(1, 1);
    assert_payload_eq(
        Payload::json(
            "editForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "editForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1,
                "name": "topic-name",
                "icon_custom_emoji_id": "emoji-id"
            }),
        ),
        method.name("topic-name").icon_custom_emoji_id("emoji-id"),
    );
}

#[test]
fn get_forum_topic_icon_stickers() {
    assert_payload_eq(Payload::empty("getForumTopicIconStickers"), GetForumTopicIconStickers);
}

#[test]
fn reopen_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "reopenForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        ReopenForumTopic::new(1, 1),
    );
}

#[test]
fn unpin_all_forum_topic_messages() {
    assert_payload_eq(
        Payload::json(
            "unpinAllForumTopicMessages",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        UnpinAllForumTopicMessages::new(1, 1),
    );
}

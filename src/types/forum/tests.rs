use crate::types::{tests::assert_json_eq, ForumTopicClosed, ForumTopicCreated, ForumTopicReopened};

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

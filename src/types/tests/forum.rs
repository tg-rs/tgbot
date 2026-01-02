use crate::types::*;

#[test]
fn forum_topic() {
    let expected_struct = ForumTopic::new(ForumTopicIconColor::Bittersweet, 1, "topic-name");
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_icon_custom_emoji_id("emoji-id")
            .with_is_name_implicit(true)
    );
}

#[test]
fn close_forum_topic() {
    assert_payload_eq!(POST JSON "closeForumTopic" => CloseForumTopic::new(1, 1));
}

#[test]
fn close_general_forum_topic() {
    assert_payload_eq!(POST JSON "closeGeneralForumTopic" => CloseGeneralForumTopic::new(1));
}

#[test]
fn create_forum_topic() {
    let method = CreateForumTopic::new(1, "topic-name");
    assert_payload_eq!(POST JSON "createForumTopic" => method.clone());
    let method = method
        .with_icon_color(ForumTopicIconColor::BrightLavender)
        .with_icon_custom_emoji_id("emoji-id");
    assert_payload_eq!(POST JSON "createForumTopic" => method);
}

#[test]
fn delete_forum_topic() {
    assert_payload_eq!(POST JSON "deleteForumTopic" => DeleteForumTopic::new(1, 1));
}

#[test]
fn edit_forum_topic() {
    let method = EditForumTopic::new(1, 1);
    assert_payload_eq!(POST JSON "editForumTopic" => method.clone());
    let method = method.with_icon_custom_emoji_id("emoji-id").with_name("topic-name");
    assert_payload_eq!(POST JSON "editForumTopic" => method);
}

#[test]
fn edit_general_forum_topic() {
    let method = EditGeneralForumTopic::new(1, "new-name");
    assert_payload_eq!(POST JSON "editGeneralForumTopic" => method);
}

#[test]
fn get_forum_topic_icon_stickers() {
    assert_payload_eq!(GET "getForumTopicIconStickers" => GetForumTopicIconStickers);
}

#[test]
fn hide_general_forum_topic() {
    assert_payload_eq!(POST JSON "hideGeneralForumTopic" => HideGeneralForumTopic::new(1));
}

#[test]
fn reopen_forum_topic() {
    assert_payload_eq!(POST JSON "reopenForumTopic" => ReopenForumTopic::new(1, 1));
}

#[test]
fn reopen_general_forum_topic() {
    let method = ReopenGeneralForumTopic::new(1);
    assert_payload_eq!(POST JSON "reopenGeneralForumTopic" => method);
}

#[test]
fn unhide_general_forum_topic() {
    let method = UnhideGeneralForumTopic::new(1);
    assert_payload_eq!(POST JSON "unhideGeneralForumTopic" => method);
}

#[test]
fn unpin_all_forum_topic_messages() {
    let method = UnpinAllForumTopicMessages::new(1, 1);
    assert_payload_eq!(POST JSON "unpinAllForumTopicMessages" => method);
}

#[test]
fn unpin_all_general_forum_topic_messages() {
    let method = UnpinAllGeneralForumTopicMessages::new(1);
    assert_payload_eq!(POST JSON "unpinAllGeneralForumTopicMessages" => method);
}

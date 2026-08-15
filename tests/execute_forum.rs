#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([("close-forum-topic", CloseForumTopic::new(1, 1), |x| assert!(x))])
        .await;

    cx.execute_batch([("close-general-forum-topic", CloseGeneralForumTopic::new(1), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([
        ("create-forum-topic-base", CreateForumTopic::new(1, "topic-name"), |x| {
            assert_eq!(x.name, "topic-name");
            assert_eq!(x.message_thread_id, 1);
            assert_eq!(x.icon_color, ForumTopicIconColor::BakerMillerPink);
            assert!(x.icon_custom_emoji_id.is_none());
            assert!(x.is_name_implicit.is_none());
        }),
        (
            "create-forum-topic-all",
            CreateForumTopic::new(1, "topic-name")
                .with_icon_color(ForumTopicIconColor::BrightLavender)
                .with_icon_custom_emoji_id("emoji-id"),
            |x| {
                assert_eq!(x.name, "topic-name");
                assert_eq!(x.message_thread_id, 1);
                assert_eq!(x.icon_color, ForumTopicIconColor::Unknown(0));
                assert_eq!(x.icon_custom_emoji_id.unwrap(), "test");
                assert!(x.is_name_implicit.unwrap());
            },
        ),
    ])
    .await;

    cx.execute_batch([("delete-forum-topic", DeleteForumTopic::new(1, 1), |x| assert!(x))])
        .await;

    cx.execute_batch([
        ("edit-forum-topic-base", EditForumTopic::new(1, 1), |x| assert!(x)),
        (
            "edit-forum-topic-all",
            EditForumTopic::new(1, 1)
                .with_icon_custom_emoji_id("emoji-id")
                .with_name("topic-name"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "edit-general-forum-topic",
        EditGeneralForumTopic::new(1, "new-name"),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([("get-forum-topic-icon-stickers", GetForumTopicIconStickers, |x| {
        assert!(x.is_empty())
    })])
    .await;

    cx.execute_batch([("hide-general-forum-topic", HideGeneralForumTopic::new(1), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([("reopen-forum-topic", ReopenForumTopic::new(1, 1), |x| assert!(x))])
        .await;

    cx.execute_batch([("reopen-general-forum-topic", ReopenGeneralForumTopic::new(1), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([("unhide-general-forum-topic", UnhideGeneralForumTopic::new(1), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([(
        "unpin-all-forum-topic-messages",
        UnpinAllForumTopicMessages::new(1, 1),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "unpin-all-general-forum-topic-messages",
        UnpinAllGeneralForumTopicMessages::new(1),
        |x| assert!(x),
    )])
    .await;
}

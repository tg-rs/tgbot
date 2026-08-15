#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "delete-all-message-reactions-base",
            DeleteAllMessageReactions::new(1),
            |x| assert!(x),
        ),
        (
            "delete-all-message-reactions-all",
            DeleteAllMessageReactions::new(1).with_actor_chat_id(3).with_user_id(4),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("delete-message-reaction-base", DeleteMessageReaction::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "delete-message-reaction-all",
            DeleteMessageReaction::new(1, 2).with_actor_chat_id(3).with_user_id(4),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("set-message-reaction-base", SetMessageReaction::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "set-message-reaction-all",
            SetMessageReaction::new(1, 2)
                .with_is_big(true)
                .with_reaction([ReactionType::emoji("🤡")]),
            |x| assert!(x),
        ),
        (
            "set-message-reaction-custom-emoji",
            SetMessageReaction::new(1, 2)
                .with_is_big(true)
                .with_reaction([ReactionType::custom_emoji("emoji-id")]),
            |x| assert!(x),
        ),
    ])
    .await;
}

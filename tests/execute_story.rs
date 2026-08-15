#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("repost-story-base", RepostStory::new(1, "test", 2, 3), |x| {
            assert_eq!(x.id, 1);
            assert!(matches!(x.chat, Chat::Private(_)));
        }),
        (
            "repost-story-all",
            RepostStory::new(1, "test", 2, 3)
                .with_post_to_chat_page(true)
                .with_protect_content(true),
            |x| {
                assert_eq!(x.id, 1);
                assert!(matches!(x.chat, Chat::Private(_)));
            },
        ),
    ])
    .await;
}

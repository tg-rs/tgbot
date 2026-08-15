#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([(
        "get-user-personal-chat-messages",
        GetUserPersonalChatMessages::new(1, 2),
        |x| {
            assert_eq!(x.len(), 1);
            let MessageData::Text(text) = &x[0].data else {
                panic!("Got an unexpected message data")
            };
            assert_eq!(text, "test");
        },
    )])
    .await;
    cx.execute_batch([
        ("get-user-profile-audios-base", GetUserProfileAudios::new(1), |x| {
            assert_eq!(x.audios.len(), 1);
            assert_eq!(x.total_count, 1);
        }),
        (
            "get-user-profile-audios-all",
            GetUserProfileAudios::new(1).with_offset(0).with_limit(10),
            |x| {
                assert_eq!(x.audios.len(), 1);
                assert_eq!(x.total_count, 1);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-user-profile-photos-base", GetUserProfilePhotos::new(1), |x| {
            assert_eq!(x.photos.len(), 1);
            assert_eq!(x.total_count, 1);
        }),
        (
            "get-user-profile-photos-all",
            GetUserProfilePhotos::new(1).with_offset(0).with_limit(10),
            |x| {
                assert_eq!(x.photos.len(), 1);
                assert_eq!(x.total_count, 1);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("set-user-emoji-status-base", SetUserEmojiStatus::new(1), |x| assert!(x)),
        (
            "set-user-emoji-status-all",
            SetUserEmojiStatus::new(1)
                .with_emoji_id("emoji-id")
                .with_expiration_date(1),
            |x| assert!(x),
        ),
    ])
    .await;
}

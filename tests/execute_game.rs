#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-game-base", SendGame::new(1, "Game"), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Game(data) = x.data {
                assert_eq!(data.title, "Title");
                assert_eq!(data.description, "Description");
                assert!(data.photo.is_empty());
                assert!(data.text.is_none());
                assert!(data.animation.is_none());
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        (
            "send-game-full",
            SendGame::new(1, "Game")
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_disable_notification(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "example.com")]])
                .with_reply_parameters(ReplyParameters::new(1)),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Game(data) = x.data {
                    assert_eq!(data.title, "Title");
                    assert_eq!(data.description, "Description");
                    assert_eq!(data.photo.len(), 1);
                    let photo = &data.photo[0];
                    assert_eq!(photo.file_id, "pfid");
                    let text = data.text.unwrap();
                    assert_eq!(text.data, "test");
                    let mut text_entities = text.entities.unwrap().into_iter();
                    let text_entity = text_entities.next().unwrap();
                    assert!(matches!(
                        text_entity,
                        TextEntity::Bold(TextEntityPosition { offset: 0, length: 2 })
                    ));
                    assert!(text_entities.next().is_none());
                    let animation = data.animation.unwrap();
                    assert_eq!(animation.file_id, "afid");
                    assert_eq!(animation.file_unique_id, "afuid");
                    assert_eq!(animation.height, 200);
                    assert_eq!(animation.width, 200);
                    assert_eq!(animation.duration, 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "get-game-high-scores-chat",
            GetGameHighScores::for_chat_message(1, 2, 3),
            |x| {
                assert_eq!(x.len(), 1);
                let score = &x[0];
                assert_eq!(score.position, 1);
                assert_eq!(score.user.id, 1);
                assert_eq!(score.score, 1000);
            },
        ),
        (
            "get-game-high-scores-inline",
            GetGameHighScores::for_inline_message(1, "inline-message-id"),
            |x| {
                assert_eq!(x.len(), 1);
                let score = &x[0];
                assert_eq!(score.position, 1);
                assert_eq!(score.user.id, 1);
                assert_eq!(score.score, 1000);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "set-game-score-chat-base",
            SetGameScore::for_chat_message(1, 2, 3, 100),
            |x| {
                assert!(matches!(x, EditMessageResult::Message(_)));
            },
        ),
        (
            "set-game-score-chat-all",
            SetGameScore::for_chat_message(1, 2, 3, 100)
                .with_disable_edit_message(true)
                .with_force(true),
            |x| {
                assert!(matches!(x, EditMessageResult::Message(_)));
            },
        ),
        (
            "set-game-score-inline-base",
            SetGameScore::for_inline_message("inline-message-id", 3, 100),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "set-game-score-inline-all",
            SetGameScore::for_inline_message("inline-message-id", 3, 100)
                .with_disable_edit_message(true)
                .with_force(true),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;
}

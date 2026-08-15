#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-quiz-base", SendQuiz::new(1, "Q", [0], ["X"]), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Poll(data) = x.data {
                let Poll::Quiz(data) = data else {
                    panic!("Expects quiz, got a regular poll");
                };
                assert_eq!(data.id, "poll-id");
                assert_eq!(data.total_voter_count, 1);
                assert!(data.is_closed);
                assert!(data.is_anonymous);
                assert!(!data.allows_revoting);
                assert!(!data.members_only);
                assert_eq!(data.correct_option_ids.unwrap(), [0]);
            } else {
                panic!("Got an unexpected message data");
            }
        }),
        (
            "send-quiz-question-entities",
            SendQuiz::new(1, ("Q", [TextEntity::bold(0..1)]), [0], ["X"]),
            |x| {
                if let MessageData::Poll(data) = x.data {
                    let Poll::Quiz(data) = data else {
                        panic!("Expects quiz, got a regular poll");
                    };
                    assert_eq!(data.id, "poll-id");
                    assert_eq!(data.total_voter_count, 1);
                    assert!(data.is_closed);
                    assert!(data.is_anonymous);
                    assert!(!data.allows_revoting);
                    assert!(!data.members_only);
                    assert_eq!(data.correct_option_ids.unwrap(), [0]);
                    assert_eq!(data.question.data, "Q");
                    let mut entitites = data.question.entities.unwrap().into_iter();
                    assert!(entitites.next().is_some());
                    assert!(entitites.next().is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-quiz-question-parse-mode",
            SendQuiz::new(
                1,
                InputText::from("Q")
                    .with_format([TextEntity::bold(0..1)])
                    .with_format(ParseMode::MarkdownV2),
                [0],
                ["X"],
            ),
            |x| {
                if let MessageData::Poll(data) = x.data {
                    let Poll::Quiz(data) = data else {
                        panic!("Expects quiz, got a regular poll");
                    };
                    assert_eq!(data.id, "poll-id");
                    assert_eq!(data.total_voter_count, 1);
                    assert!(data.is_closed);
                    assert!(data.is_anonymous);
                    assert!(!data.allows_revoting);
                    assert!(!data.members_only);
                    assert_eq!(data.correct_option_ids.unwrap(), [0]);
                    assert_eq!(data.question.data, "Q");
                    let mut entitites = data.question.entities.unwrap().into_iter();
                    assert!(entitites.next().is_some());
                    assert!(entitites.next().is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-quiz-full",
            SendQuiz::new(1, "Q", [0], ["O1", "O2"])
                .with_allow_paid_broadcast(true)
                .with_allows_multiple_answers(true)
                .with_allows_revoting(false)
                .with_business_connection_id("id")
                .with_country_codes(["NL"])
                .with_description(("test", ParseMode::MarkdownV2))
                .with_disable_notification(true)
                .with_explanation("test")
                .with_explanation_media(InputMediaLocation::new(1.0, 2.0))
                .with_hide_results_until_closes(true)
                .with_is_anonymous(false)
                .with_is_closed(false)
                .with_members_only(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_open_period(40)
                .with_close_date(20)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_shuffle_options(true),
            |x| {
                if let MessageData::Poll(data) = x.data {
                    let Poll::Quiz(data) = data else {
                        panic!("Expects quiz, got a regular poll");
                    };
                    assert_eq!(data.id, "poll-id");
                    assert_eq!(data.total_voter_count, 1);
                    assert!(data.is_closed);
                    assert!(data.is_anonymous);
                    assert!(!data.allows_revoting);
                    assert!(!data.members_only);
                    assert_eq!(data.correct_option_ids.unwrap(), [0]);
                    assert_eq!(data.question.data, "Q");
                    let mut entitites = data.question.entities.unwrap().into_iter();
                    assert!(entitites.next().is_some());
                    assert!(entitites.next().is_none());
                    assert!(matches!(data.explanation_media.unwrap(), PollMedia::Location(_)));
                    assert_eq!(data.explanation.unwrap().data, "test");
                    assert_eq!(data.description.unwrap().data, "test");
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
    ])
    .await;
}

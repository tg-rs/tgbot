#![allow(missing_docs)]
use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-poll-base", SendPoll::new(1, "Q", ["X"]), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Poll(data) = x.data {
                let Poll::Regular(data) = data else {
                    panic!("Expects a regular poll, got a quiz");
                };
                assert_eq!(data.id, "poll-id");
                assert_eq!(data.question.data, "question");
                assert_eq!(data.total_voter_count, 1);
                assert!(data.is_closed);
                assert!(data.is_anonymous);
                assert!(!data.allows_multiple_answers);
                assert!(!data.allows_revoting);
                assert!(!data.members_only);
                assert_eq!(data.options.len(), 2);
                let option_1 = &data.options[0];
                assert_eq!(option_1.persistent_id, "opt-1");
                assert_eq!(option_1.text.data, "test 1");
                assert_eq!(option_1.voter_count, 1);
                let option_2 = &data.options[1];
                assert_eq!(option_2.persistent_id, "opt-2");
                assert_eq!(option_2.text.data, "test 2");
                assert_eq!(option_2.voter_count, 0);
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        (
            "send-poll-question-entities",
            SendPoll::new(1, ("Q", [TextEntity::bold(0..1)]), ["X"]),
            |x| {
                assert_eq!(x.id, 1);
                assert!(matches!(x.data, MessageData::Poll(_)));
            },
        ),
        (
            "send-poll-question-parse-mode",
            SendPoll::new(
                1,
                InputText::from("Q")
                    .with_format([TextEntity::bold(0..1)])
                    .with_format(ParseMode::MarkdownV2),
                ["X"],
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert!(matches!(x.data, MessageData::Poll(_)));
            },
        ),
        (
            "send-poll-full",
            SendPoll::new(1, "Q", ["X"])
                .with_allow_adding_options(true)
                .with_allow_paid_broadcast(true)
                .with_allows_multiple_answers(true)
                .with_allows_revoting(true)
                .with_business_connection_id("id")
                .with_country_codes(["US"])
                .with_open_period(30)
                .with_close_date(10)
                .with_description(("test", [TextEntity::bold(0..2)]))
                .with_disable_notification(true)
                .with_hide_results_until_closes(true)
                .with_is_anonymous(true)
                .with_allow_adding_options(true)
                .with_is_anonymous(false)
                .with_is_closed(false)
                .with_media(InputMediaLocation::new(1.0, 2.0))
                .with_members_only(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_open_period(20)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(
                    ReplyParameters::new(1).with_quote(ReplyQuote::new(0, ("test", [TextEntity::bold(0..2)]))),
                )
                .with_shuffle_options(true),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Poll(data) = x.data {
                    let Poll::Regular(data) = data else {
                        panic!("Expects a regular poll, got a quiz");
                    };
                    assert_eq!(data.id, "poll-id");
                    assert_eq!(data.question.data, "question");
                    assert_eq!(data.total_voter_count, 1);
                    assert!(data.is_closed);
                    assert!(data.is_anonymous);
                    assert!(!data.allows_multiple_answers);
                    assert!(!data.allows_revoting);
                    assert!(!data.members_only);
                    assert_eq!(data.options.len(), 2);
                    let option_1 = &data.options[0];
                    assert_eq!(option_1.persistent_id, "opt-1");
                    assert_eq!(option_1.text.data, "test 1");
                    assert_eq!(option_1.voter_count, 1);
                    let option_2 = &data.options[1];
                    assert_eq!(option_2.persistent_id, "opt-2");
                    assert_eq!(option_2.text.data, "test 2");
                    assert_eq!(option_2.voter_count, 0);
                    assert_eq!(data.country_codes.unwrap(), ["US"]);
                    assert_eq!(data.open_period.unwrap(), 3600);
                    let description = data.description.unwrap();
                    assert_eq!(description.data, "description");
                    let mut description_entities = description.entities.unwrap().into_iter();
                    assert!(description_entities.next().is_some());
                    assert!(description_entities.next().is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-poll-media",
            SendPoll::new(
                1,
                "Q",
                [
                    InputPollOption::new("X1")
                        .with_media(InputMediaAudio::from(Cursor::new("audio-data")).with_caption("Audio")),
                    InputPollOption::new("X2").with_media(InputMediaLocation::new(1.0, 2.0)),
                    InputPollOption::new("X3").with_media(InputMedia::link("https://example.com")),
                ],
            )
            .with_media(InputMediaLocation::new(1.0, 2.0)),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Poll(data) = x.data {
                    let Poll::Regular(data) = data else {
                        panic!("Expects a regular poll, got a quiz");
                    };
                    assert_eq!(data.options.len(), 3);
                    assert!(matches!(&data.options[0].media.as_ref().unwrap(), PollMedia::Audio(_)));
                    assert!(matches!(
                        &data.options[1].media.as_ref().unwrap(),
                        PollMedia::Location(_)
                    ));
                    assert!(matches!(&data.options[2].media.as_ref().unwrap(), PollMedia::Link(_)));
                    assert!(matches!(data.media.unwrap(), PollMedia::Location(_)));
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("stop-poll-base", StopPoll::new(1, 2), |x| {
            let Poll::Regular(poll) = x else {
                panic!("Got an unexpected poll")
            };
            assert_eq!(poll.id, "poll-id");
        }),
        (
            "stop-poll-all",
            StopPoll::new(1, 2)
                .with_business_connection_id("c-id")
                .with_reply_markup(
                    InlineKeyboardMarkup::from([[
                        InlineKeyboardButton::for_url("text", "url"),
                        InlineKeyboardButton::disabled("disabled"),
                    ]])
                    .with_force_reply(true),
                ),
            |x| {
                let Poll::Regular(poll) = x else {
                    panic!("Got an unexpected poll")
                };
                assert_eq!(poll.id, "poll-id");
            },
        ),
    ])
    .await;
}

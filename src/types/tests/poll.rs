use std::io::Cursor;

use crate::types::*;

#[test]
fn poll() {
    insta::assert_json_snapshot!(Poll::from(
        RegularPoll::new("poll-id", "Rust?")
            .with_is_anonymous(true)
            .with_is_closed(true)
            .with_country_codes(["RU", "GE"])
            .with_description("test")
            .with_media(PollMedia::Location(Location::new(1.0, 2.0)))
            .with_members_only(true)
            .with_options([PollOption::new("1", "Yes", 1000), PollOption::new("2", "No", 0)])
            .with_total_voter_count(1000),
    ));
}

#[test]
fn quiz() {
    insta::assert_json_snapshot!(Poll::from(
        Quiz::new("poll-id", "Rust?")
            .with_correct_option_ids([0])
            .with_country_codes(["CZ", "SK"])
            .with_explanation(Text::from("text").with_entities(vec![TextEntity::bold(0..2)].into_iter().collect()))
            .with_explanation_media(PollMedia::Location(Location::new(1.0, 2.0)))
            .with_is_anonymous(true)
            .with_is_closed(true)
            .with_description(Text::from("test").with_entities(vec![TextEntity::bold(0..2)].into_iter().collect()))
            .with_members_only(true)
            .with_options([PollOption::new("1", "Yes", 1000), PollOption::new("2", "No", 0)])
            .with_total_voter_count(100),
    ));
}

#[test]
fn poll_answer() {
    insta::assert_json_snapshot!(PollAnswer::new([0], "poll-id", User::new(1, "User", false)));
}

#[test]
fn poll_answer_voter() {
    insta::assert_json_snapshot!(PollAnswerVoter::from(User::new(1, "User", false)));
    insta::assert_json_snapshot!(PollAnswerVoter::from(Chat::Channel(
        ChannelChat::new(1, "test-channel").with_username("test_channel"),
    )));
}

#[test]
fn poll_media() {
    let data = vec![
        PollMedia::Animation(Animation::new(1, "id", "uid", 200, 400)),
        PollMedia::Audio(Audio::new(1, "id", "uid")),
        PollMedia::Document(Document::new("id", "uid")),
        PollMedia::LivePhoto(LivePhoto::new(1, "id", "uid", 200, 400)),
        PollMedia::Location(Location::new(1.0, 2.0)),
        PollMedia::Photo(vec![PhotoSize::new("id", "uid", 200, 200)]),
        PollMedia::Sticker(Sticker::new("id", "uid", StickerType::Regular, 512, 512)),
        PollMedia::Venue(Venue::new("test", "addr", Location::new(1.0, 2.0))),
        PollMedia::Video(Video::new(1, "id", "uid", 200, 400)),
    ];
    insta::assert_json_snapshot!(data);
}

#[test]
fn send_quiz() {
    let method = SendQuiz::new(1, "Q", [0], ["X"]).unwrap();
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendQuiz::new(1, "Q", [0], ["X"])
        .unwrap()
        .with_question_entities([TextEntity::bold(0..1)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendQuiz::new(1, "Q", [0], ["X"])
        .unwrap()
        .with_question_entities([TextEntity::bold(0..1)])
        .unwrap()
        .with_question_parse_mode(ParseMode::MarkdownV2);
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendQuiz::new(1, "Q", [0], ["O1", "O2"])
        .unwrap()
        .with_allow_adding_options(true)
        .with_allow_paid_broadcast(true)
        .with_allows_multiple_answers(true)
        .with_allows_revoting(false)
        .with_business_connection_id("id")
        .with_country_codes(["NL"])
        .unwrap()
        .with_description("test")
        .with_description_parse_mode(ParseMode::MarkdownV2)
        .with_disable_notification(true)
        .with_explanation("test")
        .with_explanation_media(InputMedia::for_location(InputMediaLocation::new(1.0, 2.0)))
        .unwrap()
        .with_hide_results_until_closes(true)
        .with_is_anonymous(false)
        .with_is_closed(false)
        .with_members_only(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_shuffle_options(true);
    assert_payload_eq!(POST FORM "sendPoll" => method);
}

#[test]
fn send_poll() {
    let method = SendPoll::new(1, "Q", ["X"]).unwrap();
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendPoll::new(1, "Q", ["X"])
        .unwrap()
        .with_question_entities([TextEntity::bold(0..1)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendPoll::new(1, "Q", ["X"])
        .unwrap()
        .with_question_entities([TextEntity::bold(0..1)])
        .unwrap()
        .with_question_parse_mode(ParseMode::MarkdownV2);
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendPoll::new(1, "Q", ["X"])
        .unwrap()
        .with_allow_adding_options(true)
        .with_allow_paid_broadcast(true)
        .with_allows_multiple_answers(true)
        .with_allows_revoting(true)
        .with_business_connection_id("id")
        .with_country_codes(["US"])
        .unwrap()
        .with_description("test")
        .with_description_entities([TextEntity::bold(0..2)])
        .unwrap()
        .with_disable_notification(true)
        .with_hide_results_until_closes(true)
        .with_is_anonymous(false)
        .with_is_closed(false)
        .with_media(InputMedia::for_location(InputMediaLocation::new(1.0, 2.0)))
        .unwrap()
        .with_members_only(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_shuffle_options(true);
    assert_payload_eq!(POST FORM "sendPoll" => method);
    let method = SendPoll::new(
        1,
        "Q",
        [
            InputPollOption::new("X1").with_media(InputMedia::for_audio(
                Cursor::new("audio-data"),
                InputMediaAudio::default().with_caption("Audio"),
            )),
            InputPollOption::new("X2").with_media(InputMedia::for_location(InputMediaLocation::new(1.0, 2.0))),
        ],
    )
    .unwrap();
    assert_payload_eq!(POST FORM "sendPoll" => method);
}

#[test]
fn stop_poll() {
    let method = StopPoll::new(1, 2)
        .with_business_connection_id("c-id")
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]);
    assert_payload_eq!(POST JSON "stopPoll" => method);
}

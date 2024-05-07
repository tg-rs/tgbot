use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        ChannelChat,
        Chat,
        ForceReply,
        InlineKeyboardButton,
        ParseMode,
        Poll,
        PollAnswer,
        PollAnswerVoter,
        PollOption,
        Quiz,
        RegularPoll,
        ReplyParameters,
        SendPoll,
        SendQuiz,
        StopPoll,
        Text,
        TextEntity,
        User,
    },
};

#[test]
fn poll() {
    assert_json_eq(
        Poll::from(
            RegularPoll::new("poll-id", "Rust?")
                .with_is_anonymous(true)
                .with_is_closed(true)
                .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                .with_total_voter_count(1000),
        ),
        serde_json::json!({
            "id": "poll-id",
            "question": "Rust?",
            "options": [
                {"text": "Yes", "voter_count": 1000},
                {"text": "No", "voter_count": 0}
            ],
            "is_closed": true,
            "total_voter_count": 1000,
            "is_anonymous": true,
            "type": "regular",
            "allows_multiple_answers": false
        }),
    );
}

#[test]
fn quiz() {
    assert_json_eq(
        Poll::from(
            Quiz::new("poll-id", "Rust?")
                .with_correct_option_id(0)
                .with_explanation(Text::from("text").with_entities(vec![TextEntity::bold(0..2)].into_iter().collect()))
                .with_is_anonymous(true)
                .with_is_closed(true)
                .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                .with_total_voter_count(100),
        ),
        serde_json::json!({
            "id": "poll-id",
            "question": "Rust?",
            "options": [
                {"text": "Yes", "voter_count": 1000},
                {"text": "No", "voter_count": 0}
            ],
            "explanation": "text",
            "explanation_entities": [{
                "type": "bold",
                "offset": 0,
                "length": 2
            }],
            "is_closed": true,
            "total_voter_count": 100,
            "is_anonymous": true,
            "type": "quiz",
            "correct_option_id": 0
        }),
    );
}

#[test]
fn poll_answer() {
    assert_json_eq(
        PollAnswer::new([0], "poll-id", User::new(1, "User", false)),
        serde_json::json!({
            "poll_id": "poll-id",
            "user": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            },
            "option_ids": [0],
        }),
    );
}

#[test]
fn poll_answer_voter() {
    assert_json_eq(
        PollAnswerVoter::from(User::new(1, "User", false)),
        serde_json::json!({
            "user": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            }
        }),
    );
    assert_json_eq(
        PollAnswerVoter::from(Chat::Channel(
            ChannelChat::new(1, "test-channel").with_username("test_channel"),
        )),
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "test-channel",
                "username": "test_channel"
            }
        }),
    );
}

#[test]
fn send_quiz() {
    let method = SendQuiz::new(1, "Q", 0, ["X"]);
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": [{"text": "X"}],
                "correct_option_id": 0
            }),
        ),
        method.clone(),
    );

    let method = method.with_question_entities([TextEntity::bold(0..1)]);
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": [{"text": "X"}],
                "correct_option_id": 0,
                "question_entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 1
                    }
                ]
            }),
        ),
        method.clone(),
    );

    let method = method.with_question_parse_mode(ParseMode::MarkdownV2);
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": [{"text": "X"}],
                "correct_option_id": 0,
                "question_parse_mode": "MarkdownV2"
            }),
        ),
        method,
    );

    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": [{"text": "O1"}, {"text": "O2"}],
                "is_anonymous": false,
                "is_closed": false,
                "correct_option_id": 0,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_thread_id": 1,
                "reply_markup": {
                    "force_reply": true
                },
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        SendQuiz::new(1, "Q", 0, ["O1", "O2"])
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_is_anonymous(false)
            .with_is_closed(false)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    )
}

#[test]
fn send_poll() {
    let method = SendPoll::new(1, "Q", ["X"]);

    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": [{"text": "X"}]
            }),
        ),
        method.clone(),
    );

    let method = method.with_question_entities([TextEntity::bold(0..1)]);
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": [{"text": "X"}],
                "question_entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 1
                    }
                ]
            }),
        ),
        method.clone(),
    );

    let method = method.with_question_parse_mode(ParseMode::MarkdownV2);
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": [{"text": "X"}],
                "question_parse_mode": "MarkdownV2"
            }),
        ),
        method,
    );

    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": [{"text": "O1"}, {"text": "O2"}],
                "is_anonymous": false,
                "is_closed": false,
                "allows_multiple_answers": true,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_thread_id": 1,
                "reply_markup": {
                    "force_reply": true
                },
                "reply_parameters": {
                    "message_id": 1
                },
            }),
        ),
        SendPoll::new(1, "Q", ["O1", "O2"])
            .with_allows_multiple_answers(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_is_anonymous(false)
            .with_is_closed(false)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    );
}

#[test]
fn stop_poll() {
    assert_payload_eq(
        Payload::json(
            "stopPoll",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            }),
        ),
        StopPoll::new(1, 2).with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    );
}

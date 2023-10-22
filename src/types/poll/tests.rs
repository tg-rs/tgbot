use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        ChannelChat,
        Chat,
        ForceReply,
        InlineKeyboardButton,
        Poll,
        PollAnswer,
        PollAnswerVoter,
        PollOption,
        Quiz,
        RegularPoll,
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
        Poll::Regular(RegularPoll {
            id: String::from("poll-id"),
            question: String::from("Rust?"),
            options: vec![
                PollOption {
                    text: String::from("Yes"),
                    voter_count: 1000,
                },
                PollOption {
                    text: String::from("No"),
                    voter_count: 0,
                },
            ],
            total_voter_count: 1000,
            is_closed: true,
            is_anonymous: true,
            allows_multiple_answers: false,
            open_period: None,
            close_date: None,
        }),
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
        Poll::Quiz(Quiz {
            id: String::from("poll-id"),
            question: String::from("Rust?"),
            options: vec![
                PollOption {
                    text: String::from("Yes"),
                    voter_count: 1000,
                },
                PollOption {
                    text: String::from("No"),
                    voter_count: 0,
                },
            ],
            total_voter_count: 100,
            is_closed: true,
            is_anonymous: true,
            correct_option_id: 0,
            explanation: Some(Text {
                data: String::from("text"),
                entities: Some(vec![TextEntity::bold(0..2)].into_iter().collect()),
            }),
            open_period: None,
            close_date: None,
        }),
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
        PollAnswer {
            poll_id: String::from("poll-id"),
            voter: PollAnswerVoter::User(User {
                id: 1,
                first_name: String::from("Jamie"),
                last_name: None,
                username: None,
                is_bot: false,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            option_ids: vec![0],
        },
        serde_json::json!({
            "poll_id": "poll-id",
            "user": {
                "id": 1,
                "first_name": "Jamie",
                "is_bot": false
            },
            "option_ids": [0],
        }),
    );
}

#[test]
fn poll_answer_voter() {
    assert_json_eq(
        PollAnswerVoter::User(User {
            id: 1,
            first_name: String::from("Jamie"),
            last_name: None,
            username: None,
            is_bot: false,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
        }),
        serde_json::json!({
            "user": {
                "id": 1,
                "first_name": "Jamie",
                "is_bot": false
            }
        }),
    );
    assert_json_eq(
        PollAnswerVoter::Chat(Chat::Channel(ChannelChat {
            id: 1,
            title: String::from("test-channel"),
            username: Some(String::from("test_channel")),
            photo: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            linked_chat_id: None,
            has_protected_content: None,
            message_auto_delete_time: None,
            active_usernames: None,
        })),
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
    let method = SendQuiz::new(1, "Q");
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": [],
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "quiz",
                "options": ["O1", "O2"],
                "is_anonymous": false,
                "is_closed": false,
                "correct_option_id": 0,
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                },
                "message_thread_id": 1,
            }),
        ),
        method
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .correct_option_id(0)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .message_thread_id(1),
    )
}

#[test]
fn send_poll() {
    let method = SendPoll::new(1, "Q");
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": []
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendPoll",
            serde_json::json!({
                "chat_id": 1,
                "question": "Q",
                "type": "regular",
                "options": ["O1", "O2"],
                "is_anonymous": false,
                "is_closed": false,
                "allows_multiple_answers": true,
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                },
                "message_thread_id": 1
            }),
        ),
        method
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .allows_multiple_answers(true)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .message_thread_id(1),
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
        StopPoll::new(1, 2).reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]]),
    );
}

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{
        ForceReply,
        InlineKeyboardButton,
        Poll,
        PollAnswer,
        SendPoll,
        SendQuiz,
        StopPoll,
        TextEntities,
        TextEntity,
    },
};

#[test]
fn deserialize_poll() {
    let data: Poll = serde_json::from_value(serde_json::json!({
        "id": "poll-id",
        "question": "Rust?",
        "options": [
            {"text": "Yes", "voter_count": 1000},
            {"text": "No", "voter_count": 0}
        ],
        "is_closed": true,
        "total_voter_count": 100,
        "is_anonymous": true,
        "type": "regular",
        "allows_multiple_answers": false
    }))
    .unwrap();
    if let Poll::Regular(data) = data {
        assert_eq!(data.id, "poll-id");
        assert_eq!(data.question, "Rust?");
        assert_eq!(data.options.len(), 2);
        let yes = &data.options[0];
        assert_eq!(yes.text, "Yes");
        assert_eq!(yes.voter_count, 1000);
        let no = &data.options[1];
        assert_eq!(no.text, "No");
        assert_eq!(no.voter_count, 0);
        assert!(data.is_closed);
        assert_eq!(data.total_voter_count, 100);
        assert!(data.is_anonymous);
        assert!(!data.allows_multiple_answers);
    } else {
        panic!("Unexpected poll kind")
    }
}

#[test]
fn deserialize_quiz() {
    let data: Poll = serde_json::from_value(serde_json::json!({
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
    }))
    .unwrap();
    if let Poll::Quiz(data) = data {
        assert_eq!(data.id, "poll-id");
        assert_eq!(data.question, "Rust?");
        assert_eq!(data.options.len(), 2);
        let yes = &data.options[0];
        assert_eq!(yes.text, "Yes");
        assert_eq!(yes.voter_count, 1000);
        let no = &data.options[1];
        assert_eq!(no.text, "No");
        assert_eq!(no.voter_count, 0);
        let explanation = data.explanation.unwrap();
        assert_eq!(explanation.data, "text");
        assert_eq!(
            explanation.entities.unwrap(),
            TextEntities::from_iter(vec![TextEntity::bold(0..2)])
        );

        assert!(data.is_closed);
        assert_eq!(data.total_voter_count, 100);
        assert!(data.is_anonymous);
        assert_eq!(data.correct_option_id, 0);
    } else {
        panic!("Unexpected poll kind")
    }
}

#[test]
fn deserialize_poll_answer() {
    let data: PollAnswer = serde_json::from_value(serde_json::json!({
        "poll_id": "poll-id",
        "user": {
            "id": 1,
            "first_name": "Jamie",
            "is_bot": false
        },
        "option_ids": [0],
    }))
    .unwrap();
    assert_eq!(data.poll_id, "poll-id");
    assert_eq!(data.user.id, 1);
    assert_eq!(data.option_ids, vec![0]);
}

#[test]
fn send_quiz() {
    let request = SendQuiz::new(1, "Q")
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
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
    match request.into_body() {
        RequestBody::Json(data) => {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
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
                    }
                })
            );
        }
        data => panic!("Unexpected request data: {:?}", data),
    }
}

#[test]
fn send_poll() {
    let request = SendPoll::new(1, "Q")
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
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
    match request.into_body() {
        RequestBody::Json(data) => {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();

            assert_eq!(
                data,
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
                    }
                })
            );
        }
        data => panic!("Unexpected request data: {:?}", data),
    }
}

#[test]
fn stop_poll() {
    let request = StopPoll::new(1, 2)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/stopPoll");
    match request.into_body() {
        RequestBody::Json(data) => {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        }
        data => panic!("Unexpected request data: {:?}", data),
    }
}

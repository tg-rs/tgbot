use serde::{Deserialize, Deserializer, Serialize};

use crate::types::{primitive::Integer, text::Text, user::User, TextEntities};

/// Contains information about a poll
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Poll {
    /// A regular poll
    Regular(RegularPoll),
    /// A quiz
    Quiz(Quiz),
}

/// Kind of a poll
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollKind {
    /// A quiz
    Quiz,
    /// A regular poll
    Regular,
}

/// A regular poll
#[derive(Clone, Debug, Deserialize)]
pub struct RegularPoll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: Integer,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<Integer>,
}

/// A quiz
#[derive(Clone, Debug, Deserialize)]
pub struct Quiz {
    /// Unique quiz identifier
    pub id: String,
    /// Quiz question, 1-255 characters
    pub question: String,
    /// List of quiz options
    pub options: Vec<PollOption>,
    /// Total number of users that answered to the quiz
    pub total_voter_count: Integer,
    /// True, if the quiz is closed
    pub is_closed: bool,
    /// True, if the quiz is anonymous
    pub is_anonymous: bool,
    /// 0-based identifier of the correct answer option
    ///
    /// Available only for a closed quiz,
    /// or was sent (not forwarded) by the bot or
    /// to the private chat with the bot
    pub correct_option_id: Integer,
    /// Text that is shown when a user chooses an incorrect answer or
    /// taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(deserialize_with = "deserialize_explanation")]
    #[serde(flatten)]
    pub explanation: Option<Text>,
    /// Amount of time in seconds the quiz will be active after creation
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the quiz will be automatically closed
    pub close_date: Option<Integer>,
}

fn deserialize_explanation<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        explanation: String,
        explanation_entities: Option<TextEntities>,
    }

    Option::<Wrapper>::deserialize(deserializer).map(|wrapper| {
        wrapper.map(
            |Wrapper {
                 explanation: data,
                 explanation_entities: entities,
             }| Text { data, entities },
        )
    })
}

/// Contains information about one answer option in a poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: Integer,
}

/// An answer of a user in a non-anonymous poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

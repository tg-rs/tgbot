use crate::types::{
    primitive::Integer,
    text::{RawTextEntity, Text, TextEntityError},
    user::User,
};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize};
use std::{error::Error, fmt};
use vec1::Vec1;

/// Contains information about a poll
#[derive(Clone, Debug)]
pub enum Poll {
    /// A regular poll
    Regular(RegularPoll),
    /// A quiz
    Quiz(Quiz),
}

impl Poll {
    fn from_raw(raw: RawPoll) -> Result<Self, PollError> {
        match raw.kind {
            PollKind::Regular => Ok(Poll::Regular(RegularPoll::from_raw(raw))),
            PollKind::Quiz => Quiz::from_raw(raw).map(Poll::Quiz),
        }
    }
}

impl<'de> Deserialize<'de> for Poll {
    fn deserialize<D>(deserializer: D) -> Result<Poll, D::Error>
    where
        D: Deserializer<'de>,
    {
        Poll::from_raw(Deserialize::deserialize(deserializer)?).map_err(D::Error::custom)
    }
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
#[derive(Clone, Debug)]
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

impl RegularPoll {
    fn from_raw(raw: RawPoll) -> Self {
        Self {
            id: raw.id,
            question: raw.question,
            options: raw.options,
            total_voter_count: raw.total_voter_count,
            is_closed: raw.is_closed,
            is_anonymous: raw.is_anonymous,
            allows_multiple_answers: raw.allows_multiple_answers.unwrap_or(false),
            open_period: raw.open_period,
            close_date: raw.close_date,
        }
    }
}

/// A quiz
#[derive(Clone, Debug)]
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
    pub explanation: Option<Text>,
    /// Amount of time in seconds the quiz will be active after creation
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the quiz will be automatically closed
    pub close_date: Option<Integer>,
}

impl Quiz {
    fn from_raw(raw: RawPoll) -> Result<Self, PollError> {
        Ok(Self {
            id: raw.id,
            question: raw.question,
            options: raw.options,
            total_voter_count: raw.total_voter_count,
            is_closed: raw.is_closed,
            is_anonymous: raw.is_anonymous,
            correct_option_id: raw.correct_option_id.ok_or(PollError::CorrectOptionIdMissing)?,
            explanation: if let Some(text) = raw.explanation {
                Some(Text::from_raw(text, raw.explanation_entities).map_err(PollError::ParseExplanation)?)
            } else {
                None
            },
            open_period: raw.open_period,
            close_date: raw.close_date,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RawPoll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: Integer,
    is_closed: bool,
    is_anonymous: bool,
    #[serde(rename = "type")]
    kind: PollKind,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<Integer>,
    explanation: Option<String>,
    explanation_entities: Option<Vec1<RawTextEntity>>,
    open_period: Option<Integer>,
    close_date: Option<Integer>,
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

#[derive(Debug)]
enum PollError {
    CorrectOptionIdMissing,
    ParseExplanation(TextEntityError),
}

impl fmt::Display for PollError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::PollError::*;
        match self {
            CorrectOptionIdMissing => write!(out, "correct option id is required for a quiz"),
            ParseExplanation(err) => write!(out, "could not to parse a quiz explanation: {}", err),
        }
    }
}

impl Error for PollError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::PollError::*;
        match self {
            CorrectOptionIdMissing => None,
            ParseExplanation(err) => Some(err),
        }
    }
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

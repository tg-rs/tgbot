use serde::Serialize;

use crate::types::{ChatId, Integer, ParseMode, PollKind, ReplyMarkup, TextEntities};

#[derive(Clone, Debug, Serialize)]
pub(super) struct PollParameters {
    pub(super) chat_id: ChatId,
    pub(super) question: String,
    pub(super) options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub(super) kind: Option<PollKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) explanation_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) close_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) reply_markup: Option<ReplyMarkup>,
}

impl PollParameters {
    pub(super) fn new(chat_id: ChatId, question: String, kind: PollKind) -> Self {
        Self {
            chat_id,
            question,
            options: Vec::new(),
            is_anonymous: None,
            kind: Some(kind),
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

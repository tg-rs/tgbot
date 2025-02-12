#![allow(unreachable_patterns)]
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

use crate::types::{
    Float,
    InlineKeyboardMarkup,
    InlineQueryResult,
    InputMessageContent,
    Integer,
    ParseMode,
    TextEntities,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RawInlineQueryResultType {
    Article,
    Audio,

    #[serde(rename = "audio")]
    CachedAudio,
    #[serde(rename = "document")]
    CachedDocument,
    #[serde(rename = "gif")]
    CachedGif,
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif,
    #[serde(rename = "photo")]
    CachedPhoto,
    #[serde(rename = "sticker")]
    CachedSticker,
    #[serde(rename = "video")]
    CachedVideo,
    #[serde(rename = "voice")]
    CachedVoice,

    Contact,
    Document,
    Game,
    Gif,
    Location,
    Mpeg4Gif,
    Photo,
    Venue,
    Video,
    Voice,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RawInlineQueryResult {
    #[serde(flatten)]
    pub(super) data: RawInlineQueryResultData,
    pub(super) id: String,
    #[serde(rename = "type")]
    pub(super) result_type: RawInlineQueryResultType,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct RawInlineQueryResultData {
    pub(super) address: Option<String>,
    pub(super) audio_duration: Option<Integer>,
    pub(super) audio_file_id: Option<String>,
    pub(super) audio_url: Option<String>,
    pub(super) caption: Option<String>,
    pub(super) caption_entities: Option<TextEntities>,
    pub(super) description: Option<String>,
    pub(super) document_file_id: Option<String>,
    pub(super) document_url: Option<String>,
    pub(super) first_name: Option<String>,
    pub(super) foursquare_id: Option<String>,
    pub(super) foursquare_type: Option<String>,
    pub(super) game_short_name: Option<String>,
    pub(super) gif_duration: Option<Integer>,
    pub(super) gif_file_id: Option<String>,
    pub(super) gif_height: Option<Integer>,
    pub(super) gif_url: Option<String>,
    pub(super) gif_width: Option<Integer>,
    pub(super) google_place_id: Option<String>,
    pub(super) google_place_type: Option<String>,
    pub(super) heading: Option<Integer>,
    pub(super) horizontal_accuracy: Option<Float>,
    pub(super) input_message_content: Option<InputMessageContent>,
    pub(super) last_name: Option<String>,
    pub(super) latitude: Option<Float>,
    pub(super) live_period: Option<Integer>,
    pub(super) longitude: Option<Float>,
    pub(super) mime_type: Option<String>,
    pub(super) mpeg4_duration: Option<Integer>,
    pub(super) mpeg4_file_id: Option<String>,
    pub(super) mpeg4_height: Option<Integer>,
    pub(super) mpeg4_url: Option<String>,
    pub(super) mpeg4_width: Option<Integer>,
    pub(super) parse_mode: Option<ParseMode>,
    pub(super) performer: Option<String>,
    pub(super) photo_file_id: Option<String>,
    pub(super) photo_height: Option<Integer>,
    pub(super) phone_number: Option<String>,
    pub(super) photo_url: Option<String>,
    pub(super) photo_width: Option<Integer>,
    pub(super) proximity_alert_radius: Option<Integer>,
    pub(super) reply_markup: Option<InlineKeyboardMarkup>,
    pub(super) show_caption_above_media: Option<bool>,
    pub(super) sticker_file_id: Option<String>,
    pub(super) thumbnail_height: Option<Integer>,
    pub(super) thumbnail_mime_type: Option<String>,
    pub(super) thumbnail_url: Option<String>,
    pub(super) thumbnail_width: Option<Integer>,
    pub(super) title: Option<String>,
    pub(super) url: Option<String>,
    pub(super) vcard: Option<String>,
    pub(super) video_duration: Option<Integer>,
    pub(super) video_file_id: Option<String>,
    pub(super) video_height: Option<Integer>,
    pub(super) video_url: Option<String>,
    pub(super) video_width: Option<Integer>,
    pub(super) voice_duration: Option<Integer>,
    pub(super) voice_file_id: Option<String>,
    pub(super) voice_url: Option<String>,
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResult {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(match value.result_type {
            RawInlineQueryResultType::Article => InlineQueryResult::Article(value.try_into()?),
            RawInlineQueryResultType::Audio | RawInlineQueryResultType::CachedAudio => {
                if value.data.audio_file_id.is_some() {
                    InlineQueryResult::CachedAudio(value.try_into()?)
                } else {
                    InlineQueryResult::Audio(value.try_into()?)
                }
            }
            RawInlineQueryResultType::CachedSticker => InlineQueryResult::CachedSticker(value.try_into()?),
            RawInlineQueryResultType::Contact => InlineQueryResult::Contact(value.try_into()?),
            RawInlineQueryResultType::Document | RawInlineQueryResultType::CachedDocument => {
                if value.data.document_file_id.is_some() {
                    InlineQueryResult::CachedDocument(value.try_into()?)
                } else {
                    InlineQueryResult::Document(value.try_into()?)
                }
            }
            RawInlineQueryResultType::Game => InlineQueryResult::Game(value.try_into()?),
            RawInlineQueryResultType::Gif | RawInlineQueryResultType::CachedGif => {
                if value.data.gif_file_id.is_some() {
                    InlineQueryResult::CachedGif(value.try_into()?)
                } else {
                    InlineQueryResult::Gif(value.try_into()?)
                }
            }
            RawInlineQueryResultType::Location => InlineQueryResult::Location(value.try_into()?),
            RawInlineQueryResultType::Mpeg4Gif | RawInlineQueryResultType::CachedMpeg4Gif => {
                if value.data.mpeg4_file_id.is_some() {
                    InlineQueryResult::CachedMpeg4Gif(value.try_into()?)
                } else {
                    InlineQueryResult::Mpeg4Gif(value.try_into()?)
                }
            }
            RawInlineQueryResultType::Photo | RawInlineQueryResultType::CachedPhoto => {
                if value.data.photo_file_id.is_some() {
                    InlineQueryResult::CachedPhoto(value.try_into()?)
                } else {
                    InlineQueryResult::Photo(value.try_into()?)
                }
            }
            RawInlineQueryResultType::Venue => InlineQueryResult::Venue(value.try_into()?),
            RawInlineQueryResultType::Video | RawInlineQueryResultType::CachedVideo => {
                if value.data.video_file_id.is_some() {
                    InlineQueryResult::CachedVideo(value.try_into()?)
                } else {
                    InlineQueryResult::Video(value.try_into()?)
                }
            }
            RawInlineQueryResultType::Voice | RawInlineQueryResultType::CachedVoice => {
                if value.data.voice_file_id.is_some() {
                    InlineQueryResult::CachedVoice(value.try_into()?)
                } else {
                    InlineQueryResult::Voice(value.try_into()?)
                }
            }
        })
    }
}

impl From<InlineQueryResult> for RawInlineQueryResult {
    fn from(value: InlineQueryResult) -> Self {
        match value {
            InlineQueryResult::Article(value) => value.into(),
            InlineQueryResult::Audio(value) => value.into(),
            InlineQueryResult::CachedAudio(value) => value.into(),
            InlineQueryResult::CachedDocument(value) => value.into(),
            InlineQueryResult::CachedGif(value) => value.into(),
            InlineQueryResult::CachedMpeg4Gif(value) => value.into(),
            InlineQueryResult::CachedPhoto(value) => value.into(),
            InlineQueryResult::CachedSticker(value) => value.into(),
            InlineQueryResult::CachedVideo(value) => value.into(),
            InlineQueryResult::CachedVoice(value) => value.into(),
            InlineQueryResult::Contact(value) => value.into(),
            InlineQueryResult::Document(value) => value.into(),
            InlineQueryResult::Game(value) => value.into(),
            InlineQueryResult::Gif(value) => value.into(),
            InlineQueryResult::Location(value) => value.into(),
            InlineQueryResult::Mpeg4Gif(value) => value.into(),
            InlineQueryResult::Photo(value) => value.into(),
            InlineQueryResult::Venue(value) => value.into(),
            InlineQueryResult::Video(value) => value.into(),
            InlineQueryResult::Voice(value) => value.into(),
        }
    }
}

#[derive(Debug)]
pub(super) enum RawInlineQueryResultDataError {
    MissingField(&'static str),
}

impl Display for RawInlineQueryResultDataError {
    fn fmt(&self, out: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RawInlineQueryResultDataError::MissingField(field_name) => write!(out, "Field {} is missing", field_name),
        }
    }
}

impl Error for RawInlineQueryResultDataError {}

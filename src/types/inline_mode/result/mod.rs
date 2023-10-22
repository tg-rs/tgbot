use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, Location, User, WebAppInfo},
};

use self::raw::RawInlineQueryResult;
pub use self::{
    article::*,
    audio::*,
    contact::*,
    document::*,
    game::*,
    gif::*,
    location::*,
    mpeg4_gif::*,
    photo::*,
    sticker::*,
    venue::*,
    video::*,
    voice::*,
};

#[cfg(test)]
mod tests;

mod article;
mod audio;
mod contact;
mod document;
mod game;
mod gif;
mod location;
mod mpeg4_gif;
mod photo;
mod raw;
mod sticker;
mod venue;
mod video;
mod voice;

/// Result of an inline query
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "RawInlineQueryResult", into = "RawInlineQueryResult")]
pub enum InlineQueryResult {
    /// Link to an article or web page
    Article(InlineQueryResultArticle),
    /// Link to an mp3 audio file
    Audio(InlineQueryResultAudio),
    /// Link to an mp3 audio file stored on the Telegram servers
    CachedAudio(InlineQueryResultCachedAudio),
    /// Link to a file stored on the Telegram servers
    CachedDocument(InlineQueryResultCachedDocument),
    /// Link to an animated GIF file stored on the Telegram servers
    CachedGif(InlineQueryResultCachedGif),
    /// Link to a video animation
    /// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// Link to a photo stored on the Telegram servers
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// Link to a sticker stored on the Telegram servers
    CachedSticker(InlineQueryResultCachedSticker),
    /// Link to a video file stored on the Telegram servers
    CachedVideo(InlineQueryResultCachedVideo),
    /// Link to a voice message stored on the Telegram servers
    CachedVoice(InlineQueryResultCachedVoice),
    /// Contact with a phone number
    Contact(InlineQueryResultContact),
    /// Link to a file
    Document(InlineQueryResultDocument),
    /// Game
    Game(InlineQueryResultGame),
    /// Link to an animated GIF file
    Gif(InlineQueryResultGif),
    /// Location on a map
    Location(InlineQueryResultLocation),
    /// Link to a video animation (H.264/MPEG-4 AVC video without sound)
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// Link to a photo
    Photo(InlineQueryResultPhoto),
    /// Venue
    Venue(InlineQueryResultVenue),
    /// Link to a page containing an embedded video player or a video file
    Video(InlineQueryResultVideo),
    /// Link to a voice recording in an .ogg container encoded with OPUS
    Voice(InlineQueryResultVoice),
}

/// Represents a button to be shown above inline query results
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InlineQueryResultsButton {
    text: String,
    #[serde(flatten)]
    button_type: InlineQueryResultsButtonType,
}

impl InlineQueryResultsButton {
    /// Creates a new InlineQueryResultsButton
    ///
    /// # Arguments
    ///
    /// * text - Label text on the button
    /// * web_app_info - Description of the Web App that will be launched
    ///                  when the user presses the button.
    ///                  The Web App will be able to switch back to the inline mode
    ///                  using the method switchInlineQuery inside the Web App.
    pub fn with_web_app<T>(text: T, web_app_info: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            button_type: InlineQueryResultsButtonType::WebApp(web_app_info),
        }
    }

    /// Creates a new InlineQueryResultsButton
    ///
    /// # Arguments
    ///
    /// * text - Label text on the button
    /// * start_parameter - Deep-linking parameter for the /start message
    ///                     sent to the bot when a user presses the button.
    ///                     1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    pub fn with_start_parameter<A, B>(text: A, start_parameter: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            text: text.into(),
            button_type: InlineQueryResultsButtonType::StartParameter(start_parameter.into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum InlineQueryResultsButtonType {
    WebApp(WebAppInfo),
    StartParameter(String),
}

/// Result of an inline query that was chosen by the user and sent to their chat partner
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message
    /// Will be also received in callback queries and can be used to edit the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

/// Use this method to send answers to an inline query
///
/// No more than 50 results per query are allowed
#[derive(Clone, Debug, Serialize)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<InlineQueryResultsButton>,
}

impl AnswerInlineQuery {
    /// Creates a new AnswerInlineQuery with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * inline_query_id - Unique identifier for the answered query
    /// * results - An array of results for the inline query
    pub fn new<S: Into<String>>(inline_query_id: S, results: Vec<InlineQueryResult>) -> Self {
        AnswerInlineQuery {
            inline_query_id: inline_query_id.into(),
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }

    /// Maximum amount of time in seconds that the result of the inline query may be cached on the server
    ///
    /// Defaults to 300
    pub fn cache_time(mut self, cache_time: Integer) -> Self {
        self.cache_time = Some(cache_time);
        self
    }

    /// Cache results on the server side only for the user that sent the query
    ///
    /// By default, results may be returned to any user who sends the same query
    pub fn personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }

    /// Offset that a client should send in the next query with the same text to receive more results
    ///
    /// Pass an empty string if there are no more results or if you don‘t support pagination
    /// Offset length can’t exceed 64 bytes
    pub fn next_offset<S: Into<String>>(mut self, next_offset: S) -> Self {
        self.next_offset = Some(next_offset.into());
        self
    }

    /// An object describing a button to be shown above inline query results
    pub fn button(mut self, value: InlineQueryResultsButton) -> Self {
        self.button = Some(value);
        self
    }
}

impl Method for AnswerInlineQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerInlineQuery", self)
    }
}

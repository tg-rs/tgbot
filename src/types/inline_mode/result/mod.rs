use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{Integer, Location, User},
};

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
mod sticker;
mod venue;
mod video;
mod voice;

/// Result of an inline query
#[derive(Clone, Debug, derive_more::From, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum InlineQueryResult {
    /// Link to an article or web page
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),
    /// Link to an mp3 audio file
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),
    /// Link to an mp3 audio file stored on the Telegram servers
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    /// Link to a file stored on the Telegram servers
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    /// Link to an animated GIF file stored on the Telegram servers
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    /// Link to a video animation
    /// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// Link to a photo stored on the Telegram servers
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// Link to a sticker stored on the Telegram servers
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    /// Link to a video file stored on the Telegram servers
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    /// Link to a voice message stored on the Telegram servers
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),
    /// Contact with a phone number
    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),
    /// Link to a file
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),
    /// Game
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),
    /// Link to an animated GIF file
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),
    /// Location on a map
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),
    /// Link to a video animation (H.264/MPEG-4 AVC video without sound)
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// Link to a photo
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),
    /// Venue
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),
    /// Link to a page containing an embedded video player or a video file
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),
    /// Link to a voice recording in an .ogg container encoded with OPUS
    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

/// Result of an inline query that was chosen by the user and sent to their chat partner
#[derive(Clone, Debug, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message
    /// Will be also received in callback queries and can be used to edit the message
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
    switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<String>,
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
            switch_pm_text: None,
            switch_pm_parameter: None,
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

    /// Offset that a clien should send in the next query with the same text to receive more results
    ///
    /// Pass an empty string if there are no more results or if you don‘t support pagination
    /// Offset length can’t exceed 64 bytes
    pub fn next_offset<S: Into<String>>(mut self, next_offset: S) -> Self {
        self.next_offset = Some(next_offset.into());
        self
    }

    /// Clients will display a button with specified text that switches the user
    /// to a private chat with the bot and sends the bot a
    /// start message with the parameter switch_pm_parameter
    pub fn switch_pm_text<S: Into<String>>(mut self, switch_pm_text: S) -> Self {
        self.switch_pm_text = Some(switch_pm_text.into());
        self
    }

    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch button
    ///
    /// 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed
    ///
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to
    /// their YouTube account to adapt search results accordingly
    /// To do this, it displays a ‘Connect your YouTube account’
    /// button above the results, or even before showing any
    /// The user presses the button, switches to a private chat with the bot and, in doing so,
    /// passes a start parameter that instructs the bot to return an oauth link
    /// Once done, the bot can offer a switch_inline button so that the user can easily
    /// return to the chat where they wanted to use the bot's inline capabilities
    pub fn switch_pm_parameter<S: Into<String>>(mut self, switch_pm_parameter: S) -> Self {
        self.switch_pm_parameter = Some(switch_pm_parameter.into());
        self
    }
}

impl Method for AnswerInlineQuery {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("answerInlineQuery", self)
    }
}

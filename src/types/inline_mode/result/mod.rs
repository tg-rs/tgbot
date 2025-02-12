use serde::{Deserialize, Serialize};

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
use crate::types::{Location, User, WebAppInfo};

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

/// Represents a result of an inline query.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "RawInlineQueryResult", into = "RawInlineQueryResult")]
pub enum InlineQueryResult {
    /// Link to an article or web page.
    Article(InlineQueryResultArticle),
    /// Link to an MP3 audio file.
    Audio(InlineQueryResultAudio),
    /// Link to an MP3 audio file stored on the Telegram servers.
    CachedAudio(InlineQueryResultCachedAudio),
    /// Link to a file stored on the Telegram servers.
    CachedDocument(InlineQueryResultCachedDocument),
    /// Link to an animated GIF file stored on the Telegram servers.
    CachedGif(InlineQueryResultCachedGif),
    /// Link to a video animation
    /// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers.
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// Link to a photo stored on the Telegram servers.
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// Link to a sticker stored on the Telegram servers.
    CachedSticker(InlineQueryResultCachedSticker),
    /// Link to a video file stored on the Telegram servers.
    CachedVideo(InlineQueryResultCachedVideo),
    /// Link to a voice message stored on the Telegram servers.
    CachedVoice(InlineQueryResultCachedVoice),
    /// Contact with a phone number.
    Contact(InlineQueryResultContact),
    /// Link to a file.
    Document(InlineQueryResultDocument),
    /// Game.
    Game(InlineQueryResultGame),
    /// Link to an animated GIF file.
    Gif(InlineQueryResultGif),
    /// Location on a map.
    Location(InlineQueryResultLocation),
    /// Link to a video animation (H.264/MPEG-4 AVC video without sound).
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// Link to a photo.
    Photo(InlineQueryResultPhoto),
    /// Venue.
    Venue(InlineQueryResultVenue),
    /// Link to a page containing an embedded video player or a video file.
    Video(InlineQueryResultVideo),
    /// Link to a voice recording in an OGG container encoded with OPUS.
    Voice(InlineQueryResultVoice),
}

/// Represents a button to be shown above inline query results.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InlineQueryResultsButton {
    text: String,
    #[serde(flatten)]
    button_type: InlineQueryResultsButtonType,
}

impl InlineQueryResultsButton {
    /// Creates a new `InlineQueryResultsButton` for a web app.
    ///
    /// # Arguments
    ///
    /// * `text` - Label text on the button.
    /// * `web_app_info` - Description of the Web App that will be launched
    ///                    when the user presses the button;
    ///                    the Web App will be able to switch back to the inline mode
    ///                    using the method `switchInlineQuery` inside the Web App.
    pub fn for_web_app<T>(text: T, web_app_info: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            button_type: InlineQueryResultsButtonType::WebApp(web_app_info),
        }
    }

    /// Creates a new `InlineQueryResultsButton` for a parameter of the `/start` message.
    ///
    /// # Arguments
    ///
    /// * `text` - Label text on the button.
    /// * `start_parameter` - Deep-linking parameter for the `/start` message
    ///                       sent to the bot when a user presses the button;
    ///                       1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    pub fn for_start_parameter<A, B>(text: A, start_parameter: B) -> Self
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

/// Represents a result of an inline query
/// that was chosen by the user and sent to their chat partner.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChosenInlineResult {
    /// The user that chose the result.
    pub from: User,
    /// The query that was used to obtain the result.
    pub query: String,
    /// The unique identifier for the result that was chosen.
    pub result_id: String,
    /// Identifier of the sent inline message.
    ///
    /// Available only if there is an inline keyboard attached to the message.
    /// Will be also received in callback queries and can be used to edit the message.
    pub inline_message_id: Option<String>,
    /// Sender location, only for bots that require user location.
    pub location: Option<Location>,
}

impl ChosenInlineResult {
    /// Creates a new `ChosenInlineResult`.
    ///
    /// # Arguments
    ///
    /// * `from` - The user that chose the result.
    /// * `query` - The query that was used to obtain the result.
    /// * `result_id` - Unique identifier of the chosen result.
    pub fn new<A, B>(from: User, query: A, result_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            from,
            query: query.into(),
            result_id: result_id.into(),
            inline_message_id: None,
            location: None,
        }
    }

    /// Sets a new inline message ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the sent inline message.
    pub fn with_inline_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(value.into());
        self
    }

    /// Sets a new location.
    ///
    /// # Arguments
    ///
    /// * `value` - Sender location.
    pub fn with_location(mut self, value: Location) -> Self {
        self.location = Some(value);
        self
    }
}

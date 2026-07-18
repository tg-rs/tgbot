use serde::{Deserialize, Serialize};

use crate::{
    api::Form,
    types::{
        Float,
        InlineKeyboardMarkup,
        InputMessageContent,
        InputMessageContentData,
        Integer,
        Location,
        ParseMode,
        SerializeError,
        TextEntities,
        TextEntity,
        User,
        WebAppInfo,
    },
};

/// Represents a result of an inline query.
#[derive(Debug, derive_more::From)]
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

impl InlineQueryResult {
    pub(crate) fn into_parts(self) -> (Option<Form>, InlineQueryResultData) {
        match self {
            InlineQueryResult::Article(value) => (value.form, value.data),
            InlineQueryResult::Audio(value) => (value.form, value.data),
            InlineQueryResult::CachedAudio(value) => (value.form, value.data),
            InlineQueryResult::CachedDocument(value) => (value.form, value.data),
            InlineQueryResult::CachedGif(value) => (value.form, value.data),
            InlineQueryResult::CachedMpeg4Gif(value) => (value.form, value.data),
            InlineQueryResult::CachedPhoto(value) => (value.form, value.data),
            InlineQueryResult::CachedSticker(value) => (value.form, value.data),
            InlineQueryResult::CachedVideo(value) => (value.form, value.data),
            InlineQueryResult::CachedVoice(value) => (value.form, value.data),
            InlineQueryResult::Contact(value) => (value.form, value.data),
            InlineQueryResult::Document(value) => (value.form, value.data),
            InlineQueryResult::Game(value) => (value.form, value.data),
            InlineQueryResult::Gif(value) => (value.form, value.data),
            InlineQueryResult::Location(value) => (value.form, value.data),
            InlineQueryResult::Mpeg4Gif(value) => (value.form, value.data),
            InlineQueryResult::Photo(value) => (value.form, value.data),
            InlineQueryResult::Venue(value) => (value.form, value.data),
            InlineQueryResult::Video(value) => (value.form, value.data),
            InlineQueryResult::Voice(value) => (value.form, value.data),
        }
    }
}

/// Represents a link to an article or a web page.
#[derive(Debug)]
pub struct InlineQueryResultArticle {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultArticle {
    /// Creates a new `InlineQueryResultArticle`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 Bytes.
    /// * `input_message_content` - Content of the message.
    /// * `title` - Title of the result.
    pub fn new<A, B, C>(id: A, input_message_content: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<InputMessageContent>,
        C: Into<String>,
    {
        let (form, content) = input_message_content.into().into_parts();
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Article,
                properties: InlineQueryResultProperties {
                    input_message_content: Some(content),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form,
        }
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_width = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new URL.
    ///
    /// * `value` - URL of the result.
    ///   Pass an empty string to hide the URL.
    pub fn with_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.url = Some(value.into());
        self
    }
}

/// Represents a link to an mp3 audio file.
///
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the audio.
#[derive(Debug)]
pub struct InlineQueryResultAudio {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultAudio {
    /// Creates a new `InlineQueryResultAudio`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 bytes.
    /// * `audio_url` - A valid URL of the audio file.
    /// * `title` - Title.
    pub fn new<A, B, C>(audio_url: A, id: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Audio,
                properties: InlineQueryResultProperties {
                    audio_url: Some(audio_url.into()),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new audio duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Audio duration in seconds.
    pub fn with_audio_duration(mut self, value: Integer) -> Self {
        self.data.properties.audio_duration = Some(value);
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the audio.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.performer = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }
}

/// Represents a link to an mp3 audio file stored on the Telegram servers.
///
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the audio.
#[derive(Debug)]
pub struct InlineQueryResultCachedAudio {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedAudio {
    /// Creates a new `InlineQueryResultCachedAudio`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 bytes.
    /// * `audio_file_id` - A valid file identifier for the audio file.
    pub fn new<A, B>(audio_file_id: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Audio,
                properties: InlineQueryResultProperties {
                    audio_file_id: Some(audio_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the audio.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }
}

/// Represents a link to a file stored on the Telegram servers.
///
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the file.
#[derive(Debug)]
pub struct InlineQueryResultCachedDocument {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedDocument {
    /// Creates a new `InlineQueryResultCachedDocument`.
    ///
    /// # Arguments
    ///
    /// * `document_file_id` - A valid file identifier of the file.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    pub fn new<A, B, C>(document_file_id: A, id: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Document,
                properties: InlineQueryResultProperties {
                    document_file_id: Some(document_file_id.into()),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the file.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }
}

/// Link to an animated GIF file stored on the Telegram servers.
///
/// By default, this animated GIF file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with specified content instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultCachedGif {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedGif {
    /// Creates a new `InlineQueryResultCachedGif`.
    ///
    /// # Arguments
    ///
    /// * `gif_file_id` - A valid file identifier for the GIF file.
    /// * `id` - Unique identifier for this result; 1-64 bytes.
    pub fn new<A, B>(gif_file_id: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Gif,
                properties: InlineQueryResultProperties {
                    gif_file_id: Some(gif_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the GIF animation.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(value.into());
        self
    }
}

/// Represents a link to a video animation
/// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers.
///
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content
/// instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedMpeg4Gif {
    /// Creates a new `InlineQueryResultCachedMpeg4Gif`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mpeg4_file_id` - A valid file identifier for the MP4 file.
    pub fn new<A, B>(id: A, mpeg4_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Mpeg4Gif,
                properties: InlineQueryResultProperties {
                    mpeg4_file_id: Some(mpeg4_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the video animation.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(title.into());
        self
    }
}

/// Represents a link to a photo stored on the Telegram servers.
///
/// By default, this photo will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the photo.
#[derive(Debug)]
pub struct InlineQueryResultCachedPhoto {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedPhoto {
    /// Creates a new `InlineQueryResultCachedPhoto`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `photo_file_id` - A valid file identifier of the photo.
    pub fn new<A, B>(id: A, photo_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Photo,
                properties: InlineQueryResultProperties {
                    photo_file_id: Some(photo_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the photo.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(value.into());
        self
    }
}

/// Represents a link to a sticker stored on the Telegram servers.
///
/// By default, this sticker will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to
/// send a message with the specified content instead of the sticker.
#[derive(Debug)]
pub struct InlineQueryResultCachedSticker {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedSticker {
    /// Creates a new `InlineQueryResultCachedSticker`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `sticker_file_id` - A valid file identifier of the sticker.
    pub fn new<A, B>(id: A, sticker_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Sticker,
                properties: InlineQueryResultProperties {
                    sticker_file_id: Some(sticker_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the photo.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }
}

/// Represents a link to a video file stored on the Telegram servers.
///
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the video.
#[derive(Debug)]
pub struct InlineQueryResultCachedVideo {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedVideo {
    /// Creates a new `InlineQueryResultCachedVideo`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    /// * `video_file_id` - A valid file identifier of the video.
    pub fn new<A, B, C>(id: A, title: B, video_file_id: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Video,
                properties: InlineQueryResultProperties {
                    title: Some(title.into()),
                    video_file_id: Some(video_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the video.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }
}

/// Represents a link to a voice message stored on the Telegram servers.
///
/// By default, this voice message will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the voice message.
#[derive(Debug)]
pub struct InlineQueryResultCachedVoice {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultCachedVoice {
    /// Creates a new `InlineQueryResultCachedVoice`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    /// * `voice_file_id` - A valid file identifier of the voice message.
    pub fn new<A, B, C>(id: A, title: B, voice_file_id: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Voice,
                properties: InlineQueryResultProperties {
                    title: Some(title.into()),
                    voice_file_id: Some(voice_file_id.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the voice message.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }
}

/// Represents a contact with a phone number.
///
/// By default, this contact will be sent by the user.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the contact.
#[derive(Debug)]
pub struct InlineQueryResultContact {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultContact {
    /// Creates a new `InlineQueryResultContact`.
    ///
    /// # Arguments
    ///
    /// * `first_name` - Contact's first name.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `phone_number` - Contact's phone number.
    pub fn new<A, B, C>(first_name: A, id: B, phone_number: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Contact,
                properties: InlineQueryResultProperties {
                    first_name: Some(first_name.into()),
                    phone_number: Some(phone_number.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the contact.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.last_name = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_width = Some(value);
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.vcard = Some(value.into());
        self
    }
}

/// Represents a link to a file.
///
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send a message
/// with the specified content instead of the file.
/// Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Debug)]
pub struct InlineQueryResultDocument {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultDocument {
    /// Creates a new `InlineQueryResultDocument`.
    ///
    /// # Arguments
    ///
    /// * `document_url` - A valid URL for the file.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mime_type` - MIME type of the content of the file, either “application/pdf” or “application/zip”.
    /// * `title` - Title of the result.
    pub fn new<A, B, C, D>(document_url: A, id: B, mime_type: C, title: D) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Document,
                properties: InlineQueryResultProperties {
                    document_url: Some(document_url.into()),
                    mime_type: Some(mime_type.into()),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the file.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_width = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail (jpeg only) for the file.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_url = Some(value.into());
        self
    }
}

/// Represents a game.
#[derive(Debug)]
pub struct InlineQueryResultGame {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultGame {
    /// Creates a new `InlineQueryResultGame`.
    ///
    /// # Arguments
    ///
    /// * `game_short_name` - Short name of the game.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    pub fn new<A, B>(game_short_name: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Game,
                properties: InlineQueryResultProperties {
                    game_short_name: Some(game_short_name.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }
}

/// Represents a link to an animated GIF file.
///
/// By default, this animated GIF file
/// will be sent by the user with optional caption.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultGif {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultGif {
    /// Creates a new `InlineQueryResultGif`.
    ///
    /// # Arguments
    ///
    /// * `gif_url` - A valid URL for the GIF file; file size must not exceed 1MB.
    /// * `id` - Unique identifier for this result; 1-64 bytes.
    /// * `thumbnail_url` - URL of the static thumbnail for the result (JPEG or GIF).
    pub fn new<A, B, C>(gif_url: A, id: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Gif,
                properties: InlineQueryResultProperties {
                    gif_url: Some(gif_url.into()),
                    thumbnail_url: Some(thumbnail_url.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new GIF duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration of the GIF.
    pub fn with_gif_duration(mut self, value: Integer) -> Self {
        self.data.properties.gif_duration = Some(value);
        self
    }

    /// Sets a new GIF height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height of the GIF.
    pub fn with_gif_height(mut self, value: Integer) -> Self {
        self.data.properties.gif_height = Some(value);
        self
    }

    /// Sets a new GIF width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width of the GIF.
    pub fn with_gif_width(mut self, value: Integer) -> Self {
        self.data.properties.gif_width = Some(value);
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the GIF animation.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(value.into());
        self
    }

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type of the thumbnail; default - “image/jpeg”.
    ///
    /// Must be one of “image/jpeg”, “image/gif”, or “video/mp4”.
    pub fn with_thumbnail_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_mime_type = Some(value.into());
        self
    }
}

/// Represents a location on a map.
///
/// By default, the location will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the location.
#[derive(Debug)]
pub struct InlineQueryResultLocation {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultLocation {
    /// Creates a new `InlineQueryResultLocation`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this result; 1-64 bytes.
    /// * `latitude` - Location latitude in degrees.
    /// * `longitude` - Location longitude in degrees.
    /// * `title` - Location title.
    pub fn new<A, B>(id: A, latitude: Float, longitude: Float, title: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Location,
                properties: InlineQueryResultProperties {
                    latitude: Some(latitude),
                    longitude: Some(longitude),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new heading.
    ///
    /// # Arguments
    ///
    /// * `value` - A direction in which the user is moving; in degrees; 1-360.
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.data.properties.heading = Some(value);
        self
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - A radius of uncertainty for the location; in meters; 0-1500.
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.data.properties.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the location.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new live period.
    ///
    /// # Arguments
    ///
    /// * `value` - Period in seconds during which the location can be updated,
    ///   should be between 60 and 86400,
    ///   or 0x7FFFFFFF for live locations that can be edited indefinitely.
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.data.properties.live_period = Some(value);
        self
    }

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///   about approaching another chat member; in meters; 1-100000.
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.data.properties.proximity_alert_radius = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_width = Some(value);
        self
    }
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
///
/// By default, this animated MPEG-4 file will be sent by the user with optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultMpeg4Gif {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultMpeg4Gif {
    /// Creates a new `InlineQueryResultMpeg4Gif`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the result; 1-64 bytes.
    /// * `mpeg4_url` - A valid URL for the MP4 file; file size must not exceed 1MB.
    /// * `thumbnail_url` - URL of the static thumbnail (jpeg or gif) for the result.
    pub fn new<A, B, C>(id: A, mpeg4_url: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Mpeg4Gif,
                properties: InlineQueryResultProperties {
                    mpeg4_url: Some(mpeg4_url.into()),
                    thumbnail_url: Some(thumbnail_url.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the video animation.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new MPEG4 duration.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 duration.
    pub fn with_mpeg4_duration(mut self, value: Integer) -> Self {
        self.data.properties.mpeg4_duration = Some(value);
        self
    }

    /// Sets a new MPEG4 height.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 height.
    pub fn with_mpeg4_height(mut self, value: Integer) -> Self {
        self.data.properties.mpeg4_height = Some(value);
        self
    }

    /// Sets a new MPEG4 width.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 width.
    pub fn with_mpeg4_width(mut self, value: Integer) -> Self {
        self.data.properties.mpeg4_width = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new thumbnail MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type of the thumbnail; default - “image/jpeg”.
    ///
    /// Must be one of “image/jpeg”, “image/gif”, or “video/mp4”.
    pub fn with_thumbnail_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_mime_type = Some(value.into());
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title of the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(value.into());
        self
    }
}

/// Represents a link to a photo.
///
/// By default, a photo will be sent by the user with optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the photo.
#[derive(Debug)]
pub struct InlineQueryResultPhoto {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultPhoto {
    /// Creates a new `InlineQueryResultPhoto`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `photo_url` - A valid URL of the photo; must be in jpeg format; size must not exceed 5MB.
    /// * `thumbnail_url` - URL of the thumbnail of the photo.
    pub fn new<A, B, C>(id: A, photo_url: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Photo,
                properties: InlineQueryResultProperties {
                    photo_url: Some(photo_url.into()),
                    thumbnail_url: Some(thumbnail_url.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the photo.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new photo height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height of the photo.
    pub fn with_photo_height(mut self, value: Integer) -> Self {
        self.data.properties.photo_height = Some(value);
        self
    }

    /// Sets a new photo width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width of the photo.
    pub fn with_photo_width(mut self, value: Integer) -> Self {
        self.data.properties.photo_width = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title of the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.title = Some(value.into());
        self
    }
}

/// Represents a venue.
///
/// By default, the venue will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the venue.
#[derive(Debug)]
pub struct InlineQueryResultVenue {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultVenue {
    /// Creates a new `InlineQueryResultVenue`.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the venue.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `latitude` - Latitude of the venue location in degrees.
    /// * `longitude` - Longitude of the venue location in degrees.
    /// * `title` - Title of the venue.
    pub fn new<A, B, C>(address: A, id: B, latitude: Float, longitude: Float, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Venue,
                properties: InlineQueryResultProperties {
                    address: Some(address.into()),
                    latitude: Some(latitude),
                    longitude: Some(longitude),
                    title: Some(title.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new foursquare ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Foursquare identifier of the venue.
    pub fn with_foursquare_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.foursquare_id = Some(value.into());
        self
    }

    /// Sets a new foursquare type.
    ///
    /// # Arguments
    ///
    /// * `value` - Foursquare type of the venue.
    ///
    /// For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/ice-cream”.
    pub fn with_foursquare_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.foursquare_type = Some(value.into());
        self
    }

    /// Sets a new Google Places ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places identifier of the venue.
    pub fn with_google_place_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.google_place_id = Some(value.into());
        self
    }

    /// Sets a new Google Places type.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places type of the venue.
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    pub fn with_google_place_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.google_place_type = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the venue.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.data.properties.thumbnail_width = Some(value);
        self
    }
}

/// Represents a link to a page containing an embedded video player or a video file.
///
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send a message with
/// the specified content instead of the video.
/// If an [`InlineQueryResultVideo`] message contains an embedded video (e.g., YouTube),
/// you must replace its content using [`Self::with_input_message_content`].
#[derive(Debug)]
pub struct InlineQueryResultVideo {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultVideo {
    /// Creates a new `InlineQueryResultVideo`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mime_type` - MIME type of the content of video url: “text/html” or “video/mp4”.
    /// * `thumbnail_url` - URL of the thumbnail for the video; JPEG only.
    /// * `title` - Title of the result.
    /// * `video_url` - A valid URL of the embedded video player or video file.
    pub fn new<A, B, C, D, E>(id: A, mime_type: B, thumbnail_url: C, title: D, video_url: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Video,
                properties: InlineQueryResultProperties {
                    mime_type: Some(mime_type.into()),
                    thumbnail_url: Some(thumbnail_url.into()),
                    title: Some(title.into()),
                    video_url: Some(video_url.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the video.
    ///
    /// This field is required if [`InlineQueryResultVideo`] is used
    /// to send an HTML-page as a result (e.g., a YouTube video).
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.data.properties.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new video duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Video duration in seconds.
    pub fn with_video_duration(mut self, value: Integer) -> Self {
        self.data.properties.video_duration = Some(value);
        self
    }

    /// Sets a new video height.
    ///
    /// # Arguments
    ///
    /// * `value` - Video height.
    pub fn with_video_height(mut self, value: Integer) -> Self {
        self.data.properties.video_height = Some(value);
        self
    }

    /// Sets a new vide width.
    ///
    /// # Arguments
    ///
    /// * `value` - Video width.
    pub fn with_video_width(mut self, value: Integer) -> Self {
        self.data.properties.video_width = Some(value);
        self
    }
}

/// Represents a link to a voice recording in an OGG container encoded with OPUS.
///
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the the voice message.
#[derive(Debug)]
pub struct InlineQueryResultVoice {
    data: InlineQueryResultData,
    form: Option<Form>,
}

impl InlineQueryResultVoice {
    /// Creates a new `InlineQueryResultVoice`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Recording title.
    /// * `voice_url` - A valid URL of the voice recording.
    pub fn new<A, B, C>(id: A, title: B, voice_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: InlineQueryResultData {
                id: id.into(),
                result_type: InlineQueryResultType::Voice,
                properties: InlineQueryResultProperties {
                    title: Some(title.into()),
                    voice_url: Some(voice_url.into()),
                    ..Default::default()
                },
            },
            form: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.properties.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.properties.caption_entities = Some(value.into_iter().collect());
        self.data.properties.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.properties.parse_mode = Some(value);
        self.data.properties.caption_entities = None;
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the voice recording.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        let (form, data) = value.into().into_parts();
        self.form = form;
        self.data.properties.input_message_content = Some(data);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.data.properties.reply_markup = Some(value.into());
        self
    }

    /// Sets a new voice duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Recording duration in seconds.
    pub fn with_voice_duration(mut self, value: Integer) -> Self {
        self.data.properties.voice_duration = Some(value);
        self
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum InlineQueryResultType {
    Article,
    Audio,
    Contact,
    Document,
    Game,
    Gif,
    Location,
    Mpeg4Gif,
    Photo,
    Sticker,
    Venue,
    Video,
    Voice,
}

#[derive(Debug, Serialize)]
pub(crate) struct InlineQueryResultData {
    id: String,
    #[serde(rename = "type")]
    result_type: InlineQueryResultType,
    #[serde(flatten)]
    properties: InlineQueryResultProperties,
}

impl InlineQueryResultData {
    pub(crate) fn serialize(&self) -> Result<String, SerializeError> {
        serde_json::to_string(&self).map_err(SerializeError::inline_query_result_data)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct InlineQueryResultProperties {
    address: Option<String>,
    audio_duration: Option<Integer>,
    audio_file_id: Option<String>,
    audio_url: Option<String>,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    description: Option<String>,
    document_file_id: Option<String>,
    document_url: Option<String>,
    first_name: Option<String>,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    game_short_name: Option<String>,
    gif_duration: Option<Integer>,
    gif_file_id: Option<String>,
    gif_height: Option<Integer>,
    gif_url: Option<String>,
    gif_width: Option<Integer>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    heading: Option<Integer>,
    horizontal_accuracy: Option<Float>,
    input_message_content: Option<InputMessageContentData>,
    last_name: Option<String>,
    latitude: Option<Float>,
    live_period: Option<Integer>,
    longitude: Option<Float>,
    mime_type: Option<String>,
    mpeg4_duration: Option<Integer>,
    mpeg4_file_id: Option<String>,
    mpeg4_height: Option<Integer>,
    mpeg4_url: Option<String>,
    mpeg4_width: Option<Integer>,
    parse_mode: Option<ParseMode>,
    performer: Option<String>,
    photo_file_id: Option<String>,
    photo_height: Option<Integer>,
    phone_number: Option<String>,
    photo_url: Option<String>,
    photo_width: Option<Integer>,
    proximity_alert_radius: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
    show_caption_above_media: Option<bool>,
    sticker_file_id: Option<String>,
    thumbnail_height: Option<Integer>,
    thumbnail_mime_type: Option<String>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<Integer>,
    title: Option<String>,
    url: Option<String>,
    vcard: Option<String>,
    video_duration: Option<Integer>,
    video_file_id: Option<String>,
    video_height: Option<Integer>,
    video_url: Option<String>,
    video_width: Option<Integer>,
    voice_duration: Option<Integer>,
    voice_file_id: Option<String>,
    voice_url: Option<String>,
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
    ///   when the user presses the button;
    ///   the Web App will be able to switch back to the inline mode
    ///   using the method `switchInlineQuery` inside the Web App.
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
    ///   sent to the bot when a user presses the button;
    ///   1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
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

    pub(crate) fn serialize(&self) -> Result<String, SerializeError> {
        serde_json::to_string(&self).map_err(SerializeError::inline_query_results_button)
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

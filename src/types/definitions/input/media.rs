use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, WriteForm},
    types::{Float, InputFile, Integer, ParseMode, TextEntities, TextEntity},
};

/// Represents a metadata of the input media.
#[derive(Debug)]
pub struct InputMedia {
    media_type: InputMediaType,
    cover: Option<InputFile>,
    media: Option<InputFile>,
    parameters: InputMediaParameters,
    photo: Option<InputFile>,
    thumbnail: Option<InputFile>,
}

impl From<InputMediaAnimation> for InputMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::new(InputMediaType::Animation, value.parameters)
            .with_media(value.media)
            .with_thumbnail(value.thumbnail)
    }
}

impl From<InputMediaAudio> for InputMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::new(InputMediaType::Audio, value.parameters)
            .with_media(value.media)
            .with_thumbnail(value.thumbnail)
    }
}

impl From<InputMediaDocument> for InputMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::new(InputMediaType::Document, value.parameters)
            .with_media(value.media)
            .with_thumbnail(value.thumbnail)
    }
}

impl From<InputMediaLivePhoto> for InputMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::new(InputMediaType::LivePhoto, value.parameters)
            .with_media(value.media)
            .with_photo(value.photo)
    }
}

impl From<InputMediaLocation> for InputMedia {
    fn from(value: InputMediaLocation) -> Self {
        Self::new(InputMediaType::Location, value.parameters)
    }
}

impl From<InputMediaPhoto> for InputMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::new(InputMediaType::Photo, value.parameters).with_media(value.media)
    }
}

impl From<InputMediaSticker> for InputMedia {
    fn from(value: InputMediaSticker) -> Self {
        Self::new(InputMediaType::Sticker, value.parameters).with_media(value.media)
    }
}

impl From<InputMediaVenue> for InputMedia {
    fn from(value: InputMediaVenue) -> Self {
        Self::new(InputMediaType::Venue, value.parameters)
    }
}

impl From<InputMediaVideo> for InputMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::new(InputMediaType::Video, value.parameters)
            .with_cover(value.cover)
            .with_media(value.media)
            .with_thumbnail(value.thumbnail)
    }
}

impl From<InputMediaVoiceNote> for InputMedia {
    fn from(value: InputMediaVoiceNote) -> Self {
        Self::new(InputMediaType::VoiceNote, value.parameters).with_media(value.media)
    }
}

impl InputMedia {
    /// Creates a new `InputMedia` for an HTTP link.
    ///
    /// # Arguments
    ///
    /// * `value` - HTTP URL of the link
    pub fn link<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(
            InputMediaType::Link,
            InputMediaParameters {
                url: Some(value.into()),
                ..Default::default()
            },
        )
    }

    fn new(media_type: InputMediaType, parameters: InputMediaParameters) -> Self {
        Self {
            media_type,
            cover: None,
            media: None,
            parameters,
            photo: None,
            thumbnail: None,
        }
    }

    fn with_cover(mut self, value: Option<InputFile>) -> Self {
        self.cover = value;
        self
    }

    fn with_media(mut self, value: InputFile) -> Self {
        self.media = Some(value);
        self
    }

    fn with_photo(mut self, value: InputFile) -> Self {
        self.photo = Some(value);
        self
    }

    fn with_thumbnail(mut self, value: Option<InputFile>) -> Self {
        self.thumbnail = value;
        self
    }
}

impl WriteForm for InputMedia {
    type Output = InputMediaData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            cover,
            media,
            media_type,
            mut parameters,
            photo,
            thumbnail,
        } = self;
        parameters.cover = cover.map(|x| x.write(form));
        parameters.media = media.map(|x| x.write(form));
        parameters.photo = photo.map(|x| x.write(form));
        parameters.thumbnail = thumbnail.map(|x| x.write(form));
        InputMediaData { media_type, parameters }
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Debug)]
pub struct InputMediaAnimation {
    parameters: InputMediaParameters,
    media: InputFile,
    thumbnail: Option<InputFile>,
}

impl<T> From<T> for InputMediaAnimation
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
            thumbnail: None,
        }
    }
}

impl InputMediaAnimation {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.parameters.height = Some(value);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.parameters.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail of the file sent.
    ///
    /// Can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    pub fn with_thumbnail<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(value.into());
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.parameters.width = Some(value);
        self
    }
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Debug)]
pub struct InputMediaAudio {
    media: InputFile,
    parameters: InputMediaParameters,
    thumbnail: Option<InputFile>,
}

impl<T> From<T> for InputMediaAudio
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
            thumbnail: None,
        }
    }
}

impl InputMediaAudio {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
        self
    }

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer of the audio.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.performer = Some(value.into());
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail of the file sent.
    ///
    /// Can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    pub fn with_thumbnail<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(value.into());
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title of the audio.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.title = Some(value.into());
        self
    }
}

/// Represents a general file to be sent.
#[derive(Debug)]
pub struct InputMediaDocument {
    media: InputFile,
    parameters: InputMediaParameters,
    thumbnail: Option<InputFile>,
}

impl<T> From<T> for InputMediaDocument
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
            thumbnail: None,
        }
    }
}

impl InputMediaDocument {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new value for the `disable_content_type_detection` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to disable automatic server-side content type detection
    ///   for files uploaded using `multipart/form-data`.
    ///
    /// Always [`true`], if the document is sent as part of an album.
    pub fn with_disable_content_type_detection(mut self, value: bool) -> Self {
        self.parameters.disable_content_type_detection = Some(value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail of the file sent.
    ///
    /// Can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    pub fn with_thumbnail<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(value.into());
        self
    }
}

/// Represents a live photo to be sent.
#[derive(Debug)]
pub struct InputMediaLivePhoto {
    media: InputFile,
    photo: InputFile,
    parameters: InputMediaParameters,
}

impl<A, B> From<(A, B)> for InputMediaLivePhoto
where
    A: Into<InputFile>,
    B: Into<InputFile>,
{
    fn from((media, photo): (A, B)) -> Self {
        Self {
            media: media.into(),
            photo: photo.into(),
            parameters: Default::default(),
        }
    }
}

impl InputMediaLivePhoto {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the live photo to be sent;
    ///   0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the live photo caption.
    ///   Caption entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the caption;
    ///   Parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.parameters.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the live photo needs to be covered with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
        self
    }
}

/// Represents a location to be sent.
#[derive(Debug)]
pub struct InputMediaLocation {
    parameters: InputMediaParameters,
}

impl InputMediaLocation {
    /// Creates a new `InputMediaLocation`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude of the location.
    /// * `longitude` - Longitude of the location.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            parameters: InputMediaParameters {
                latitude: Some(latitude),
                longitude: Some(longitude),
                ..Default::default()
            },
        }
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.parameters.horizontal_accuracy = Some(value);
        self
    }
}

/// Represents a photo to be sent.
#[derive(Debug)]
pub struct InputMediaPhoto {
    media: InputFile,
    parameters: InputMediaParameters,
}

impl<T> From<T> for InputMediaPhoto
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
        }
    }
}

impl InputMediaPhoto {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.parameters.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
        self
    }
}

/// Represents a sticker file to be sent.
#[derive(Debug)]
pub struct InputMediaSticker {
    media: InputFile,
    parameters: InputMediaParameters,
}

impl<T> From<T> for InputMediaSticker
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
        }
    }
}

impl InputMediaSticker {
    /// Sets a new emoji.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji associated with the sticker; only for just uploaded stickers
    pub fn with_emoji<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.emoji = Some(value.into());
        self
    }
}

/// Represents a venue to be sent.
#[derive(Debug)]
pub struct InputMediaVenue {
    parameters: InputMediaParameters,
}

impl InputMediaVenue {
    /// Creates a new `InputMediaVenue`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude of the location.
    /// * `longitude` - Longitude of the location.
    /// * `title` - Name of the venue.
    /// * `address` - Address of the venue.
    pub fn new<A, B>(latitude: Float, longitude: Float, title: A, address: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            parameters: InputMediaParameters {
                address: Some(address.into()),
                latitude: Some(latitude),
                longitude: Some(longitude),
                title: Some(title.into()),
                ..Default::default()
            },
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
        self.parameters.foursquare_id = Some(value.into());
        self
    }

    /// Sets a new foursquare type.
    ///
    /// # Arguments
    ///
    /// * `value` - Foursquare type of the venue, if known.
    pub fn with_foursquare_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.foursquare_type = Some(value.into());
        self
    }

    /// Sets a new google place ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places identifier of the venue.
    pub fn with_google_place_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.google_place_id = Some(value.into());
        self
    }

    /// Sets a new google place type.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places type of the venue.
    pub fn with_google_place_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.google_place_type = Some(value.into());
        self
    }
}

/// Represents a video to be sent.
#[derive(Debug)]
pub struct InputMediaVideo {
    media: InputFile,
    parameters: InputMediaParameters,
    cover: Option<InputFile>,
    thumbnail: Option<InputFile>,
}

impl<T> From<T> for InputMediaVideo
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: InputMediaParameters::default(),
            cover: None,
            thumbnail: None,
        }
    }
}

impl InputMediaVideo {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new cover.
    ///
    /// # Arguments
    ///
    /// * `value` - Cover for the video in the message.
    pub fn with_cover<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.cover = Some(value.into());
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.parameters.height = Some(value);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.parameters.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Start timestamp for the video in the message.
    pub fn with_start_timestamp(mut self, value: Integer) -> Self {
        self.parameters.start_timestamp = Some(value);
        self
    }

    /// Sets a new value for the `supports_streaming` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the uploaded video is suitable for streaming.
    pub fn with_supports_streaming(mut self, value: bool) -> Self {
        self.parameters.supports_streaming = Some(value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail of the file sent.
    ///
    /// Can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    pub fn with_thumbnail<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(value.into());
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.parameters.width = Some(value);
        self
    }
}

/// Represents a voice message to be sent.
#[derive(Debug)]
pub struct InputMediaVoiceNote {
    media: InputFile,
    parameters: InputMediaParameters,
}

impl<T> From<T> for InputMediaVoiceNote
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            parameters: Default::default(),
        }
    }
}

impl InputMediaVoiceNote {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.caption = Some(value.into());
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
        self.parameters.caption_entities = Some(value.into_iter().collect());
        self.parameters.parse_mode = None;
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
        self.parameters.parse_mode = Some(value);
        self.parameters.caption_entities = None;
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration of the voice message in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct InputMediaData {
    #[serde(rename = "type")]
    media_type: InputMediaType,
    #[serde(flatten)]
    parameters: InputMediaParameters,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
struct InputMediaParameters {
    address: Option<String>,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    cover: Option<String>,
    disable_content_type_detection: Option<bool>,
    duration: Option<Integer>,
    emoji: Option<String>,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    has_spoiler: Option<bool>,
    height: Option<Integer>,
    horizontal_accuracy: Option<Float>,
    latitude: Option<Float>,
    longitude: Option<Float>,
    media: Option<String>,
    parse_mode: Option<ParseMode>,
    performer: Option<String>,
    photo: Option<String>,
    show_caption_above_media: Option<bool>,
    start_timestamp: Option<Integer>,
    supports_streaming: Option<bool>,
    thumbnail: Option<String>,
    title: Option<String>,
    url: Option<String>,
    width: Option<Integer>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum InputMediaType {
    Animation,
    Audio,
    Document,
    Link,
    LivePhoto,
    Location,
    Photo,
    Sticker,
    Venue,
    Video,
    VoiceNote,
}

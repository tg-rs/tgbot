use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    Animation,
    Audio,
    Chat,
    Contact,
    Dice,
    Document,
    Game,
    Giveaway,
    GiveawayWinners,
    Integer,
    Invoice,
    LinkPreviewOptions,
    Location,
    Message,
    MessageOrigin,
    PhotoSize,
    Poll,
    Sticker,
    Story,
    Venue,
    Video,
    VideoNote,
    Voice,
};

#[cfg(test)]
mod tests;

/// Contains information about a message or a story that is being replied to.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ReplyTo {
    /// The original message.
    ///
    /// Note that the Message object in this field will not contain further
    /// `reply_to` fields even if it itself is a reply.
    #[serde(rename = "reply_to_message")]
    Message(Box<Message>),
    /// The original story.
    #[serde(rename = "reply_to_story")]
    Story(Story),
}

impl From<Message> for ReplyTo {
    fn from(value: Message) -> Self {
        Self::Message(Box::new(value))
    }
}

impl From<Story> for ReplyTo {
    fn from(value: Story) -> Self {
        Self::Story(value)
    }
}

/// Contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to.
    ///
    /// Available only if the chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Whether the message media is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Options used for link preview generation for the original message, if it is a text message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Unique message identifier inside the original chat.
    ///
    /// Available only if the original chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,

    /// Contains data of the message.
    #[serde(flatten)]
    pub data: ExternalReplyData,
}

impl ExternalReplyInfo {
    /// Creates a new `ExternalReplyInfo`.
    ///
    /// # Arguments
    ///
    /// * `data` - Data of the message.
    /// * `origin` - Origin of the message.
    pub fn new<A, B>(data: A, origin: B) -> Self
    where
        A: Into<ExternalReplyData>,
        B: Into<MessageOrigin>,
    {
        Self {
            origin: origin.into(),
            chat: None,
            has_media_spoiler: None,
            link_preview_options: None,
            message_id: None,
            data: data.into(),
        }
    }

    /// Sets a new chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat the original message belongs to.
    pub fn with_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.chat = Some(value.into());
        self
    }

    /// Sets a new value for a `has_media_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message media is covered by a spoiler animation.
    pub fn with_has_media_spoiler(mut self, value: bool) -> Self {
        self.has_media_spoiler = Some(value);
        self
    }

    /// Sets new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - Options used for link preview generation for the original message, if it is a text message.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(value);
        self
    }

    /// Sets a new message ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique message identifier inside the original chat.
    pub fn with_message_id(mut self, value: Integer) -> Self {
        self.message_id = Some(value);
        self
    }
}

/// Contains data of an external reply info.
#[derive(Clone, Debug, Deserialize, derive_more::From, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalReplyData {
    /// Message is an animation, information about the animation.
    Animation(Animation),
    /// Message is an audio file, information about the file.
    Audio(Audio),
    /// Message is a shared contact, information about the contact.
    Contact(Contact),
    /// Message is a dice with random value.
    Dice(Dice),
    /// Message is a general file, information about the file.
    Document(Document),
    /// Message is a game, information about the game.
    Game(Game),
    /// Message is a scheduled giveaway, information about the giveaway.
    Giveaway(Giveaway),
    /// A giveaway with public winners was completed.
    GiveawayWinners(GiveawayWinners),
    /// Message is an invoice for a payment, information about the invoice.
    Invoice(Invoice),
    /// Message is a shared location, information about the location.
    Location(Location),
    /// Message is a photo, available sizes of the photo.
    Photo(Vec<PhotoSize>),
    /// Message is a native poll, information about the poll.
    Poll(Poll),
    /// Message is a sticker, information about the sticker.
    Sticker(Sticker),
    /// Message is a forwarded story.
    Story(Story),
    /// Message is a venue, information about the venue.
    Venue(Venue),
    /// Message is a video, information about the video.
    Video(Video),
    /// Message is a video note, information about the video message.
    VideoNote(VideoNote),
    /// Message is a voice message, information about the file.
    Voice(Voice),
    /// Contains arbitrary data for future variants.
    #[serde(untagged)]
    Unknown(Value),
}

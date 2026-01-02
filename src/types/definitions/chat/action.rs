use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

/// Represents a type of an action to tell a user that something is happening on a bot side.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    /// Indicates the bot is choosing a sticker.
    ChooseSticker,
    /// Indicates the bot is finding location data.
    FindLocation,
    /// Indicates the bot is recording a video.
    RecordVideo,
    /// Indicates the bot is recording a voice message.
    RecordVoice,
    /// Indicates the bot is recording a video note.
    RecordVideoNote,
    /// Indicates the bot is typing a text message.
    Typing,
    /// Indicates the bot is uploading a document file.
    UploadDocument,
    /// Indicates the bot is uploading a photo.
    UploadPhoto,
    /// Indicates the bot is uploading a video.
    UploadVideo,
    /// Indicates the bot is uploading a video note.
    UploadVideoNote,
    /// Indicates the bot is uploading a voice message.
    UploadVoice,
}

/// Tells a user that something is happening on a bot side.
///
/// A status is set for 5 seconds or less
/// (when a message arrives from your bot, Telegram clients clear its typing status).
///
/// Example: The ImageBot needs some time to process a request and upload the image.
/// Instead of sending a text message along the lines of “Retrieving image, please wait…”,
/// the bot may use [`SendChatAction`] with [`ChatAction::UploadPhoto`].
/// The user will see a “sending photo” status for the bot.
///
/// We only recommend using this method when a response from the bot
/// will take a noticeable amount of time to arrive.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendChatAction {
    action: ChatAction,
    chat_id: ChatId,
    business_connection_id: Option<String>,
    message_thread_id: Option<Integer>,
}

impl SendChatAction {
    /// Creates a new `SendChatAction`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The unique identifier of the target chat.
    /// * `action` - The type of action to broadcast.
    pub fn new<T>(chat_id: T, action: ChatAction) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            action,
            chat_id: chat_id.into(),
            business_connection_id: None,
            message_thread_id: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the action will be sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }
}

impl Method for SendChatAction {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("sendChatAction", self)
    }
}

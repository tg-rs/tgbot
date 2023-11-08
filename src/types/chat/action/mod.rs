use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

#[cfg(test)]
mod tests;

/// Type of action to tell the user that some is happening on the bot side
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    /// For stickers
    ChooseSticker,
    /// For location data
    FindLocation,
    /// For videos
    RecordVideo,
    /// For voice notes
    RecordVoice,
    /// For video notes
    RecordVideoNote,
    /// For text messages
    Typing,
    /// For general files
    UploadDocument,
    /// For photos
    UploadPhoto,
    /// For videos
    UploadVideo,
    /// For video notes
    UploadVideoNote,
    /// For voice notes
    UploadVoice,
}

/// Tell the user that something is happening on the bot side
///
/// The status is set for 5 seconds or less
/// (when a message arrives from your bot, Telegram clients clear its typing status).
///
/// Example: The ImageBot needs some time to process a request and upload the image.
/// Instead of sending a text message along the lines of “Retrieving image, please wait…”,
/// the bot may use `sendChatAction` with `action = upload_photo`.
/// The user will see a “sending photo” status for the bot.
///
/// We only recommend using this method when a response from the bot
/// will take a noticeable amount of time to arrive.
#[derive(Clone, Debug, Serialize)]
pub struct SendChatAction {
    action: ChatAction,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
}

impl SendChatAction {
    /// Creates a new SendChatAction
    ///
    /// # Arguments
    ///
    /// * chat_id - The unique identifier of the target chat
    /// * action - The type of action to broadcast
    pub fn new<T>(chat_id: T, action: ChatAction) -> Self
    where
        T: Into<ChatId>,
    {
        SendChatAction {
            action,
            chat_id: chat_id.into(),
            message_thread_id: None,
        }
    }

    /// Sets a new message thread ID
    ///
    /// # Arguments
    ///
    /// * value - Unique identifier of the target message thread; supergroups only
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

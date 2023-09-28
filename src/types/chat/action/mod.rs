use serde::{Deserialize, Serialize};

use crate::{method::Method, request::Request, types::ChatId};

#[cfg(test)]
mod tests;

/// Type of action to tell the user that some is happening on the bot's side
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

/// Tell the user that something is happening on the bot's side
///
/// The status is set for 5 seconds or less
/// (when a message arrives from your bot, Telegram clients clear its typing status)
///
/// Example: The ImageBot needs some time to process a request and upload the image
/// Instead of sending a text message along the lines of “Retrieving image, please wait…”,
/// the bot may use sendChatAction with action = upload_photo
/// The user will see a “sending photo” status for the bot
/// We only recommend using this method when a response from the bot
/// will take a noticeable amount of time to arrive
#[derive(Clone, Debug, Serialize)]
pub struct SendChatAction {
    chat_id: ChatId,
    action: ChatAction,
}

impl SendChatAction {
    /// Creates a new SendChatAction
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identitifer for the target chat
    /// * action - Type of action to broadcast
    pub fn new<C: Into<ChatId>>(chat_id: C, action: ChatAction) -> Self {
        SendChatAction {
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl Method for SendChatAction {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("sendChatAction", self)
    }
}

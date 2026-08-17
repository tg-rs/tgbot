use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{ChatId, InputFile},
};

/// Represents a chat photo.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatPhoto {
    /// File identifier of a big (640x640) chat photo.
    ///
    /// Can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of a big (640x640) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
    /// File identifier of a small (160x160) chat photo.
    ///
    /// Can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of a small (160x160) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
}

/// Deletes a chat photo.
///
/// Photos can't be changed for private chats.
/// The bot must be an administrator in the chat for this
/// to work and must have the appropriate admin rights.
///
/// #  Notes
///
/// In regular groups (non-supergroups), this method
/// will only work if the ‘All Members Are Admins’
/// setting is off in the target group.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatPhoto {
    chat_id: ChatId,
}

impl DeleteChatPhoto {
    /// Creates a new `DeleteChatPhoto`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        DeleteChatPhoto {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for DeleteChatPhoto {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("deleteChatPhoto", self)
    }
}

/// Sets a new profile photo for a chat.
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
///
/// # Notes
///
/// In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group.
#[derive(Debug)]
pub struct SetChatPhoto {
    photo: InputFile,
    parameters: SetChatPhotoParameters,
}

impl SetChatPhoto {
    /// Creates a new `SetChatPhoto`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `photo` - New chat photo, uploaded using `multipart/form-data`
    ///   (url and file_id are not supported).
    pub fn new<A, B>(chat_id: A, photo: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        Self {
            photo: photo.into(),
            parameters: SetChatPhotoParameters {
                chat_id: Some(chat_id.into()),
                ..Default::default()
            },
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct SetChatPhotoParameters {
    chat_id: Option<ChatId>,
    photo: Option<String>,
}

impl Method for SetChatPhoto {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self { photo, mut parameters } = self;
        let mut form = Form::default();
        parameters.photo = Some(photo.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("setChatPhoto", form)
    }
}

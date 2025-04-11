use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{ChatId, InputFile},
};

#[cfg(test)]
mod tests;

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

impl ChatPhoto {
    /// Creates a new `ChatPhoto`.
    ///
    /// # Arguments
    ///
    /// * `big_file_id` - File identifier of a big (640x640) chat photo.
    /// * `big_file_unique_id` - Unique file identifier of a big (640x640) chat photo.
    /// * `small_file_id` - File identifier of a small (160x160) chat photo.
    /// * `small_file_unique_id` - Unique file identifier of a small (160x160) chat photo.
    pub fn new<A, B, C, D>(big_file_id: A, big_file_unique_id: B, small_file_id: C, small_file_unique_id: D) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
    {
        Self {
            big_file_id: big_file_id.into(),
            big_file_unique_id: big_file_unique_id.into(),
            small_file_id: small_file_id.into(),
            small_file_unique_id: small_file_unique_id.into(),
        }
    }
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

    fn into_payload(self) -> Payload {
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
    form: Form,
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
            form: Form::from([("chat_id", chat_id.into().into()), ("photo", photo.into().into())]),
        }
    }
}

impl Method for SetChatPhoto {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("setChatPhoto", self.form)
    }
}

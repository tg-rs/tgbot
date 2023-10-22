use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{ChatId, InputFile},
};

#[cfg(test)]
mod tests;

/// Chat photo
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo
    ///
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo
    ///
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

/// Delete a chat photo
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this
/// to work and must have the appropriate admin rights
/// Note: In regular groups (non-supergroups), this method
/// will only work if the ‘All Members Are Admins’
/// setting is off in the target group
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatPhoto {
    chat_id: ChatId,
}

impl DeleteChatPhoto {
    /// Creates a new DeleteChatPhoto
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
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

/// Set a new profile photo for the chat
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group
#[derive(Debug)]
pub struct SetChatPhoto {
    form: Form,
}

impl SetChatPhoto {
    /// Creates a new SetChatPhoto
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * photo - New chat photo, uploaded using multipart/form-data (url and file_id are not supported)
    pub fn new<C, P>(chat_id: C, photo: P) -> Self
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        SetChatPhoto {
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

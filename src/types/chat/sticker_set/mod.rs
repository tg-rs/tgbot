use serde::Serialize;

use crate::{method::Method, request::Request, types::ChatId};

#[cfg(test)]
mod tests;

/// Delete a group sticker set from a supergroup
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
/// Use the field can_set_sticker_set optionally returned
/// in getChat requests to check if the bot can use this method
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatStickerSet {
    chat_id: ChatId,
}

impl DeleteChatStickerSet {
    /// Creates a new DeleteChatStickerSet
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        DeleteChatStickerSet {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for DeleteChatStickerSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("deleteChatStickerSet", self)
    }
}

/// Set a new group sticker set for a supergroup
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Use the field can_set_sticker_set optionally returned in getChat requests
/// to check if the bot can use this method
#[derive(Clone, Debug, Serialize)]
pub struct SetChatStickerSet {
    chat_id: ChatId,
    sticker_set_name: String,
}

impl SetChatStickerSet {
    /// Creates a new SetChatStickerSet
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * sticker_set_name - Name of the sticker set to be set as the group sticker set
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, sticker_set_name: S) -> Self {
        SetChatStickerSet {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl Method for SetChatStickerSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setChatStickerSet", self)
    }
}

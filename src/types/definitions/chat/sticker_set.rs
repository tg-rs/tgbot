use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::ChatId,
};

/// Deletes a group sticker set from a supergroup.
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights.
///
/// Use the field `can_set_sticker_set` optionally returned
/// in [`crate::types::GetChat`] requests to check if the bot can use this method.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatStickerSet {
    chat_id: ChatId,
}

impl DeleteChatStickerSet {
    /// Creates a new `DeleteChatStickerSet`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        DeleteChatStickerSet {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for DeleteChatStickerSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteChatStickerSet", self)
    }
}

/// Sets a new group sticker set for a supergroup.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
///
/// Use the field `can_set_sticker_set` optionally returned in [`crate::types::GetChat`] requests
/// to check if the bot can use this method.
#[derive(Clone, Debug, Serialize)]
pub struct SetChatStickerSet {
    chat_id: ChatId,
    sticker_set_name: String,
}

impl SetChatStickerSet {
    /// Creates a new `SetChatStickerSet`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `sticker_set_name` - Name of the sticker set to be set as the group sticker set.
    pub fn new<A, B>(chat_id: A, sticker_set_name: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        SetChatStickerSet {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl Method for SetChatStickerSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatStickerSet", self)
    }
}

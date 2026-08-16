use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{Integer, KeyboardButton},
};

/// Describes a keyboard button to be used by a user of a Mini App.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button.
    pub id: String,
}

/// Stores a keyboard button that can be used by a user within a Mini App.
#[derive(Clone, Debug, Serialize)]
pub struct SavePreparedKeyboardButton {
    user_id: Integer,
    button: KeyboardButton,
}

impl SavePreparedKeyboardButton {
    /// Creates a new `SavePreparedKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user that can use the button.
    /// * `button` - An object describing the button to be saved;
    ///   the button must be of the type request_users, request_chat, or request_managed_bot.
    pub fn new(user_id: Integer, button: KeyboardButton) -> Self {
        Self { user_id, button }
    }
}

impl Method for SavePreparedKeyboardButton {
    type Response = PreparedKeyboardButton;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("savePreparedKeyboardButton", self)
    }
}

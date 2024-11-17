use serde::{Deserialize, Serialize};

use crate::types::Integer;

#[cfg(test)]
mod tests;

/// Describes an inline message to be sent by a user of a Mini App.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PreparedInlineMessage {
    // Unique identifier of the prepared message.
    pub id: String,
    /// Expiration date of the prepared message, in Unix time.
    /// Expired prepared messages can no longer be used.
    pub expiration_date: Integer,
}

impl PreparedInlineMessage {
    /// Creates a new `PreparedInlineMessage`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the prepared message.
    /// * `expiration_date` - Expiration date of the prepared message, in Unix time.
    pub fn new<T>(id: T, expiration_date: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: id.into(),
            expiration_date,
        }
    }
}

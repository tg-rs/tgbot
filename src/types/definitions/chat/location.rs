use serde::{Deserialize, Serialize};

use crate::types::Location;

/// Represents a location to which a chat is connected.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatLocation {
    /// The location address, defined by the chat owner, with a length of 1-64 characters.
    pub address: String,
    /// The location to which the chat is connected.
    ///
    /// Can't be a live location.
    pub location: Location,
}

impl ChatLocation {
    /// Creates a new `ChatLocation`.
    ///
    /// # Arguments
    ///
    /// * `address` - Location address.
    /// * `location` - The location to which the chat is connected.
    pub fn new<T>(address: T, location: Location) -> Self
    where
        T: Into<String>,
    {
        Self {
            address: address.into(),
            location,
        }
    }
}

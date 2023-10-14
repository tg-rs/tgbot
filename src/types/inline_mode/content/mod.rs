use serde::{Deserialize, Serialize};

use crate::types::{Contact, Location, Text, Venue};

pub use self::{contact::*, invoice::*, location::*, text::*, venue::*};

mod contact;
mod invoice;
mod location;
mod text;
mod venue;

#[cfg(test)]
mod tests;

/// Content of a message to be sent as a result of an inline query
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Contact message
    Contact(InputMessageContentContact),
    /// Invoice message
    Invoice(InputMessageContentInvoice),
    /// Venue message
    Venue(InputMessageContentVenue),
    /// Location message
    Location(InputMessageContentLocation),
    /// Text message
    Text(InputMessageContentText),
}

impl From<Contact> for InputMessageContent {
    fn from(value: Contact) -> Self {
        Self::Contact(value.into())
    }
}

impl From<Location> for InputMessageContent {
    fn from(value: Location) -> Self {
        Self::Location(value.into())
    }
}

impl<T> From<T> for InputMessageContent
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::Text(value.into().into())
    }
}

impl From<Text> for InputMessageContent {
    fn from(value: Text) -> Self {
        Self::Text(value.into())
    }
}

impl From<Venue> for InputMessageContent {
    fn from(value: Venue) -> Self {
        Self::Venue(value.into())
    }
}

use serde::{Deserialize, Serialize};

pub use self::{contact::*, invoice::*, location::*, text::*, venue::*};
use crate::types::{Contact, Location, Text, Venue};

mod contact;
mod invoice;
mod location;
mod text;
mod venue;

#[cfg(test)]
mod tests;

/// Represents a content of a message to be sent as a result of an inline query.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Represents a contact.
    Contact(InputMessageContentContact),
    /// Represents an invoice.
    Invoice(InputMessageContentInvoice),
    /// Represents a venue.
    Venue(InputMessageContentVenue),
    /// Represents a location.
    Location(InputMessageContentLocation),
    /// Represents a text.
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

use serde::{Deserialize, Serialize};

pub use self::{contact::*, invoice::*, location::*, text::*, venue::*};

mod contact;
mod invoice;
mod location;
mod text;
mod venue;

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

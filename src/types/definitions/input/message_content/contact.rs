use serde::{Deserialize, Serialize};

use crate::types::Contact;

/// Represents a contact message to be sent as the result of an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentContact {
    first_name: String,
    phone_number: String,
    last_name: Option<String>,
    vcard: Option<String>,
}

impl InputMessageContentContact {
    /// Creates a new `InputMessageContentContact`.
    ///
    /// # Arguments
    ///
    /// * `first_name` - The first name of the contact.
    /// * `phone_numer` - The phone number of the contact.
    pub fn new<A, B>(first_name: A, phone_number: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - The last name of the contact.
    pub fn with_last_name<T>(mut self, last_name: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(last_name.into());
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, vcard: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(vcard.into());
        self
    }
}

impl From<Contact> for InputMessageContentContact {
    fn from(value: Contact) -> Self {
        Self {
            first_name: value.first_name,
            phone_number: value.phone_number,
            last_name: value.last_name,
            vcard: value.vcard,
        }
    }
}

use serde::Serialize;

#[cfg(test)]
mod tests;

/// Contact message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentContact {
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl InputMessageContentContact {
    /// Creates a new InputMessageContentContact with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * phone_numer - Contact's phone number
    /// * first_name - Contact's first name
    pub fn new<S: Into<String>>(phone_number: S, first_name: S) -> Self {
        InputMessageContentContact {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Contact's last name
    pub fn last_name<S: Into<String>>(mut self, last_name: S) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(mut self, vcard: S) -> Self {
        self.vcard = Some(vcard.into());
        self
    }
}

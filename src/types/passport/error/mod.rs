use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{EncryptedPassportElementType, Integer},
};

#[cfg(test)]
mod tests;

/// Represents an error in the Telegram Passport element
/// which was submitted that should be resolved by the user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PassportElementError {
    #[serde(flatten)]
    error_type: PassportElementErrorType,
}

impl PassportElementError {
    /// Creates a new PassportElementError
    ///
    /// Represents an issue in one of the data fields that was provided by the user.
    /// The error is considered resolved when the field's value changes.
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the error
    /// * field_name - Name of the data field which has the error
    /// * data_hash - Base64-encoded data hash
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::Address`]
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    /// * [`EncryptedPassportElementType::InternalPassport`]
    /// * [`EncryptedPassportElementType::Passport`]
    /// * [`EncryptedPassportElementType::PersonalDetails`]
    pub fn data_field<A, B, C>(
        element_type: EncryptedPassportElementType,
        field_name: A,
        data_hash: B,
        message: C,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            Address | DriverLicense | IdentityCard | InternalPassport | Passport | PersonalDetails => Ok(Self {
                error_type: PassportElementErrorType::DataField {
                    element_type,
                    field_name: field_name.into(),
                    data_hash: data_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with the front side of a document.
    /// The error is considered resolved when the file
    /// with the front side of the document changes.
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the issue
    /// * file_hash - Base64-encoded hash of the file with the front side of the document
    /// * message -  Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    /// * [`EncryptedPassportElementType::InternalPassport`]
    /// * [`EncryptedPassportElementType::Passport`]
    pub fn front_side<A, B>(
        element_type: EncryptedPassportElementType,
        file_hash: A,
        message: B,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(Self {
                error_type: PassportElementErrorType::FrontSide {
                    element_type,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with the reverse side of a document.
    /// The error is considered resolved when the
    /// file with reverse side of the document changes.
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the issue
    /// * file_hash - Base64-encoded hash of the file with the reverse side of the document
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    pub fn reverse_side<A, B>(
        element_type: EncryptedPassportElementType,
        file_hash: A,
        message: B,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            DriverLicense | IdentityCard => Ok(Self {
                error_type: PassportElementErrorType::ReverseSide {
                    element_type,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with the selfie with a document.
    /// The error is considered resolved when the file with the selfie changes.
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the issue
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    /// * [`EncryptedPassportElementType::InternalPassport`]
    /// * [`EncryptedPassportElementType::PassportRegistration`]
    pub fn selfie<A, B>(
        element_type: EncryptedPassportElementType,
        file_hash: A,
        message: B,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(Self {
                error_type: PassportElementErrorType::Selfie {
                    element_type,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }
    /// Creates a new PassportElementError
    ///
    /// Represents an issue with a document scan.
    /// The error is considered resolved when the file with the document scan changes.
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the issue
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::BankStatement`]
    /// * [`EncryptedPassportElementType::PassportRegistration`]
    /// * [`EncryptedPassportElementType::RentalAgreement`]
    /// * [`EncryptedPassportElementType::TemporaryRegistration`]
    /// * [`EncryptedPassportElementType::UtilityBill`]
    pub fn file<A, B>(
        element_type: EncryptedPassportElementType,
        file_hash: A,
        message: B,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => Ok(Self {
                error_type: PassportElementErrorType::File {
                    element_type,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with a list of scans.
    /// The error is considered resolved when the list of files containing the scans changes
    ///
    /// # Arguments
    ///
    /// * element_type - The section of the user's Telegram Passport which has the issue
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::BankStatement`]
    /// * [`EncryptedPassportElementType::PassportRegistration`]
    /// * [`EncryptedPassportElementType::RentalAgreement`]
    /// * [`EncryptedPassportElementType::TemporaryRegistration`]
    /// * [`EncryptedPassportElementType::UtilityBill`]
    pub fn files<A, B, C>(
        element_type: EncryptedPassportElementType,
        file_hashes: A,
        message: C,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
        C: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => Ok(Self {
                error_type: PassportElementErrorType::Files {
                    element_type,
                    file_hashes: file_hashes.into_iter().map(Into::into).collect(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with one of the files that constitute
    /// the translation of a document.
    /// The error is considered resolved when the file changes.
    ///
    /// # Arguments
    ///
    /// * element_type - Type of element of the user's Telegram Passport which has the issue
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::BankStatement`]
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    /// * [`EncryptedPassportElementType::InternalPassport`]
    /// * [`EncryptedPassportElementType::Passport`]
    /// * [`EncryptedPassportElementType::PassportRegistration`]
    /// * [`EncryptedPassportElementType::RentalAgreement`]
    /// * [`EncryptedPassportElementType::TemporaryRegistration`]
    /// * [`EncryptedPassportElementType::UtilityBill`]
    pub fn translation_file<A, B>(
        element_type: EncryptedPassportElementType,
        file_hash: A,
        message: B,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: Into<String>,
        B: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(Self {
                error_type: PassportElementErrorType::TranslationFile {
                    element_type,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue with the translated version of a document.
    /// The error is considered resolved when a file with the document translation change.
    ///
    /// # Arguments
    ///
    /// * element_type - Type of element of the user's Telegram Passport which has the issue
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    ///
    /// Supported element types:
    ///
    /// * [`EncryptedPassportElementType::BankStatement`]
    /// * [`EncryptedPassportElementType::DriverLicense`]
    /// * [`EncryptedPassportElementType::IdentityCard`]
    /// * [`EncryptedPassportElementType::InternalPassport`]
    /// * [`EncryptedPassportElementType::Passport`]
    /// * [`EncryptedPassportElementType::PassportRegistration`]
    /// * [`EncryptedPassportElementType::RentalAgreement`]
    /// * [`EncryptedPassportElementType::TemporaryRegistration`]
    /// * [`EncryptedPassportElementType::UtilityBill`]
    pub fn translation_files<A, B, C>(
        element_type: EncryptedPassportElementType,
        file_hashes: A,
        message: C,
    ) -> Result<Self, UnexpectedEncryptedPassportElementType>
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
        C: Into<String>,
    {
        use self::EncryptedPassportElementType::*;
        match element_type {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(Self {
                error_type: PassportElementErrorType::TranslationFiles {
                    element_type,
                    file_hashes: file_hashes.into_iter().map(Into::into).collect(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementType(element_type)),
        }
    }

    /// Creates a new PassportElementError
    ///
    /// Represents an issue in an unspecified place.
    /// The error is considered resolved when new data is added.
    ///
    /// # Arguments
    ///
    /// * element_type - Type of element of the user's Telegram Passport which has the issue
    /// * element_hash - Base64-encoded element hash
    /// * message - Error message
    ///
    /// Accepts any variant of [`EncryptedPassportElementType`].
    pub fn unspecified<A, B>(element_type: EncryptedPassportElementType, element_hash: A, message: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            error_type: PassportElementErrorType::Unspecified {
                element_type,
                element_hash: element_hash.into(),
                message: message.into(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "source")]
enum PassportElementErrorType {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        field_name: String,
        data_hash: String,
        message: String,
    },
    #[serde(rename = "front_side")]
    FrontSide {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "reverse_side")]
    ReverseSide {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "selfie")]
    Selfie {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "files")]
    Files {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "translation_file")]
    TranslationFile {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "translation_files")]
    TranslationFiles {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "unspecified")]
    Unspecified {
        #[serde(rename = "type")]
        element_type: EncryptedPassportElementType,
        element_hash: String,
        message: String,
    },
}

/// Got an unexpected encrypted passport element type
#[derive(Clone, Debug)]
pub struct UnexpectedEncryptedPassportElementType(EncryptedPassportElementType);

impl Error for UnexpectedEncryptedPassportElementType {}

impl fmt::Display for UnexpectedEncryptedPassportElementType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "unexpected element type: {:?}", self.0)
    }
}

/// Informs a user that some of the Telegram Passport elements they provided contains errors
///
/// The user will not be able to re-submit their Passport to you until the errors are fixed.
/// The contents of the field for which you returned the error must change.
///
/// Use this if the data submitted by the user doesn't satisfy the standards
/// your service requires for any reason.
///
/// For example, if a birthday date seems invalid, a submitted document is blurry,
/// a scan shows evidence of tampering, etc.
///
/// Supply some details in the error message to make sure the user knows how to correct the issues.
#[derive(Clone, Debug, Serialize)]
pub struct SetPassportDataErrors {
    user_id: Integer,
    errors: Vec<PassportElementError>,
}

impl SetPassportDataErrors {
    /// Creates a new SetPassportDataErrors
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier
    /// * errors - Array describing the errors
    pub fn new<T>(user_id: Integer, errors: T) -> Self
    where
        T: IntoIterator<Item = PassportElementError>,
    {
        SetPassportDataErrors {
            user_id,
            errors: errors.into_iter().collect(),
        }
    }
}

impl Method for SetPassportDataErrors {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setPassportDataErrors", self)
    }
}

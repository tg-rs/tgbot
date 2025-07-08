use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// Represents a file uploaded to Telegram Passport.
///
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PassportFile {
    /// A unix timestamp when a file was uploaded.
    pub file_date: Integer,
    /// An identifier for a file, which can be used to download or reuse the file.
    pub file_id: String,
    /// A file size in bytes.
    pub file_size: Integer,
    /// A unique identifier for a file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse a file.
    pub file_unique_id: String,
}

impl PassportFile {
    /// Creates a new `PassportFile`.
    ///
    /// # Arguments
    ///
    /// * `file_date` - A unix time when a file was uploaded.
    /// * `file_id` - An identifier for a file.
    /// * `file_size` - A file size in bytes.
    /// * `file_unique_id` - A unique identifier for a file.
    pub fn new<A, B>(file_date: Integer, file_id: A, file_size: Integer, file_unique_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            file_date,
            file_id: file_id.into(),
            file_size,
            file_unique_id: file_unique_id.into(),
        }
    }
}

/// Represents a data required for decrypting and authenticating [`EncryptedPassportElement`].
///
/// See the [Telegram Passport Documentation][1] for a complete description
/// of the data decryption and authentication processes.
///
/// [1]: https://core.telegram.org/passport#receiving-information
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedCredentials {
    /// A base64-encoded encrypted JSON-serialized data
    /// with unique user's payload,
    /// data hashes and secrets required
    /// for [`EncryptedPassportElement`] decryption and authentication.
    pub data: String,
    /// A base64-encoded data hash for data authentication.
    pub hash: String,
    /// A base64-encoded secret, encrypted
    /// with the bot public RSA key,
    /// required for data decryption.
    pub secret: String,
}

impl EncryptedCredentials {
    /// Creates a new `EncryptedCredentials`.
    ///
    /// # Arguments
    ///
    /// * `data` - A unique payload.
    /// * `hash` - A hash for data authentication.
    /// * `secret` - A secret for data decryption.
    pub fn new<A, B, C>(data: A, hash: B, secret: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            data: data.into(),
            hash: hash.into(),
            secret: secret.into(),
        }
    }
}

/// Represents an address.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementAddress {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by the user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
}

impl EncryptedPassportElementAddress {
    /// Creates a new `EncryptedPassportElementAddress`
    ///
    /// # Arguments
    ///
    /// * `data` - A data provided by a user.
    /// * `hash` - An element hash.
    pub fn new<A, B>(data: A, hash: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            hash: hash.into(),
        }
    }
}

/// Represents a bank statement.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementBankStatement {
    /// An array of encrypted files with
    /// documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementBankStatement {
    /// Creates a new `EncryptedPassportElementBankStatement`.
    ///
    /// # Arguments
    ///
    /// * `files` - An array of encrypted files with documents.
    /// * `hash` - An element hash.
    pub fn new<A, B>(files: A, hash: B) -> Self
    where
        A: IntoIterator<Item = PassportFile>,
        B: Into<String>,
    {
        Self {
            files: files.into_iter().collect(),
            hash: hash.into(),
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a driver license.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementDriverLicense {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by a user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// An encrypted file with a front side
    /// of a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An encrypted file with a selfie of a user
    /// holding a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// An encrypted file with a reverse side of a document,
    /// provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub reverse_side: PassportFile,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementDriverLicense {
    /// Creates a new `EncryptedPassportElementDriverLicense`.
    ///
    /// # Arguments
    ///
    /// * `data` - An encrypted data provided by a user.
    /// * `hash` - An element hash.
    /// * `front_side` - An encrypted file with a front side of a document.
    /// * `reverse_side` - An encrypted file with a reverse side of a document.
    /// * `selfie` - An encrypted file with a selfie of a user.
    pub fn new<A, B>(
        data: A,
        hash: B,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            front_side,
            hash: hash.into(),
            selfie,
            reverse_side,
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an E-Mail.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementEmail {
    /// A user's verified email address.
    pub email: String,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
}

impl EncryptedPassportElementEmail {
    /// Creates a new `EncryptedPassportElementEmail`.
    ///
    /// # Arguments
    ///
    /// * `email` - A user's verified email address.
    /// * `hash` - An element hash.
    pub fn new<A, B>(email: A, hash: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            email: email.into(),
            hash: hash.into(),
        }
    }
}

/// Represents an identity card.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde_with::skip_serializing_none]
pub struct EncryptedPassportElementIdentityCard {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by a user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// An encrypted file with a front side
    /// of a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An encrypted file with a reverse side of a document,
    /// provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub reverse_side: PassportFile,
    /// An encrypted file with a selfie of a user
    /// holding a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementIdentityCard {
    /// Creates a new `EncryptedPassportElementIdentityCard`.
    ///
    /// # Arguments
    ///
    /// * `data` - An encrypted data provided by a user.
    /// * `hash` - An element hash.
    /// * `front_side` - An encrypted file with a front side of a document.
    /// * `reverse_side` - An encrypted file with a reverse side of a document.
    /// * `selfie` - An encrypted file with a selfie of a user.
    pub fn new<A, B>(
        data: A,
        hash: B,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            front_side,
            hash: hash.into(),
            reverse_side,
            selfie,
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an internal passport.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementInternalPassport {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by a user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// An encrypted file with a front side
    /// of a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An encrypted file with a selfie of a user
    /// holding a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementInternalPassport {
    /// Creates a new `EncryptedPassportElementInternalPassport`.
    ///
    /// # Arguments
    ///
    /// * `data` - An encrypted data provided by a user.
    /// * `hash` - An element hash.
    /// * `front_side` - An encrypted file with a front side of a document.
    /// * `selfie` - An Encrypted file with a selfie of a user.
    pub fn new<A, B>(data: A, hash: B, front_side: PassportFile, selfie: PassportFile) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            front_side,
            hash: hash.into(),
            selfie,
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a passport.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPassport {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by a user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// An encrypted file with a front side
    /// of the document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An encrypted file with a selfie of a user
    /// holding a document, provided by a user.
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementPassport {
    /// Creates a new `EncryptedPassportElementPassport`.
    ///
    /// # Arguments
    ///
    /// * `data` - An encrypted data provided by a user.
    /// * `hash` - An element hash.
    /// * `front_side` - An encrypted file with a front side of a document.
    /// * `selfie` - An encrypted file with a selfie of a user.
    pub fn new<A, B>(data: A, hash: B, front_side: PassportFile, selfie: PassportFile) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            front_side,
            hash: hash.into(),
            selfie,
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a passport registration.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPassportRegistration {
    /// Array of encrypted files with
    /// documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementPassportRegistration {
    /// Creates a new `EncryptedPassportElementPassportRegistration`.
    ///
    /// # Arguments
    ///
    /// * `files` - An array of encrypted files with documents.
    /// * `hash` - An element hash.
    pub fn new<A, B>(files: A, hash: B) -> Self
    where
        A: IntoIterator<Item = PassportFile>,
        B: Into<String>,
    {
        Self {
            files: files.into_iter().collect(),
            hash: hash.into(),
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents personal details.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPersonalDetails {
    /// A base64-encoded encrypted
    /// Telegram Passport element data provided by a user.
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
}

impl EncryptedPassportElementPersonalDetails {
    /// Creates a new `EncryptedPassportElementPersonalDetails`.
    ///
    /// # Arguments
    ///
    /// * `data` - An encrypted data provided by a user.
    /// * `hash` - An element hash.
    pub fn new<A, B>(data: A, hash: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: data.into(),
            hash: hash.into(),
        }
    }
}

/// Represents a phone number.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPhoneNumber {
    /// A base64-encoded element hash for.
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// A user's verified phone number.
    pub phone_number: String,
}

impl EncryptedPassportElementPhoneNumber {
    /// Creates a new `EncryptedPassportElementPhoneNumber`.
    ///
    /// # Arguments
    ///
    /// * `hash` - An element hash.
    /// * `phone_number` - A user's verified phone number.
    pub fn new<A, B>(hash: A, phone_number: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            hash: hash.into(),
            phone_number: phone_number.into(),
        }
    }
}

/// Represents a rental agreement.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementRentalAgreement {
    /// An array of encrypted files with
    /// documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// A base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// An array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementRentalAgreement {
    /// Creates a new `EncryptedPassportElementRentalAgreement`.
    ///
    /// # Arguments
    ///
    /// * `files` - An array of encrypted files with documents.
    /// * `hash` - An element hash.
    pub fn new<A, B>(files: A, hash: B) -> Self
    where
        A: IntoIterator<Item = PassportFile>,
        B: Into<String>,
    {
        Self {
            files: files.into_iter().collect(),
            hash: hash.into(),
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a temporary registration.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with
    /// documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementTemporaryRegistration {
    /// Creates a new `EncryptedPassportElementTemporaryRegistration`.
    ///
    /// # Arguments
    ///
    /// * `files` - An array of encrypted files with documents.
    /// * `hash` - An element hash.
    pub fn new<A, B>(files: A, hash: B) -> Self
    where
        A: IntoIterator<Item = PassportFile>,
        B: Into<String>,
    {
        Self {
            files: files.into_iter().collect(),
            hash: hash.into(),
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an utility bill.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with
    /// documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`].
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by a user.
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementUtilityBill {
    /// Creates a new `EncryptedPassportElementUtilityBill`.
    ///
    /// # Arguments
    ///
    /// * `files` - An array of encrypted files with documents.
    /// * `hash` - An element hash.
    pub fn new<A, B>(files: A, hash: B) -> Self
    where
        A: IntoIterator<Item = PassportFile>,
        B: Into<String>,
    {
        Self {
            files: files.into_iter().collect(),
            hash: hash.into(),
            translation: None,
        }
    }

    /// Sets a new translation.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of encrypted files with translated versions of documents.
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an information about documents
/// or other Telegram Passport elements shared with a bot by a user.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum EncryptedPassportElement {
    /// Represents an address.
    Address(EncryptedPassportElementAddress),
    /// Represents a bank statement.
    BankStatement(EncryptedPassportElementBankStatement),
    /// Represents a driver license.
    DriverLicense(EncryptedPassportElementDriverLicense),
    /// Represents an E-Mail.
    Email(EncryptedPassportElementEmail),
    /// Represents an identity card.
    IdentityCard(EncryptedPassportElementIdentityCard),
    /// Represents an internal passport.
    InternalPassport(EncryptedPassportElementInternalPassport),
    /// Represents a passport.
    Passport(EncryptedPassportElementPassport),
    /// Represents a passport registration.
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    /// Represents personal details.
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    /// Represents a phone number.
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    /// Represents a rental agreement.
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    /// Represents a temporary registration.
    TemporaryRegistration(EncryptedPassportElementTemporaryRegistration),
    /// Represents a utility bill.
    UtilityBill(EncryptedPassportElementUtilityBill),
}

/// Represents a type of an encrypted passport element.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    /// Address.
    Address,
    /// Bank statement.
    BankStatement,
    /// Driver license.
    DriverLicense,
    /// E-Mail.
    Email,
    /// Identity card.
    IdentityCard,
    /// Internal passport.
    InternalPassport,
    /// Passport.
    Passport,
    /// Passport registration.
    PassportRegistration,
    /// Personal details.
    PersonalDetails,
    /// Phone number.
    PhoneNumber,
    /// Rental agreement.
    RentalAgreement,
    /// Temporary registration.
    TemporaryRegistration,
    /// Utility bill.
    UtilityBill,
}

/// Represents a telegram Passport data shared with a bot by a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PassportData {
    /// An encrypted credentials required to decrypt the data.
    pub credentials: EncryptedCredentials,
    /// An array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with a bot.
    pub data: Vec<EncryptedPassportElement>,
}

impl PassportData {
    /// Creates a new `PassportData`.
    ///
    /// # Arguments
    ///
    /// * `credentials` - An Encrypted credentials required to decrypt the data.
    /// * `data` - An array with information about documents.
    pub fn new<T>(credentials: EncryptedCredentials, data: T) -> Self
    where
        T: IntoIterator<Item = EncryptedPassportElement>,
    {
        Self {
            credentials,
            data: data.into_iter().collect(),
        }
    }
}

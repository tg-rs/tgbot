use serde::{Deserialize, Serialize};

use crate::types::Integer;

#[cfg(test)]
mod tests;

/// Represents a file uploaded to Telegram Passport
///
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PassportFile {
    /// Unix time when the file was uploaded
    pub file_date: Integer,
    /// Identifier for the file, which can be used to download or reuse the file
    pub file_id: String,
    /// File size in bytes
    pub file_size: Integer,
    /// Unique identifier for the file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
}

impl PassportFile {
    /// Creates a new PassportFile
    ///
    /// # Arguments
    ///
    /// * file_date - Unix time when the file was uploaded
    /// * file_id - Identifier for the file
    /// * file_size - File size in bytes
    /// * file_unique_id - Unique identifier for the file
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

/// Represents a data required for decrypting and authenticating [`EncryptedPassportElement`]
///
/// See the [Telegram Passport Documentation][1] for a complete description
/// of the data decryption and authentication processes.
///
/// [1]: https://core.telegram.org/passport#receiving-information
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data
    /// with unique user's payload,
    /// data hashes and secrets required
    /// for [`EncryptedPassportElement`] decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted
    /// with the bot public RSA key,
    /// required for data decryption
    pub secret: String,
}

impl EncryptedCredentials {
    /// Creates a new EncryptedCredentials
    ///
    /// # Arguments
    ///
    /// * data - Unique payload
    /// * hash - Hash for data authentication
    /// * secret - Secret for data decryption
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

/// Represents an address
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementAddress {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
}

impl EncryptedPassportElementAddress {
    /// Creates a new EncryptedPassportElementAddress
    ///
    /// # Arguments
    ///
    /// * data - Data provided by the user
    /// * hash - Element hash
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

/// Represents a bank statement
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementBankStatement {
    /// Array of encrypted files with
    /// documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementBankStatement {
    /// Creates a new EncryptedPassportElementBankStatement
    ///
    /// # Arguments
    ///
    /// * files - An array of encrypted files with documents
    /// * hash - Element hash
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a driver license
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementDriverLicense {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub reverse_side: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementDriverLicense {
    /// Creates a new EncryptedPassportElementDriverLicense
    ///
    /// # Arguments
    ///
    /// * data - Encrypted data provided by the user
    /// * hash - Element hash
    /// * front_side - Encrypted file with the front side of the document
    /// * reverse_side - Encrypted file with the reverse side of the document
    /// * selfie - Encrypted file with the selfie of the user
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an E-Mail
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address
    pub email: String,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
}

impl EncryptedPassportElementEmail {
    /// Creates a new EncryptedPassportElementEmail
    ///
    /// # Arguments
    ///
    /// * email - User's verified email address
    /// * hash - Element hash
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

/// Represents an identity card
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementIdentityCard {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementIdentityCard {
    /// Creates a new EncryptedPassportElementIdentityCard
    ///
    /// # Arguments
    ///
    /// * data - Encrypted data provided by the user
    /// * hash - Element hash
    /// * front_side - Encrypted file with the front side of the document
    /// * reverse_side - Encrypted file with the reverse side of the document
    /// * selfie - Encrypted file with the selfie of the user
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an internal passport
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementInternalPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementInternalPassport {
    /// Creates a new EncryptedPassportElementInternalPassport
    ///
    /// # Arguments
    ///
    /// * data - Encrypted data provided by the user
    /// * hash - Element hash
    /// * front_side - Encrypted file with the front side of the document
    /// * selfie - Encrypted file with the selfie of the user
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a passport
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub front_side: PassportFile,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    ///
    /// The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementPassport {
    /// Creates a new EncryptedPassportElementPassport
    ///
    /// # Arguments
    ///
    /// * data - Encrypted data provided by the user
    /// * hash - Element hash
    /// * front_side - Encrypted file with the front side of the document
    /// * selfie - Encrypted file with the selfie of the user
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a passport registration
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPassportRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementPassportRegistration {
    /// Creates a new EncryptedPassportElementPassportRegistration
    ///
    /// # Arguments
    ///
    /// * files - An array of encrypted files with documents
    /// * hash - Element hash
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents personal details
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPersonalDetails {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    ///
    /// Can be decrypted and verified using
    /// the accompanying [`EncryptedCredentials`].
    pub data: String,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
}

impl EncryptedPassportElementPersonalDetails {
    /// Creates a new EncryptedPassportElementPersonalDetails
    ///
    /// # Arguments
    ///
    /// * data - Encrypted data provided by the user
    /// * hash - Element hash
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

/// Represents a phone number
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementPhoneNumber {
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// User's verified phone number
    pub phone_number: String,
}

impl EncryptedPassportElementPhoneNumber {
    /// Creates a new EncryptedPassportElementPhoneNumber
    ///
    /// # Arguments
    ///
    /// * hash - Element hash
    /// * phone_number - User's verified phone number
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

/// Represents a rental agreement
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementRentalAgreement {
    /// Array of encrypted files with
    /// documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementRentalAgreement {
    /// Creates a new EncryptedPassportElementRentalAgreement
    ///
    /// # Arguments
    ///
    /// * files - An array of encrypted files with documents
    /// * hash - Element hash
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents a temporary registration
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementTemporaryRegistration {
    /// Creates a new EncryptedPassportElementTemporaryRegistration
    ///
    /// # Arguments
    ///
    /// * files - An array of encrypted files with documents
    /// * hash - Element hash
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an utility bill
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with
    /// documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    pub files: Vec<PassportFile>,
    /// Base64-encoded element hash for
    /// using in [`crate::types::PassportElementError::unspecified`]
    pub hash: String,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    ///
    /// Files can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl EncryptedPassportElementUtilityBill {
    /// Creates a new EncryptedPassportElementUtilityBill
    ///
    /// # Arguments
    ///
    /// * files - An array of encrypted files with documents
    /// * hash - Element hash
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

    /// Sets a new translation
    ///
    /// # Arguments
    ///
    /// * value - An array of encrypted files with translated versions of documents
    pub fn with_translation<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PassportFile>,
    {
        self.translation = Some(value.into_iter().collect());
        self
    }
}

/// Represents an information about documents
/// or other Telegram Passport elements shared with the bot by the user
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum EncryptedPassportElement {
    /// Address
    Address(EncryptedPassportElementAddress),
    /// Bank statement
    BankStatement(EncryptedPassportElementBankStatement),
    /// Driver license
    DriverLicense(EncryptedPassportElementDriverLicense),
    /// E-Mail
    Email(EncryptedPassportElementEmail),
    /// Identity card
    IdentityCard(EncryptedPassportElementIdentityCard),
    /// Internal passport
    InternalPassport(EncryptedPassportElementInternalPassport),
    /// Passport
    Passport(EncryptedPassportElementPassport),
    /// Passport registration
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    /// Personal details
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    /// Phone number
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    /// Rental agreement
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    /// Temporary registration
    TemporaryRegistration(EncryptedPassportElementTemporaryRegistration),
    /// Utility bill
    UtilityBill(EncryptedPassportElementUtilityBill),
}

/// Represents a type of an encrypted passport element
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    /// Address
    Address,
    /// Bank statement
    BankStatement,
    /// Driver license
    DriverLicense,
    /// E-Mail
    Email,
    /// Identity card
    IdentityCard,
    /// Internal passport
    InternalPassport,
    /// Passport
    Passport,
    /// Passport registration
    PassportRegistration,
    /// Personal details
    PersonalDetails,
    /// Phone number
    PhoneNumber,
    /// Rental agreement
    RentalAgreement,
    /// Temporary registration
    TemporaryRegistration,
    /// Utility bill
    UtilityBill,
}

/// Represents a telegram Passport data shared with the bot by the user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PassportData {
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
    /// Array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
}

impl PassportData {
    /// Creates a new PassportData
    ///
    /// # Arguments
    ///
    /// * credentials - Encrypted credentials required to decrypt the data
    /// * data - Array with information about documents
    pub fn new<T>(credentials: EncryptedCredentials, data: T) -> Self
    where
        T: IntoIterator<Item = EncryptedPassportElement>,
    {
        Self {
            credentials,
            data: data.into_iter().map(Into::into).collect(),
        }
    }
}

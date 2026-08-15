#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([(
        "set-passport-data-errors",
        SetPassportDataErrors::new(
            1,
            [
                PassportElementError::data_field(
                    EncryptedPassportElementType::Address,
                    "address",
                    "data_hash",
                    "bad address",
                )
                .unwrap(),
                PassportElementError::front_side(EncryptedPassportElementType::DriverLicense, "file_hash", "bad file")
                    .unwrap(),
                PassportElementError::reverse_side(
                    EncryptedPassportElementType::DriverLicense,
                    "file_hash",
                    "bad file",
                )
                .unwrap(),
                PassportElementError::selfie(EncryptedPassportElementType::DriverLicense, "file_hash", "bad file")
                    .unwrap(),
                PassportElementError::file(EncryptedPassportElementType::BankStatement, "file_hash", "bad file")
                    .unwrap(),
                PassportElementError::files(
                    EncryptedPassportElementType::BankStatement,
                    vec![String::from("file_hash")],
                    "bad file",
                )
                .unwrap(),
                PassportElementError::translation_file(
                    EncryptedPassportElementType::BankStatement,
                    "file_hash",
                    "bad file",
                )
                .unwrap(),
                PassportElementError::translation_files(
                    EncryptedPassportElementType::BankStatement,
                    vec![String::from("file_hash")],
                    "bad file",
                )
                .unwrap(),
                PassportElementError::unspecified(EncryptedPassportElementType::Passport, "test-hash", "test-message"),
            ],
        ),
        |x| assert!(x),
    )])
    .await;
}

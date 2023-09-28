use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{EncryptedPassportElementKind, PassportElementError, SetPassportDataErrors},
};

#[test]
fn serialize_error() {
    let err = PassportElementError::data_field(
        EncryptedPassportElementKind::Address,
        "address",
        "data_hash",
        "bad address",
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"data","type":"address","field_name":"address","data_hash":"data_hash","message":"bad address"})
    );

    let err =
        PassportElementError::front_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file").unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"front_side","type":"driver_license","file_hash":"file_hash","message":"bad file"})
    );

    let err = PassportElementError::reverse_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file")
        .unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"reverse_side","type":"driver_license","file_hash":"file_hash","message":"bad file"})
    );

    let err =
        PassportElementError::selfie(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file").unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"selfie","type":"driver_license","file_hash":"file_hash","message":"bad file"})
    );

    let err = PassportElementError::file(EncryptedPassportElementKind::BankStatement, "file_hash", "bad file").unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"file","type":"bank_statement","file_hash":"file_hash","message":"bad file"})
    );

    let err = PassportElementError::files(
        EncryptedPassportElementKind::BankStatement,
        vec![String::from("file_hash")],
        "bad file",
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"})
    );

    let err =
        PassportElementError::translation_file(EncryptedPassportElementKind::BankStatement, "file_hash", "bad file")
            .unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"translation_file","type":"bank_statement","file_hash":"file_hash","message":"bad file"})
    );

    let err = PassportElementError::translation_files(
        EncryptedPassportElementKind::BankStatement,
        vec![String::from("file_hash")],
        "bad file",
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"translation_files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"})
    );

    let err =
        PassportElementError::unspecified(EncryptedPassportElementKind::BankStatement, "element_hash", "bad file");
    assert_eq!(
        serde_json::to_value(&err).unwrap(),
        serde_json::json!({"source":"unspecified","type":"bank_statement","element_hash":"element_hash","message":"bad file"})
    );
}

#[test]
fn create_error_accepts_kind() {
    use self::EncryptedPassportElementKind::*;
    for (kind, flag) in &[
        (Address, true),
        (BankStatement, false),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, true),
        (Passport, true),
        (PassportRegistration, false),
        (PersonalDetails, true),
        (PhoneNumber, false),
        (RentalAgreement, false),
        (TemporaryRegistration, false),
        (UtilityBill, false),
    ] {
        let err = PassportElementError::data_field(*kind, "address", "data_hash", "bad address");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, false),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, true),
        (Passport, true),
        (PassportRegistration, false),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, false),
        (TemporaryRegistration, false),
        (UtilityBill, false),
    ] {
        let err = PassportElementError::front_side(*kind, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, false),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, false),
        (Passport, false),
        (PassportRegistration, false),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, false),
        (TemporaryRegistration, false),
        (UtilityBill, false),
    ] {
        let err = PassportElementError::reverse_side(*kind, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, false),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, true),
        (Passport, true),
        (PassportRegistration, false),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, false),
        (TemporaryRegistration, false),
        (UtilityBill, false),
    ] {
        let err = PassportElementError::selfie(*kind, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, true),
        (DriverLicense, false),
        (Email, false),
        (IdentityCard, false),
        (InternalPassport, false),
        (Passport, false),
        (PassportRegistration, true),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, true),
        (TemporaryRegistration, true),
        (UtilityBill, true),
    ] {
        let err = PassportElementError::file(*kind, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, true),
        (DriverLicense, false),
        (Email, false),
        (IdentityCard, false),
        (InternalPassport, false),
        (Passport, false),
        (PassportRegistration, true),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, true),
        (TemporaryRegistration, true),
        (UtilityBill, true),
    ] {
        let err = PassportElementError::files(*kind, vec![String::from("file_hash")], "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, true),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, true),
        (Passport, true),
        (PassportRegistration, true),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, true),
        (TemporaryRegistration, true),
        (UtilityBill, true),
    ] {
        let err = PassportElementError::translation_file(*kind, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (kind, flag) in &[
        (Address, false),
        (BankStatement, true),
        (DriverLicense, true),
        (Email, false),
        (IdentityCard, true),
        (InternalPassport, true),
        (Passport, true),
        (PassportRegistration, true),
        (PersonalDetails, false),
        (PhoneNumber, false),
        (RentalAgreement, true),
        (TemporaryRegistration, true),
        (UtilityBill, true),
    ] {
        let err = PassportElementError::translation_files(*kind, vec![String::from("file_hash")], "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }
}

#[test]
fn set_passport_data_errors() {
    let request = SetPassportDataErrors::new(1, vec![]).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setPassportDataErrors"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["user_id"], 1);
        assert!(data["errors"].as_array().unwrap().is_empty());
    } else {
        panic!("Unexpected request body");
    }
}

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, EncryptedPassportElementKind, PassportElementError, SetPassportDataErrors},
};

#[test]
fn passport_element_error() {
    for (expected_struct, expected_value) in [
        (
            PassportElementError::data_field(
                EncryptedPassportElementKind::Address,
                "address",
                "data_hash",
                "bad address",
            ),
            serde_json::json!({
                "source": "data",
                "type": "address",
                "field_name": "address",
                "data_hash": "data_hash",
                "message": "bad address"
            }),
        ),
        (
            PassportElementError::front_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file"),
            serde_json::json!({
                "source": "front_side",
                "type": "driver_license",
                "file_hash": "file_hash",
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::reverse_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file"),
            serde_json::json!({
                "source": "reverse_side",
                "type": "driver_license",
                "file_hash": "file_hash",
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::selfie(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file"),
            serde_json::json!({
                "source": "selfie",
                "type": "driver_license",
                "file_hash": "file_hash",
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::file(EncryptedPassportElementKind::BankStatement, "file_hash", "bad file"),
            serde_json::json!({
                "source": "file",
                "type": "bank_statement",
                "file_hash": "file_hash",
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::files(
                EncryptedPassportElementKind::BankStatement,
                vec![String::from("file_hash")],
                "bad file",
            ),
            serde_json::json!({
                "source": "files",
                "type": "bank_statement",
                "file_hashes": ["file_hash"],
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::translation_file(
                EncryptedPassportElementKind::BankStatement,
                "file_hash",
                "bad file",
            ),
            serde_json::json!({
                "source": "translation_file",
                "type": "bank_statement",
                "file_hash": "file_hash",
                "message": "bad file"
            }),
        ),
        (
            PassportElementError::translation_files(
                EncryptedPassportElementKind::BankStatement,
                vec![String::from("file_hash")],
                "bad file",
            ),
            serde_json::json!({
                "source": "translation_files",
                "type": "bank_statement",
                "file_hashes": ["file_hash"],
                "message": "bad file"
            }),
        ),
    ] {
        assert_json_eq(expected_struct.unwrap(), expected_value);
    }

    assert_json_eq(
        PassportElementError::unspecified(EncryptedPassportElementKind::BankStatement, "element_hash", "bad file"),
        serde_json::json!({
            "source": "unspecified",
            "type": "bank_statement",
            "element_hash": "element_hash",
            "message": "bad file"
        }),
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
    assert_payload_eq(
        Payload::json(
            "setPassportDataErrors",
            serde_json::json!({
                "user_id": 1,
                "errors": []
            }),
        ),
        SetPassportDataErrors::new(1, vec![]),
    );
}

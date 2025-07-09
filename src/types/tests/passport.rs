use crate::types::*;

#[test]
fn passport_data() {
    let expected_struct = PassportData::new(
        EncryptedCredentials::new("d", "h", "s"),
        [
            EncryptedPassportElementAddress::new("d", "h").into(),
            EncryptedPassportElementBankStatement::new([PassportFile::new(0, "f", 1, "uf")], "h")
                .with_translation([PassportFile::new(0, "f", 1, "uf")])
                .into(),
            EncryptedPassportElementDriverLicense::new(
                "d",
                "h",
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
            )
            .with_translation([PassportFile::new(0, "f", 1, "uf")])
            .into(),
            EncryptedPassportElementEmail::new("u@h.z", "h").into(),
            EncryptedPassportElementIdentityCard::new(
                "d",
                "h",
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
            )
            .with_translation([PassportFile::new(0, "f", 1, "uf")])
            .into(),
            EncryptedPassportElementInternalPassport::new(
                "d",
                "h",
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
            )
            .with_translation([PassportFile::new(0, "f", 1, "uf")])
            .into(),
            EncryptedPassportElementPassport::new(
                "d",
                "h",
                PassportFile::new(0, "f", 1, "uf"),
                PassportFile::new(0, "f", 1, "uf"),
            )
            .with_translation([PassportFile::new(0, "f", 1, "uf")])
            .into(),
            EncryptedPassportElementPassportRegistration::new([PassportFile::new(0, "f", 1, "uf")], "h")
                .with_translation([PassportFile::new(0, "f", 1, "uf")])
                .into(),
            EncryptedPassportElementPersonalDetails::new("d", "h").into(),
            EncryptedPassportElementPhoneNumber::new("h", "+79270000000").into(),
            EncryptedPassportElementRentalAgreement::new([PassportFile::new(0, "f", 1, "uf")], "h")
                .with_translation([PassportFile::new(0, "f", 1, "uf")])
                .into(),
            EncryptedPassportElementTemporaryRegistration::new([PassportFile::new(0, "f", 1, "uf")], "h")
                .with_translation([PassportFile::new(0, "f", 1, "uf")])
                .into(),
            EncryptedPassportElementUtilityBill::new([PassportFile::new(0, "f", 1, "uf")], "h")
                .with_translation([PassportFile::new(0, "f", 1, "uf")])
                .into(),
        ],
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn passport_element_error() {
    for value in [
        PassportElementError::data_field(
            EncryptedPassportElementType::Address,
            "address",
            "data_hash",
            "bad address",
        ),
        PassportElementError::front_side(EncryptedPassportElementType::DriverLicense, "file_hash", "bad file"),
        PassportElementError::reverse_side(EncryptedPassportElementType::DriverLicense, "file_hash", "bad file"),
        PassportElementError::selfie(EncryptedPassportElementType::DriverLicense, "file_hash", "bad file"),
        PassportElementError::file(EncryptedPassportElementType::BankStatement, "file_hash", "bad file"),
        PassportElementError::files(
            EncryptedPassportElementType::BankStatement,
            vec![String::from("file_hash")],
            "bad file",
        ),
        PassportElementError::translation_file(EncryptedPassportElementType::BankStatement, "file_hash", "bad file"),
        PassportElementError::translation_files(
            EncryptedPassportElementType::BankStatement,
            vec![String::from("file_hash")],
            "bad file",
        ),
    ] {
        insta::assert_json_snapshot!(value.unwrap());
    }

    insta::assert_json_snapshot!(PassportElementError::unspecified(
        EncryptedPassportElementType::BankStatement,
        "element_hash",
        "bad file"
    ));
}

#[test]
fn create_error_accepts_type() {
    use self::EncryptedPassportElementType::*;
    for (element_type, flag) in &[
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
        let err = PassportElementError::data_field(*element_type, "address", "data_hash", "bad address");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::front_side(*element_type, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::reverse_side(*element_type, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::selfie(*element_type, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::file(*element_type, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::files(*element_type, vec![String::from("file_hash")], "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::translation_file(*element_type, "file_hash", "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }

    for (element_type, flag) in &[
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
        let err = PassportElementError::translation_files(*element_type, vec![String::from("file_hash")], "bad file");
        assert!(if *flag { err.is_ok() } else { err.is_err() });
    }
}

#[test]
fn set_passport_data_errors() {
    assert_payload_eq!(POST JSON "setPassportDataErrors" => SetPassportDataErrors::new(1, vec![]));
}

use crate::types::{
    EncryptedCredentials,
    EncryptedPassportElementAddress,
    EncryptedPassportElementBankStatement,
    EncryptedPassportElementDriverLicense,
    EncryptedPassportElementEmail,
    EncryptedPassportElementIdentityCard,
    EncryptedPassportElementInternalPassport,
    EncryptedPassportElementPassport,
    EncryptedPassportElementPassportRegistration,
    EncryptedPassportElementPersonalDetails,
    EncryptedPassportElementPhoneNumber,
    EncryptedPassportElementRentalAgreement,
    EncryptedPassportElementTemporaryRegistration,
    EncryptedPassportElementUtilityBill,
    PassportData,
    PassportFile,
    tests::assert_json_eq,
};

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
    let expected_value = serde_json::json!({
        "data": [
            {
                "type": "address",
                "data": "d",
                "hash": "h"
            },
            {
                "type": "bank_statement",
                "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "driver_license",
                "data": "d",
                "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "reverse_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "email",
                "email": "u@h.z",
                "hash": "h"
            },
            {
                "type": "identity_card",
                "data": "d",
                "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "reverse_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "internal_passport",
                "data": "d",
                "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "passport",
                "data": "d",
                "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "passport_registration",
                "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "personal_details",
                "data": "d",
                "hash": "h"
            },
            {
                "type": "phone_number",
                "phone_number": "+79270000000",
                "hash": "h"
            },
            {
                "type": "rental_agreement",
                "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "temporary_registration",
                "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "utility_bill",
                "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                "hash": "h"
            }
        ],
        "credentials": {
            "data": "d",
            "hash": "h",
            "secret": "s"
        }
    });
    assert_json_eq(expected_struct, expected_value);
}

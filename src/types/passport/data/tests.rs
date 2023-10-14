use crate::types::{
    tests::assert_json_eq,
    EncryptedCredentials,
    EncryptedPassportElement,
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
};

#[test]
fn passport_data() {
    let expected_struct = PassportData {
        credentials: EncryptedCredentials {
            data: String::from("d"),
            hash: String::from("h"),
            secret: String::from("s"),
        },
        data: vec![
            EncryptedPassportElement::Address(EncryptedPassportElementAddress {
                data: String::from("d"),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::BankStatement(EncryptedPassportElementBankStatement {
                files: vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }],
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::DriverLicense(EncryptedPassportElementDriverLicense {
                data: String::from("d"),
                front_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                reverse_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                selfie: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::Email(EncryptedPassportElementEmail {
                email: String::from("u@h.z"),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::IdentityCard(EncryptedPassportElementIdentityCard {
                data: String::from("d"),
                front_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                reverse_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                selfie: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::InternalPassport(EncryptedPassportElementInternalPassport {
                data: String::from("d"),
                front_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                selfie: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::Passport(EncryptedPassportElementPassport {
                data: String::from("d"),
                front_side: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                selfie: PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                },
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::PassportRegistration(EncryptedPassportElementPassportRegistration {
                files: vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }],
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::PersonalDetails(EncryptedPassportElementPersonalDetails {
                data: String::from("d"),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::PhoneNumber(EncryptedPassportElementPhoneNumber {
                phone_number: String::from("+79270000000"),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::RentalAgreement(EncryptedPassportElementRentalAgreement {
                files: vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }],
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::TemporaryRegistration(EncryptedPassportElementTemporaryRegistration {
                files: vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }],
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
            EncryptedPassportElement::UtilityBill(EncryptedPassportElementUtilityBill {
                files: vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }],
                translation: Some(vec![PassportFile {
                    file_id: String::from("f"),
                    file_unique_id: String::from("uf"),
                    file_size: 1,
                    file_date: 0,
                }]),
                hash: String::from("h"),
            }),
        ],
    };
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

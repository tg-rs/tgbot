use crate::api::{Form, FormValue};

#[derive(Clone, Copy, Debug)]
enum ExpectedData {
    Json,
    Form,
    Empty,
}

fn convert_form_into_json(form: Form) -> serde_json::Value {
    let mut result = serde_json::json!({});
    let mut attachments = Vec::new();
    for (name, value) in form.into_fields() {
        match value {
            FormValue::Bytes(data) => {
                result[name] = serde_json::Value::String(format!("!!binary ({})", data.len()));
            }
            FormValue::Json(data) => {
                result[name] = serde_json::from_slice(&data)
                    .unwrap_or_else(|_| panic!("Failed to parse JSON from a form value: {name}"));
            }
            FormValue::Text(data) => {
                result[name] = serde_json::Value::String(data);
            }
            FormValue::File {
                name: file_name,
                mime_type,
                reader: _reader,
            } => {
                let mime_type = mime_type.map(|x| x.to_string());
                attachments.push(serde_json::json!({
                        "id": name,
                        "name": file_name,
                        "mime_type": mime_type,
                }));
            }
        }
    }
    if !attachments.is_empty() {
        result["__attachments__"] = serde_json::json!(attachments);
    }
    result
}

macro_rules! assert_form_eq {
    ($actual_form:expr) => {{
        let actual_data = crate::types::tests::convert_form_into_json($actual_form);
        insta::assert_json_snapshot!(actual_data);
    }};
}

macro_rules! assert_write_form_eq {
    ($obj:expr, $serialize:expr) => {{
        let mut form = crate::api::Form::default();
        let data = crate::api::WriteForm::write($obj, &mut form);
        if $serialize {
            serde::Serialize::serialize(&data, &mut form).unwrap();
        }
        assert_form_eq!(form);
    }};
    ($obj:expr) => {{
        assert_write_form_eq!($obj, false);
    }};
    ($obj:expr; serialize) => {{
        assert_write_form_eq!($obj, true);
    }};
}

macro_rules! assert_payload_eq {
    (GET $expected_path:expr => $method:expr) => {
        assert_payload_eq!(
            reqwest::Method::GET,
            $expected_path,
            crate::types::tests::ExpectedData::Empty,
            $method
        );
    };
    (POST FORM $expected_path:expr => $method:expr) => {
        assert_payload_eq!(
            reqwest::Method::POST,
            $expected_path,
            crate::types::tests::ExpectedData::Form,
            $method
        );
    };
    (POST JSON $expected_path:expr => $method:expr) => {
        assert_payload_eq!(
            reqwest::Method::POST,
            $expected_path,
            crate::types::tests::ExpectedData::Json,
            $method
        );
    };
    ($expected_http_method:expr, $expected_path:expr, $expected_data:expr, $method:expr) => {{
        use crate::api::Method;
        let payload = $method.into_payload().unwrap();
        let actual_url = payload.build_url("base-url", "-token");
        let expected_url = format!("base-url/bot-token/{}", $expected_path);
        assert_eq!(expected_url, actual_url);
        assert_eq!($expected_http_method, payload.http_method);
        match ($expected_data, payload.payload_data) {
            (crate::types::tests::ExpectedData::Json, crate::api::PayloadData::Json(actual_result)) => {
                let actual_data: serde_json::Value = serde_json::from_slice(&actual_result).unwrap();
                insta::assert_json_snapshot!(actual_data);
            }
            (crate::types::tests::ExpectedData::Form, crate::api::PayloadData::Form(actual_form)) => {
                assert_form_eq!(actual_form);
            }
            (expected_data, actual_body) => {
                assert!(matches!(expected_data, crate::types::tests::ExpectedData::Empty));
                assert!(matches!(actual_body, crate::api::PayloadData::Empty));
            }
        }
    }};
}

mod background;
mod bot;
mod business;
mod callback;
mod chat;
mod checklist;
mod color;
mod community;
mod contact;
mod dice;
mod ephemeral;
mod file;
mod forum;
mod game;
mod gift;
mod giveaway;
mod inline_mode;
mod input;
mod link;
mod live_photo;
mod location;
mod media;
mod menu;
mod message;
mod passport;
mod payment;
mod poll;
mod primitive;
mod reaction;
mod reply;
mod response;
mod rich;
mod sticker;
mod story;
mod suggested_post;
mod text;
mod update;
mod user;
mod venue;
mod verification;
mod web_app;
mod webhook;

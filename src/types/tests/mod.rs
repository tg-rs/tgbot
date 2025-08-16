#[derive(Clone, Copy, Debug)]
enum ExpectedData {
    Json,
    Form,
    Empty,
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
        let payload = $method.into_payload();
        let actual_url = payload.build_url("base-url", "-token");
        let expected_url = format!("base-url/bot-token/{}", $expected_path);
        assert_eq!(expected_url, actual_url);
        assert_eq!($expected_http_method, payload.http_method);
        match ($expected_data, payload.payload_data) {
            (crate::types::tests::ExpectedData::Json, crate::api::PayloadData::Json(actual_result)) => {
                let actual_data_raw = actual_result.unwrap();
                let actual_data: serde_json::Value = serde_json::from_str(&actual_data_raw).unwrap();
                insta::assert_json_snapshot!(actual_data);
            }
            (crate::types::tests::ExpectedData::Form, crate::api::PayloadData::Form(actual_form)) => {
                insta::assert_debug_snapshot!(actual_form.into_fields());
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
mod contact;
mod dice;
mod file;
mod forum;
mod game;
mod gift;
mod giveaway;
mod inline_mode;
mod input;
mod link;
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

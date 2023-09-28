use std::{collections::HashSet, time::Duration};

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{AllowedUpdate, GetUpdates, Poll, Update, UpdateKind},
};

#[test]
fn allowed_update_serialize() {
    use crate::types::AllowedUpdate::*;
    for (variant, expected) in [
        (Message, r#""message""#),
        (EditedMessage, r#""edited_message""#),
        (ChannelPost, r#""channel_post""#),
        (EditedChannelPost, r#""edited_channel_post""#),
        (InlineQuery, r#""inline_query""#),
        (ChosenInlineResult, r#""chosen_inline_result""#),
        (CallbackQuery, r#""callback_query""#),
        (ShippingQuery, r#""shipping_query""#),
        (PreCheckoutQuery, r#""pre_checkout_query""#),
        (Poll, r#""poll""#),
        (PollAnswer, r#""poll_answer""#),
        (ChatMember, r#""chat_member""#),
    ] {
        assert_eq!(serde_json::to_string(&variant).unwrap(), expected);
    }
}

#[test]
fn allowed_update_deserialize() {
    use crate::types::AllowedUpdate::*;
    for (expected, raw_value) in [
        (Message, r#""message""#),
        (EditedMessage, r#""edited_message""#),
        (ChannelPost, r#""channel_post""#),
        (EditedChannelPost, r#""edited_channel_post""#),
        (InlineQuery, r#""inline_query""#),
        (ChosenInlineResult, r#""chosen_inline_result""#),
        (CallbackQuery, r#""callback_query""#),
        (ShippingQuery, r#""shipping_query""#),
        (PreCheckoutQuery, r#""pre_checkout_query""#),
        (Poll, r#""poll""#),
        (PollAnswer, r#""poll_answer""#),
        (ChatMember, r#""chat_member""#),
    ] {
        assert_eq!(serde_json::from_str::<AllowedUpdate>(raw_value).unwrap(), expected)
    }
}

#[test]
fn bot_status_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "my_chat_member": {
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "firstname"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "firstname",
                },
                "until_date": 0
            }
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id(), Some(1));
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::BotStatus(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.date, 0);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn callback_query_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "callback_query": {
            "id": "test",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::CallbackQuery(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, "test");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn channel_post_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "channel_post": {
            "message_id": 1111,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle",
                "username": "channelusername"
            },
            "text": "test message from channel"
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id().unwrap(), 1);
    assert_eq!(update.get_chat_username().unwrap(), "channelusername");
    assert!(update.get_user().is_none());
    if let Update {
        id,
        kind: UpdateKind::ChannelPost(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, 1111);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn chat_join_request_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "chat_join_request": {
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            },
            "date": 0
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id(), Some(1));
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::ChatJoinRequest(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.date, 0);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn chosen_inline_result_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "chosen_inline_result": {
            "result_id": "result id",
            "from": {
                "id": 1111,
                "first_name": "Test Firstname",
                "is_bot": false
            },
            "query": "q",
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1111);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::ChosenInlineResult(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.result_id, "result id");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn edited_channel_post_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "edited_channel_post": {
            "message_id": 1111,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle",
                "username": "channelusername"
            },
            "text": "test message from channel"
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id().unwrap(), 1);
    assert_eq!(update.get_chat_username().unwrap(), "channelusername");
    assert!(update.get_user().is_none());
    if let Update {
        id,
        kind: UpdateKind::EditedChannelPost(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, 1111);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn edited_message_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "edited_message": {
            "date": 1441,
            "chat": {
                "id": 1111,
                "first_name": "Test Firstname",
                "last_name": "Test Lastname",
                "username": "Testusername",
                "type": "private",
            },
            "message_id": 1365,
            "from": {
                "id": 1111,
                "first_name": "Test Firstname",
                "last_name": "Test Lastname",
                "username": "Testusername",
                "is_bot": false
            },
            "text": "Edited text",
            "edit_date": 1441
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id().unwrap(), 1111);
    assert_eq!(update.get_chat_username().unwrap(), "Testusername");
    assert_eq!(update.get_user_id().unwrap(), 1111);
    assert_eq!(update.get_user_username().unwrap(), "Testusername");
    if let Update {
        id,
        kind: UpdateKind::EditedMessage(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, 1365);
        assert!(data.is_edited());
        assert_eq!(data.edit_date.unwrap(), 1441);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn inline_query_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "inline_query": {
            "id": "query id",
            "from": {
                "id": 1111,
                "first_name": "Test Firstname",
                "is_bot": false
            },
            "query": "query text",
            "offset": "query offset"
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1111);
    if let Update {
        id,
        kind: UpdateKind::InlineQuery(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, "query id");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn message_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "test"
            },
            "chat": {
                "id": 1,
                "type": "private",
                "first_name": "test"
            },
            "text": "test"
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id().unwrap(), 1);
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
    if let Update {
        id,
        kind: UpdateKind::Message(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, 1);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn poll_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "poll": {
            "id": "poll-id",
            "question": "Rust?",
            "options": [
                {"text": "Yes", "voter_count": 1000},
                {"text": "No", "voter_count": 0}
            ],
            "is_closed": true,
            "total_voter_count": 100,
            "is_anonymous": true,
            "type": "regular",
            "allows_multiple_answers": false
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert!(update.get_user().is_none());
    if let Update {
        id,
        kind: UpdateKind::Poll(data),
    } = update
    {
        assert_eq!(id, 1);
        if let Poll::Regular(data) = data {
            assert_eq!(data.id, "poll-id");
        } else {
            panic!("Unexpected poll kind");
        }
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn poll_answer_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "poll_answer": {
            "poll_id": "poll-id",
            "user": {
                "id": 1,
                "first_name": "Jamie",
                "is_bot": false
            },
            "option_ids": [0],
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::PollAnswer(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.poll_id, "poll-id");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn pre_checkout_query_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "pre_checkout_query": {
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "currency": "GEL",
            "total_amount": 100,
            "invoice_payload": "invoice payload"
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::PreCheckoutQuery(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, "query id");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn shipping_query_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "shipping_query": {
            "id": "query-id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "invoice_payload": "payload",
            "shipping_address": {
                "country_code": "RU",
                "state": "Chechen Republic",
                "city": "Gudermes",
                "street_line1": "Nuradilov st., 12",
                "street_line2": "",
                "post_code": "366200",
            }
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::ShippingQuery(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.id, "query-id");
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn unknown_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "unknown": {
            "key": "value"
        }
    }))
    .unwrap();
    assert!(update.get_chat_id().is_none());
    assert!(update.get_chat_username().is_none());
    assert!(update.get_user_id().is_none());
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::Unknown(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data, serde_json::json!({"key": "value"}));
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn user_status_deserialize() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "chat_member": {
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "firstname"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "firstname",
                },
                "until_date": 0
            }
        }
    }))
    .unwrap();
    assert_eq!(update.get_chat_id(), Some(1));
    assert!(update.get_chat_username().is_none());
    assert_eq!(update.get_user_id().unwrap(), 1);
    assert!(update.get_user_username().is_none());
    if let Update {
        id,
        kind: UpdateKind::UserStatus(data),
    } = update
    {
        assert_eq!(id, 1);
        assert_eq!(data.date, 0);
    } else {
        panic!("Unexpected update {:?}", update);
    }
}

#[test]
fn get_updates() {
    let request = GetUpdates::default().into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getUpdates");
    match request.into_body() {
        RequestBody::Json(data) => {
            assert_eq!(data.unwrap(), "{}");
        }
        data => panic!("Unexpected request data: {:?}", data),
    }

    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    updates.insert(AllowedUpdate::Message);
    updates.insert(AllowedUpdate::EditedMessage);
    updates.insert(AllowedUpdate::ChannelPost);
    updates.insert(AllowedUpdate::EditedChannelPost);
    updates.insert(AllowedUpdate::ChosenInlineResult);
    let request = GetUpdates::default()
        .offset(0)
        .limit(10)
        .timeout(Duration::from_secs(10))
        .allowed_updates(updates)
        .add_allowed_update(AllowedUpdate::InlineQuery)
        .add_allowed_update(AllowedUpdate::CallbackQuery)
        .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
        .add_allowed_update(AllowedUpdate::ShippingQuery)
        .into_request();
    match request.into_body() {
        RequestBody::Json(data) => {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["offset"], 0);
            assert_eq!(data["limit"], 10);
            assert_eq!(data["timeout"], 10);
            let mut updates: Vec<&str> = data["allowed_updates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap())
                .collect();
            updates.sort_unstable();
            assert_eq!(
                updates,
                vec![
                    "callback_query",
                    "channel_post",
                    "chosen_inline_result",
                    "edited_channel_post",
                    "edited_message",
                    "inline_query",
                    "message",
                    "pre_checkout_query",
                    "shipping_query",
                ]
            );
        }
        data => panic!("Unexpected request data: {:?}", data),
    }

    let method = GetUpdates::default().add_allowed_update(AllowedUpdate::Message);
    assert_eq!(method.allowed_updates.unwrap().len(), 1);
}

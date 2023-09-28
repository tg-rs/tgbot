use std::collections::HashMap;

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{GetUserProfilePhotos, ParseMode, User, UserId, UserProfilePhotos},
};

#[test]
fn deserialize_user_full() {
    let data: User = serde_json::from_value(serde_json::json!({
        "id": 1,
        "first_name": "Vladimir",
        "last_name": "Zelenskiy",
        "is_bot": false,
        "username": "zelenskiy",
        "language_code": "UA"
    }))
    .unwrap();
    assert_eq!(data.id, 1);
    assert_eq!(data.first_name, "Vladimir");
    assert_eq!(data.last_name.unwrap(), "Zelenskiy");
    assert!(!data.is_bot);
    assert_eq!(data.username.unwrap(), "zelenskiy");
    assert_eq!(data.language_code.unwrap(), "UA");
}

#[test]
fn deserialize_user_partial() {
    let data: User = serde_json::from_value(serde_json::json!({
        "id": 1,
        "first_name": "Vladimir",
        "is_bot": false
    }))
    .unwrap();
    assert_eq!(data.id, 1);
    assert_eq!(data.first_name, "Vladimir");
    assert!(data.last_name.is_none());
    assert!(!data.is_bot);
    assert!(data.username.is_none());
    assert!(data.language_code.is_none());
}

#[test]
fn get_user_full_name() {
    let user: User = serde_json::from_value(serde_json::json!({
        "id": 1,
        "first_name": "first",
        "last_name": "last",
        "is_bot": false
    }))
    .unwrap();
    assert_eq!(user.get_full_name(), "first last");

    let user: User = serde_json::from_value(serde_json::json!({
        "id": 1,
        "first_name": "first",
        "is_bot": false
    }))
    .unwrap();
    assert_eq!(user.get_full_name(), "first");
}

#[test]
fn get_user_mention() {
    let user: User = serde_json::from_value(serde_json::json!({
        "id": 1,
        "first_name": r#"_*[]()~`>#+-=|{}.!<&"#,
        "is_bot": false
    }))
    .unwrap();
    assert_eq!(
        user.get_mention(ParseMode::Html).unwrap(),
        r#"<a href="tg://user?id=1">_*[]()~`&gt;#+-=|{}.!&lt;&amp;</a>"#
    );
    assert_eq!(
        user.get_mention(ParseMode::MarkdownV2).unwrap(),
        r#"[\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!<&](tg://user?id=1)"#
    );
    assert!(user.get_mention(ParseMode::Markdown).is_err());
}

#[test]
fn deserialize_user_profile_photos() {
    let data: UserProfilePhotos = serde_json::from_value(serde_json::json!({
        "total_count": 2,
        "photos": [
            [
                {
                    "file_id": "photo-1-big",
                    "file_unique_id": "photo-1-big-unique",
                    "width": 500,
                    "height": 500,
                    "file_size": 9999
                },
                {
                    "file_id": "photo-1-small",
                    "file_unique_id": "photo-1-small-unique",
                    "width": 100,
                    "height": 100,
                    "file_size": 1111
                },
            ],
            [
                {
                    "file_id": "photo-2-big",
                    "file_unique_id": "photo-2-big-unique",
                    "width": 500,
                    "height": 500,
                    "file_size": 9999
                },
                {
                    "file_id": "photo-2-small",
                    "file_unique_id": "photo-2-small-unique",
                    "width": 100,
                    "height": 100,
                    "file_size": 1111
                },
            ],
        ]
    }))
    .unwrap();
    assert_eq!(data.total_count, 2);

    assert_eq!(data.photos.len(), 2);

    let group1 = &data.photos[0];
    assert_eq!(group1.len(), 2);
    let big = &group1[0];
    let small = &group1[1];
    assert_eq!(big.file_id, "photo-1-big");
    assert_eq!(big.file_unique_id, "photo-1-big-unique");
    assert_eq!(small.file_id, "photo-1-small");
    assert_eq!(small.file_unique_id, "photo-1-small-unique");

    let group2 = &data.photos[1];
    assert_eq!(group2.len(), 2);
    let big = &group2[0];
    let small = &group2[1];
    assert_eq!(big.file_id, "photo-2-big");
    assert_eq!(big.file_unique_id, "photo-2-big-unique");
    assert_eq!(small.file_id, "photo-2-small");
    assert_eq!(small.file_unique_id, "photo-2-small-unique");
}

#[test]
fn user_id() {
    let username = UserId::from("username");
    if let UserId::Username(username) = username {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected username: {:?}", username);
    }

    let username = UserId::from(String::from("username"));
    if let UserId::Username(ref username) = username {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected username: {:?}", username);
    }
    assert_eq!(serde_json::to_string(&username).unwrap(), r#""username""#);
    assert_eq!(username.to_string(), "username");

    let user_id = UserId::from(1);
    if let UserId::Id(user_id) = user_id {
        assert_eq!(user_id, 1);
    } else {
        panic!("Unexpected user_id: {:?}", user_id);
    }
    assert_eq!(serde_json::to_string(&user_id).unwrap(), r#"1"#);
    assert_eq!(user_id.to_string(), "1");

    let mut map = HashMap::new();
    let chat_id_1 = UserId::from(1);
    let chat_id_2 = UserId::from("username");
    map.insert(chat_id_1.clone(), "1".to_string());
    map.insert(chat_id_2.clone(), "2".to_string());
    assert_eq!(map.get(&chat_id_1).unwrap(), "1");
    assert_eq!(map.get(&chat_id_2).unwrap(), "2");
}

#[test]
fn get_user_profile_photos() {
    let request = GetUserProfilePhotos::new(1).offset(5).limit(10).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getUserProfilePhotos"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["user_id"], 1);
        assert_eq!(data["offset"], 5);
        assert_eq!(data["limit"], 10);
    } else {
        panic!("Unexpected request body");
    }
}

use std::collections::HashMap;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, GetUserProfilePhotos, ParseMode, PhotoSize, User, UserId, UserProfilePhotos},
};

#[test]
fn user() {
    assert_json_eq(
        User::new(1, "Vladimir", false)
            .with_added_to_attachment_menu(true)
            .with_is_premium(true)
            .with_last_name("Zelenskiy")
            .with_language_code("UA")
            .with_username("zelenskiy"),
        serde_json::json!({
            "id": 1,
            "first_name": "Vladimir",
            "last_name": "Zelenskiy",
            "is_bot": false,
            "username": "zelenskiy",
            "language_code": "UA",
            "is_premium": true,
            "added_to_attachment_menu": true,
        }),
    );
    assert_json_eq(
        User::new(1, "Vladimir", false),
        serde_json::json!({
            "id": 1,
            "first_name": "Vladimir",
            "is_bot": false,
        }),
    );
}

#[test]
fn user_get_full_name() {
    let full = User::new(1, "Vladimir", false).with_last_name("Zelenskiy");
    assert_eq!(full.get_full_name(), "Vladimir Zelenskiy");

    let partial = User::new(1, "Vladimir", false);
    assert_eq!(partial.get_full_name(), "Vladimir");
}

#[test]
fn user_get_link() {
    let user = User::new(1, "Vladimir", false);
    assert_eq!(user.get_link(), "tg://user?id=1")
}

#[test]
fn user_get_mention() {
    let user: User = User::new(1, r#"_*[]()~`>#+-=|{}.!<&"#, false);
    assert_eq!(
        user.get_link_mention(ParseMode::Html).unwrap(),
        r#"<a href="tg://user?id=1">_*[]()~`&gt;#+-=|{}.!&lt;&amp;</a>"#
    );
    assert_eq!(
        user.get_link_mention(ParseMode::MarkdownV2).unwrap(),
        r"[\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!<&](tg://user?id=1)"
    );
    assert!(user.get_link_mention(ParseMode::Markdown).is_err());
}

#[test]
fn user_profile_photos() {
    assert_json_eq(
        UserProfilePhotos::new(
            [
                [
                    PhotoSize::new("photo-1-big", "photo-1-big-unique", 500, 500).with_file_size(9999),
                    PhotoSize::new("photo-1-small", "photo-1-small-unique", 100, 100).with_file_size(1111),
                ],
                [
                    PhotoSize::new("photo-2-big", "photo-2-big-unique", 500, 500).with_file_size(9999),
                    PhotoSize::new("photo-2-small", "photo-2-small-unique", 100, 100).with_file_size(1111),
                ],
            ],
            2,
        ),
        serde_json::json!({
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
        }),
    );
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
    let method = GetUserProfilePhotos::new(1);
    assert_payload_eq(
        Payload::json(
            "getUserProfilePhotos",
            serde_json::json!({
                "user_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "getUserProfilePhotos",
            serde_json::json!({
                "user_id": 1,
                "offset": 5,
                "limit": 10
            }),
        ),
        method.with_offset(5).with_limit(10),
    )
}

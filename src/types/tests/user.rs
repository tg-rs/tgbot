use std::collections::HashMap;

use crate::types::*;

#[test]
fn birthdate() {
    insta::assert_json_snapshot!(Birthdate::new(1, 1));
    insta::assert_json_snapshot!(Birthdate::new(1, 1).with_year(1970));
}

#[test]
fn shared_user() {
    insta::assert_json_snapshot!(SharedUser::new(1));
    insta::assert_json_snapshot!(
        SharedUser::new(1)
            .with_first_name("John")
            .with_last_name("Doe")
            .with_photo([PhotoSize::new("file-id", "file-unique-id", 100, 100)])
            .with_username("john_doe"),
    );
}

#[test]
fn user() {
    insta::assert_json_snapshot!(
        User::new(1, "Vladimir", false)
            .with_added_to_attachment_menu(true)
            .with_is_premium(true)
            .with_last_name("Zelenskiy")
            .with_language_code("UA")
            .with_username("zelenskiy")
    );
    insta::assert_json_snapshot!(User::new(1, "Vladimir", false));
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
fn user_profile_audios() {
    insta::assert_json_snapshot!(UserProfileAudios::new(
        [
            Audio::new(10, "test", "test-uniq"),
            Audio::new(20, "test-1", "test-uniq-1")
        ],
        2,
    ))
}

#[test]
fn user_profile_photos() {
    insta::assert_json_snapshot!(UserProfilePhotos::new(
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
    ));
}

#[test]
fn user_id() {
    let username = UserId::from("username");
    if let UserId::Username(ref username) = username {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected username: {username:?}");
    }

    let username = UserId::from(String::from("username"));
    if let UserId::Username(ref username) = username {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected username: {username:?}");
    }
    assert_eq!(serde_json::to_string(&username).unwrap(), r#""username""#);
    assert_eq!(username.to_string(), "username");

    let user_id = UserId::from(1);
    if let UserId::Id(user_id) = user_id {
        assert_eq!(user_id, 1);
    } else {
        panic!("Unexpected user_id: {user_id:?}");
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
    assert_payload_eq!(POST JSON "getUserProfilePhotos" => method.clone());
    let method = method.with_offset(5).with_limit(10);
    assert_payload_eq!(POST JSON "getUserProfilePhotos" => method);
}

#[test]
fn set_user_emoji_status() {
    let method = SetUserEmojiStatus::new(1);
    assert_payload_eq!(POST JSON "setUserEmojiStatus" => method.clone());
    let method = method.with_emoji_id("emoji-id").with_expiration_date(1);
    assert_payload_eq!(POST JSON "setUserEmojiStatus" => method);
}

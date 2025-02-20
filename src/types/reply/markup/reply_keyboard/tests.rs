use crate::types::{
    ChatAdministratorRights,
    KeyboardButton,
    KeyboardButtonRequestChat,
    KeyboardButtonRequestUsers,
    PollType,
    ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
    ReplyMarkup,
    WebAppInfo,
    tests::assert_json_eq,
};

#[test]
fn reply_keyboard_markup() {
    let row = vec![
        KeyboardButton::new("test"),
        KeyboardButton::new("request contact").with_request_contact(),
        KeyboardButton::new("request chat 1").with_request_chat(KeyboardButtonRequestChat::new(1, true)),
        KeyboardButton::new("request chat 2").with_request_chat(
            KeyboardButtonRequestChat::new(1, false)
                .with_chat_is_forum(true)
                .with_chat_has_username(true)
                .with_chat_is_created(true)
                .with_request_photo(true)
                .with_request_title(true)
                .with_request_username(true)
                .with_user_administrator_rights(ChatAdministratorRights::all())
                .with_bot_administrator_rights(ChatAdministratorRights::all())
                .with_bot_is_member(true),
        ),
        KeyboardButton::new("request location").with_request_location(),
        KeyboardButton::new("request quiz").with_request_poll(PollType::Quiz),
        KeyboardButton::new("request regular poll").with_request_poll(PollType::Regular),
        KeyboardButton::new("request any poll").with_request_poll(None),
        KeyboardButton::new("request user 1").with_request_users(KeyboardButtonRequestUsers::new(1)),
        KeyboardButton::new("request user 2").with_request_users(
            KeyboardButtonRequestUsers::new(1)
                .with_max_quantity(2)
                .with_request_name(true)
                .with_request_photo(true)
                .with_request_username(true)
                .with_user_is_bot(true)
                .with_user_is_premium(true),
        ),
        KeyboardButton::new("web app").with_web_app(WebAppInfo::from("https://example.com")),
    ];

    assert_json_eq(
        ReplyMarkup::from(
            ReplyKeyboardMarkup::from(vec![row.clone()])
                .with_is_persistent(true)
                .with_one_time_keyboard(true)
                .with_selective(true)
                .with_resize_keyboard(true)
                .with_input_field_placeholder("placeholder"),
        ),
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact", "request_contact": true},
                    {"text": "request chat 1", "request_chat": {
                        "request_id": 1,
                        "chat_is_channel": true,
                    }},
                    {"text": "request chat 2", "request_chat": {
                        "request_id": 1,
                        "chat_is_channel": false,
                        "chat_is_forum": true,
                        "chat_has_username": true,
                        "chat_is_created": true,
                        "request_photo": true,
                        "request_title": true,
                        "request_username": true,
                        "user_administrator_rights": {
                            "is_anonymous": true,
                            "can_manage_chat": true,
                            "can_delete_messages": true,
                            "can_manage_video_chats": true,
                            "can_restrict_members": true,
                            "can_promote_members": true,
                            "can_change_info": true,
                            "can_invite_users": true,
                            "can_post_messages": true,
                            "can_edit_messages": true,
                            "can_pin_messages": true,
                            "can_post_stories": true,
                            "can_edit_stories": true,
                            "can_delete_stories": true,
                            "can_manage_topics": true,
                        },
                        "bot_administrator_rights": {
                            "is_anonymous": true,
                            "can_manage_chat": true,
                            "can_delete_messages": true,
                            "can_manage_video_chats": true,
                            "can_restrict_members": true,
                            "can_promote_members": true,
                            "can_change_info": true,
                            "can_invite_users": true,
                            "can_post_messages": true,
                            "can_edit_messages": true,
                            "can_pin_messages": true,
                            "can_post_stories": true,
                            "can_edit_stories": true,
                            "can_delete_stories": true,
                            "can_manage_topics": true,
                        },
                        "bot_is_member": true,
                    }},
                    {"text": "request location", "request_location": true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                    {"text": "request user 1", "request_users": {"request_id": 1}},
                    {"text": "request user 2", "request_users": {
                        "request_id": 1,
                        "max_quantity": 2,
                        "request_name": true,
                        "request_photo": true,
                        "request_username": true,
                        "user_is_bot": true,
                        "user_is_premium": true
                    }},
                    {"text": "web app", "web_app": {"url": "https://example.com"}}
                ]
            ],
            "is_persistent": true,
            "resize_keyboard": true,
            "one_time_keyboard": true,
            "input_field_placeholder": "placeholder",
            "selective": true
        }),
    );

    assert_json_eq(
        ReplyMarkup::from(ReplyKeyboardMarkup::default().add_row(row)),
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact", "request_contact": true},
                    {"text": "request chat 1", "request_chat": {
                        "request_id": 1,
                        "chat_is_channel": true,
                    }},
                    {"text": "request chat 2", "request_chat": {
                        "request_id": 1,
                        "chat_is_channel": false,
                        "chat_is_forum": true,
                        "chat_has_username": true,
                        "chat_is_created": true,
                        "request_photo": true,
                        "request_title": true,
                        "request_username": true,
                        "user_administrator_rights": {
                            "is_anonymous": true,
                            "can_manage_chat": true,
                            "can_delete_messages": true,
                            "can_manage_video_chats": true,
                            "can_restrict_members": true,
                            "can_promote_members": true,
                            "can_change_info": true,
                            "can_invite_users": true,
                            "can_post_messages": true,
                            "can_edit_messages": true,
                            "can_pin_messages": true,
                            "can_post_stories": true,
                            "can_edit_stories": true,
                            "can_delete_stories": true,
                            "can_manage_topics": true,
                        },
                        "bot_administrator_rights": {
                            "is_anonymous": true,
                            "can_manage_chat": true,
                            "can_delete_messages": true,
                            "can_manage_video_chats": true,
                            "can_restrict_members": true,
                            "can_promote_members": true,
                            "can_change_info": true,
                            "can_invite_users": true,
                            "can_post_messages": true,
                            "can_edit_messages": true,
                            "can_pin_messages": true,
                            "can_post_stories": true,
                            "can_edit_stories": true,
                            "can_delete_stories": true,
                            "can_manage_topics": true,
                        },
                        "bot_is_member": true,
                    }},
                    {"text": "request location", "request_location": true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                    {"text": "request user 1", "request_users": {"request_id": 1}},
                    {"text": "request user 2", "request_users": {
                        "request_id": 1,
                        "max_quantity": 2,
                        "request_name": true,
                        "request_photo": true,
                        "request_username": true,
                        "user_is_bot": true,
                        "user_is_premium": true
                    }},
                    {"text": "web app", "web_app": {"url": "https://example.com"}}
                ]
            ]
        }),
    );
}

#[test]
fn reply_keyboard_remove() {
    assert_json_eq(
        ReplyMarkup::from(ReplyKeyboardRemove::default().with_selective(true)),
        serde_json::json!({
            "remove_keyboard": true,
            "selective": true
        }),
    );
}

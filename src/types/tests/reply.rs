use serde::Serialize;

use crate::types::*;

#[test]
fn force_reply() {
    for value in [
        ReplyMarkup::from(ForceReply::from(true)),
        ForceReply::new(true)
            .with_input_field_placeholder("placeholder")
            .with_selective(true)
            .into(),
        ForceReply::new(true).with_selective(false).into(),
    ] {
        insta::assert_json_snapshot!(value)
    }
}

#[derive(Serialize)]
struct CallbackData {
    value: String,
}

#[test]
fn inline_keyboard() {
    let callback_data = CallbackData {
        value: String::from("cd-struct"),
    };
    insta::assert_json_snapshot!(ReplyMarkup::from(vec![vec![
        InlineKeyboardButton::for_url("url", "tg://user?id=1").with_icon_custom_emoji_id("test"),
        InlineKeyboardButton::for_web_app("web app", WebAppInfo::from("https://example.com"))
            .with_style(InlineKeyboardButtonStyle::Danger),
        InlineKeyboardButton::for_callback_data("cd", "cd").with_style(InlineKeyboardButtonStyle::Primary),
        InlineKeyboardButton::for_callback_data_struct("cd", &callback_data)
            .unwrap()
            .with_style(InlineKeyboardButtonStyle::Success),
        InlineKeyboardButton::for_copy_text("cp", "val"),
        InlineKeyboardButton::for_switch_inline_query("siq", "siq"),
        InlineKeyboardButton::for_switch_inline_query_current_chat("siq_cc", "siq_cc"),
        InlineKeyboardButton::for_switch_inline_query_chosen_chat("siq_chc", SwitchInlineQueryChosenChat::new("query"),),
        InlineKeyboardButton::for_callback_game("cg"),
        InlineKeyboardButton::for_pay("pay"),
        InlineKeyboardButton::for_login_url("login url", "http://example.com"),
    ]]));
}

#[test]
fn inline_keyboard_markup_convert() {
    let a = vec![vec![InlineKeyboardButton::for_url("url", "tg://user?id=1")]];
    let b: Vec<Vec<InlineKeyboardButton>> = InlineKeyboardMarkup::from(a.clone()).into();
    assert_eq!(a.len(), b.len())
}

#[test]
fn login_url() {
    let mut url = LoginUrl::from("url");
    insta::assert_json_snapshot!(url.clone());
    url = url.with_forward_text("forward text");
    insta::assert_json_snapshot!(url.clone());
    url = url.with_bot_username("bot_username");
    insta::assert_json_snapshot!(url.clone());
    url = url.with_request_write_access(true);
    insta::assert_json_snapshot!(url);
}

#[test]
fn switch_inline_query_chosen_chat() {
    let expected_struct = SwitchInlineQueryChosenChat::new("query");
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_allow_bot_chats(true)
            .with_allow_channel_chats(true)
            .with_allow_group_chats(true)
            .with_allow_user_chats(true)
    );
}

#[test]
fn reply_keyboard_markup() {
    let row = vec![
        KeyboardButton::new("test")
            .with_icon_custom_emoji_id("test")
            .with_style(KeyboardButtonStyle::Danger),
        KeyboardButton::new("request contact")
            .with_request_contact()
            .with_style(KeyboardButtonStyle::Primary),
        KeyboardButton::new("request chat 1")
            .with_request_chat(KeyboardButtonRequestChat::new(1, true))
            .with_style(KeyboardButtonStyle::Success),
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
    insta::assert_json_snapshot!(ReplyMarkup::from(
        ReplyKeyboardMarkup::from(vec![row.clone()])
            .with_is_persistent(true)
            .with_one_time_keyboard(true)
            .with_selective(true)
            .with_resize_keyboard(true)
            .with_input_field_placeholder("placeholder"),
    ));
    insta::assert_json_snapshot!(ReplyMarkup::from(ReplyKeyboardMarkup::default().add_row(row)));
}

#[test]
fn reply_keyboard_remove() {
    insta::assert_json_snapshot!(ReplyMarkup::from(ReplyKeyboardRemove::default().with_selective(true)));
}

#[test]
fn reply_parameters() {
    insta::assert_json_snapshot!(ReplyParameters::new(1));
    let mut quote = ReplyQuote::new(1, "test");
    insta::assert_json_snapshot!(
        ReplyParameters::new(1)
            .with_allow_sending_without_reply(true)
            .with_chat_id(1)
            .with_checklist_task_id(2)
            .with_quote(quote.clone())
    );
    quote = quote.with_entities([TextEntity::bold(0..2)]);
    insta::assert_json_snapshot!(ReplyParameters::new(1).with_quote(quote.clone()));
    quote = quote.with_parse_mode(ParseMode::Markdown);
    insta::assert_json_snapshot!(ReplyParameters::new(1).with_quote(quote));
}

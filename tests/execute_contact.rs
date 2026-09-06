#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-contact-base", SendContact::new(1, "John", "+79001231212"), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Contact(data) = x.data {
                assert_eq!(data.first_name, "John");
                assert_eq!(data.phone_number, "+79001231212");
                assert!(data.last_name.is_none());
                assert!(data.user_id.is_none());
                assert!(data.vcard.is_none());
            }
        }),
        (
            "send-contact-full",
            SendContact::new(1, "John", "+79001231212")
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(
                    EphemeralMessageParameters::from(999)
                        .with_callback_query_id("cqid")
                        .with_replace_callback_query_message(true),
                )
                .with_last_name("Doe")
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup([[
                    InlineKeyboardButton::for_url("url", "tg://user?id=1").with_icon_custom_emoji_id("test"),
                    InlineKeyboardButton::for_web_app("web app", WebAppInfo::from("https://example.com"))
                        .with_style(InlineKeyboardButtonStyle::Danger),
                    InlineKeyboardButton::for_callback_data("cd", "cd").with_style(InlineKeyboardButtonStyle::Primary),
                    InlineKeyboardButton::for_copy_text("cp", "val"),
                    InlineKeyboardButton::for_switch_inline_query("siq", "siq"),
                    InlineKeyboardButton::for_switch_inline_query_current_chat("siq_cc", "siq_cc"),
                    InlineKeyboardButton::for_switch_inline_query_chosen_chat(
                        "siq_chc",
                        SwitchInlineQueryChosenChat::new("query"),
                    ),
                    InlineKeyboardButton::for_switch_inline_query_chosen_chat(
                        "siq_chc_flags",
                        SwitchInlineQueryChosenChat::new("query")
                            .with_allow_bot_chats(true)
                            .with_allow_channel_chats(true)
                            .with_allow_group_chats(true)
                            .with_allow_user_chats(true),
                    ),
                    InlineKeyboardButton::for_callback_game("cg"),
                    InlineKeyboardButton::for_pay("pay"),
                    InlineKeyboardButton::for_login_url("login url", "http://example.com"),
                    InlineKeyboardButton::for_login_url(
                        "login url with params",
                        LoginUrl::from("http://example.com")
                            .with_bot_username("test")
                            .with_forward_text("test")
                            .with_request_write_access(true),
                    ),
                ]])
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Contact(data) = x.data {
                    assert_eq!(data.first_name, "John");
                    assert_eq!(data.phone_number, "+79001231212");
                    assert_eq!(data.last_name.unwrap(), "Doe");
                    assert_eq!(data.user_id.unwrap(), 1);
                    assert_eq!(
                        data.vcard.unwrap(),
                        "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"
                    );
                }
            },
        ),
    ])
    .await;
}

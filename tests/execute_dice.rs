#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-dice-basketball",
            SendDice::new(1, DiceType::Basketball)
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(
                    ReplyKeyboardMarkup::default()
                        .add_row([
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
                            KeyboardButton::new("request managed bot")
                                .with_request_managed_bot(KeyboardButtonRequestManagedBot::new(1)),
                            KeyboardButton::new("request managed bot with name").with_request_managed_bot(
                                KeyboardButtonRequestManagedBot::new(1).with_suggested_name("test"),
                            ),
                            KeyboardButton::new("request managed bot with username").with_request_managed_bot(
                                KeyboardButtonRequestManagedBot::new(1).with_suggested_username("test"),
                            ),
                            KeyboardButton::new("request managed bot with name and username").with_request_managed_bot(
                                KeyboardButtonRequestManagedBot::new(1)
                                    .with_suggested_name("test name")
                                    .with_suggested_username("test username"),
                            ),
                            KeyboardButton::new("request quiz").with_request_poll(PollType::Quiz),
                            KeyboardButton::new("request regular poll").with_request_poll(PollType::Regular),
                            KeyboardButton::new("request any poll").with_request_poll(None),
                            KeyboardButton::new("request user 1")
                                .with_request_users(KeyboardButtonRequestUsers::new(1)),
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
                        ])
                        .with_is_persistent(true)
                        .with_one_time_keyboard(true)
                        .with_selective(true)
                        .with_resize_keyboard(true)
                        .with_input_field_placeholder("placeholder"),
                )
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Dice(data) = x.data {
                    assert_eq!(data.dice_type(), DiceType::Basketball);
                    assert_eq!(data.value(), 1);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-dice-bones",
            SendDice::new(1, DiceType::Bones).with_reply_markup(ReplyKeyboardRemove::default().with_selective(true)),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Dice(data) = x.data {
                    assert_eq!(data.dice_type(), DiceType::Bones);
                    assert_eq!(data.value(), 1);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        ("send-dice-bowling", SendDice::new(1, DiceType::Bowling), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Dice(data) = x.data {
                assert_eq!(data.dice_type(), DiceType::Bowling);
                assert_eq!(data.value(), 1);
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        ("send-dice-darts", SendDice::new(1, DiceType::Darts), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Dice(data) = x.data {
                assert_eq!(data.dice_type(), DiceType::Darts);
                assert_eq!(data.value(), 1);
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        ("send-dice-football", SendDice::new(1, DiceType::Football), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Dice(data) = x.data {
                assert_eq!(data.dice_type(), DiceType::Football);
                assert_eq!(data.value(), 1);
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        ("send-dice-slot-machine", SendDice::new(1, DiceType::SlotMachine), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Dice(data) = x.data {
                assert_eq!(data.dice_type(), DiceType::SlotMachine);
                assert_eq!(data.value(), 1);
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
    ])
    .await;
}

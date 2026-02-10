use std::{collections::HashSet, time::Duration};

use crate::types::*;

#[test]
fn allowed_update() {
    use crate::types::AllowedUpdate::*;
    for value in [
        BusinessConnection,
        CallbackQuery,
        ChannelPost,
        ChatBoostRemoved,
        ChatBoostUpdated,
        ChosenInlineResult,
        EditedChannelPost,
        EditedMessage,
        InlineQuery,
        Message,
        MessageReaction,
        Poll,
        PollAnswer,
        PreCheckoutQuery,
        PurchasedPaidMedia,
        ShippingQuery,
        UserStatus,
    ] {
        insta::assert_json_snapshot!(value);
    }
}

#[test]
fn bot_status() {
    let expected_struct = Update::new(
        1,
        UpdateType::BotStatus(Box::new(ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", true))),
            ChatMember::Member {
                user: User::new(2, "John", true),
                until_date: None,
            },
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatMemberUpdated::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn business_connection() {
    let expected_struct = Update::new(
        1,
        UpdateType::BusinessConnection(Box::new(BusinessConnection::new(
            0,
            "id",
            User::new(1, "John", false),
            2,
        ))),
    );

    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(BusinessConnection::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn business_message() {
    let expected_struct = Update::new(
        1,
        UpdateType::BusinessMessage(Box::new(Message::new(
            1,
            0,
            PrivateChat::new(1, "John"),
            MessageData::Text(Text::from("message-text")),
            User::new(1, "John", false),
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn callback_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::CallbackQuery(Box::new(CallbackQuery::new("query-id", User::new(1, "John", false)))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(CallbackQuery::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn channel_post() {
    let chat = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    let expected_struct = Update::new(
        1,
        UpdateType::ChannelPost(Box::new(
            Message::new(
                1111,
                0,
                chat.clone(),
                MessageData::Text(Text::from("test message from channel")),
                chat,
            )
            .with_author_signature("John D."),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_boost_removed() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatBoostRemoved(Box::new(ChatBoostRemoved::new(
            "id",
            ChannelChat::new(1, "test"),
            0,
            ChatBoostSource::GiftCode(User::new(1, "test", false)),
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_boost_updated() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatBoostUpdated(Box::new(ChatBoostUpdated::new(
            ChatBoost::new(0, "id", 0, ChatBoostSource::GiftCode(User::new(1, "test", false))),
            ChannelChat::new(1, "test"),
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_join_request() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatJoinRequest(Box::new(ChatJoinRequest::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatJoinRequest::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chosen_inline_result() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChosenInlineResult(Box::new(ChosenInlineResult::new(
            User::new(1, "John", false),
            "q",
            "chosen-inline-result-id",
        ))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChosenInlineResult::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn deleted_business_messages() {
    let expected_struct = Update::new(
        1,
        UpdateType::DeletedBusinessMessages(Box::new(BusinessMessagesDeleted::new(
            "id",
            PrivateChat::new(1, "John").with_username("john_doe"),
            [2],
        ))),
    );

    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());

    assert!(BusinessMessagesDeleted::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn edited_business_message() {
    let expected_struct = Update::new(
        1,
        UpdateType::EditedBusinessMessage(Box::new(
            Message::new(
                1365,
                1441,
                PrivateChat::new(1111, "John")
                    .with_last_name("Doe")
                    .with_username("john_doe"),
                MessageData::Text(Text::from("Edited text")),
                User::new(1111, "John", false)
                    .with_last_name("Doe")
                    .with_username("john_doe"),
            )
            .with_edit_date(1441),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_user_username().unwrap(), "john_doe");

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn edited_channel_post() {
    let chat = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    let expected_struct = Update::new(
        1,
        UpdateType::EditedChannelPost(Box::new(
            Message::new(
                1111,
                0,
                chat.clone(),
                MessageData::Text(Text::from("test message from channel")),
                chat,
            )
            .with_author_signature("John D."),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn edited_message() {
    let expected_struct = Update::new(
        1,
        UpdateType::EditedMessage(Box::new(
            Message::new(
                1365,
                1441,
                PrivateChat::new(1111, "John")
                    .with_last_name("Doe")
                    .with_username("john_doe"),
                MessageData::Text(Text::from("Edited text")),
                User::new(1111, "John", false)
                    .with_last_name("Doe")
                    .with_username("john_doe"),
            )
            .with_edit_date(1441),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_user_username().unwrap(), "john_doe");

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn inline_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::InlineQuery(Box::new(InlineQuery::new(
            User::new(1, "John", false),
            "query-id",
            "query offset",
            "query query",
        ))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(InlineQuery::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn message() {
    let expected_struct = Update::new(
        1,
        UpdateType::Message(Box::new(Message::new(
            1,
            0,
            PrivateChat::new(1, "John"),
            MessageData::Text(Text::from("message-text")),
            User::new(1, "John", false),
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn message_reaction() {
    let expected_struct = Update::new(
        1,
        UpdateType::MessageReaction(Box::new(MessageReactionUpdated::new(
            PrivateChat::new(1, "test"),
            0,
            1,
            [ReactionType::emoji("🤡")],
            [ReactionType::emoji("🤮")],
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());
    assert!(expected_struct.get_user().is_none());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn message_reaction_count() {
    let expected_struct = Update::new(
        1,
        UpdateType::MessageReactionCount(Box::new(MessageReactionCountUpdated::new(
            PrivateChat::new(1, "test"),
            0,
            1,
            [ReactionCount::new(ReactionType::emoji("🤡"), 1)],
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());
    assert!(expected_struct.get_user().is_none());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn poll() {
    let expected_struct = Update::new(
        1,
        UpdateType::Poll(Box::new(
            RegularPoll::new("poll-id", "Rust?")
                .with_allows_multiple_answers(false)
                .with_is_closed(true)
                .with_is_anonymous(true)
                .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                .with_total_voter_count(1000)
                .into(),
        )),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert!(Poll::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn poll_answer() {
    let expected_struct = Update::new(
        1,
        UpdateType::PollAnswer(Box::new(PollAnswer {
            poll_id: String::from("poll-id"),
            voter: PollAnswerVoter::User(User::new(1, "John", false)),
            option_ids: vec![0],
        })),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PollAnswer::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn pre_checkout_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::PreCheckoutQuery(Box::new(PreCheckoutQuery::new(
            "GEL",
            User::new(1, "John", false),
            "query-id",
            "invoice payload",
            100,
        ))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PreCheckoutQuery::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn purchased_paid_media() {
    let expected_struct = Update::new(
        1,
        UpdateType::PurchasedPaidMedia(Box::new(PaidMediaPurchased::new(
            User::new(1, "John", false),
            "payload",
        ))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PaidMediaPurchased::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct)
}

#[test]
fn shipping_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::ShippingQuery(Box::new(ShippingQuery::new(
            "query-id",
            User::new(1, "Ramazan", false),
            "payload",
            ShippingAddress::new("Gudermes", "RU", "366200", "Chechen Republic", "Nuradilov st., 12", ""),
        ))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ShippingQuery::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn unknown() {
    let expected_struct = Update::new(1, UpdateType::Unknown(serde_json::json!({"key": "value"})));
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn user_status() {
    let expected_struct = Update::new(
        1,
        UpdateType::UserStatus(Box::new(ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
            ChatMember::Member {
                user: User::new(2, "John", false),
                until_date: None,
            },
        ))),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatMemberUpdated::try_from(expected_struct.clone()).is_ok());

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn get_updates() {
    assert_payload_eq!(POST JSON "getUpdates" => GetUpdates::default());
    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    let method = GetUpdates::default()
        .with_offset(0)
        .with_limit(10)
        .with_timeout(Duration::from_secs(10))
        .with_allowed_updates(updates)
        .add_allowed_update(AllowedUpdate::Message);
    assert_payload_eq!(POST JSON "getUpdates" => method);
}

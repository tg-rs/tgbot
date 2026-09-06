#![allow(missing_docs)]
use std::time::Duration;

use tgbot::types::*;

mod cx;

use cx::{CheckResult, Cx, SpecBasename};

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute(
        "get-updates-all-empty",
        GetUpdates::new()
            .with_offset(0)
            .with_limit(10)
            .with_timeout(Duration::from_secs(10))
            .with_allowed_updates([
                AllowedUpdate::BotStatus,
                AllowedUpdate::BusinessConnection,
                AllowedUpdate::BusinessMessage,
                AllowedUpdate::CallbackQuery,
                AllowedUpdate::ChannelPost,
                AllowedUpdate::ChatBoostRemoved,
                AllowedUpdate::ChatBoostUpdated,
                AllowedUpdate::ChatJoinRequest,
                AllowedUpdate::ChosenInlineResult,
                AllowedUpdate::DeletedBusinessMessages,
                AllowedUpdate::EditedBusinessMessage,
                AllowedUpdate::EditedChannelPost,
                AllowedUpdate::EditedMessage,
                AllowedUpdate::GuestMessage,
                AllowedUpdate::InlineQuery,
                AllowedUpdate::Message,
                AllowedUpdate::MessageReaction,
                AllowedUpdate::MessageReactionCount,
                AllowedUpdate::Poll,
                AllowedUpdate::PollAnswer,
                AllowedUpdate::PreCheckoutQuery,
                AllowedUpdate::PurchasedPaidMedia,
                AllowedUpdate::ShippingQuery,
                AllowedUpdate::Subscription,
                AllowedUpdate::UserStatus,
            ]),
        |items| {
            assert!(items.is_empty());
        },
    )
    .await;
    cx.execute_batch(GET_UPDATES).await;
}

macro_rules! try_from_update {
    ($update:ident; [$($t:tt),*]) => {{
        let x = $update;
        $(
            let x: Update = $t::try_from(x).unwrap_err().into();
        )*
        x
    }};
}

const GET_UPDATES: [(SpecBasename, GetUpdates, CheckResult<Vec<Update>>); 29] = [
    ("get-updates-base-empty", GetUpdates::new(), |items| {
        assert!(items.is_empty());
    }),
    ("get-updates-bot-status", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::BotStatus(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChatMemberUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-business-connection", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::BusinessConnection(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 3);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(BusinessConnection::try_from(update).is_ok());
        }
    }),
    ("get-updates-business-message", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::BusinessMessage(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
            assert!(update.get_message().is_some());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-callback-query", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for (idx, update) in items.into_iter().enumerate() {
            assert!(matches!(update.update_type, UpdateType::CallbackQuery(_)));
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            match idx {
                1 => {
                    assert!(update.get_chat_id().is_none());
                    if let UpdateType::CallbackQuery(query) = &update.update_type {
                        assert!(matches!(
                            query.message,
                            Some(MaybeInaccessibleMessage::InaccessibleMessage(_)),
                        ));
                    }
                }
                2 => {
                    assert_eq!(update.get_chat_id().unwrap(), 1);
                    assert!(update.get_message().is_some());
                }
                _ => {
                    assert!(update.get_chat_id().is_none());
                    assert!(update.get_message().is_none());
                }
            }
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(CallbackQuery::try_from(update).is_ok());
        }
    }),
    ("get-updates-channel-post", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ChannelPost(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_chat_username().unwrap(), "channel_username");
            assert!(update.get_user().is_none());
            assert!(update.get_message().is_some());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-chat-boost-removed", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ChatBoostRemoved(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChatBoostRemoved::try_from(update).is_ok());
        }
    }),
    ("get-updates-chat-boost-updated", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ChatBoostUpdated(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChatBoostUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-chat-join-request", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ChatJoinRequest(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChatJoinRequest::try_from(update).is_ok());
        }
    }),
    ("get-updates-chosen-inline-result", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ChosenInlineResult(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChosenInlineResult::try_from(update).is_ok());
        }
    }),
    ("get-updates-deleted-business-messages", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_chat_username().unwrap(), "john_doe");
            assert!(update.get_user_id().is_none());
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(BusinessMessagesDeleted::try_from(update).is_ok());
        }
    }),
    ("get-updates-edited-business-message", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::EditedBusinessMessage(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_chat_username().unwrap(), "john_doe");
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert_eq!(update.get_user_username().unwrap(), "john_doe");
            assert!(update.get_message().is_some());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-edited-channel-post", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::EditedChannelPost(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_chat_username().unwrap(), "channel_username");
            assert!(update.get_user().is_none());
            let message = update.get_message().unwrap();
            assert!(message.is_edited());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-edited-message", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::EditedMessage(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_chat_username().unwrap(), "john_doe");
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert_eq!(update.get_user_username().unwrap(), "john_doe");
            assert!(update.get_message().is_some());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-guest-message", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::GuestMessage(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert_eq!(update.get_user_id().unwrap(), 2);
            assert!(update.get_message().is_some());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                MessageGenerationStopped,
                ManagedBotUpdated,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-inline-query", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::InlineQuery(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(InlineQuery::try_from(update).is_ok());
        }
    }),
    ("get-updates-managed-bot-updated", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ManagedBot(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 2);
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ManagedBotUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-message", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 104);
        for update in items {
            assert_eq!(update.id, 1);
            assert!(matches!(update.update_type, UpdateType::Message(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            if let Some(user_id) = update.get_user_id() {
                assert_eq!(user_id, 1);
                assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
            }
            assert!(update.get_user_username().is_none());
            let message = update.get_message().unwrap();
            assert_eq!(message.id, 1);
            let caption = match &message.data {
                MessageData::Audio(x) => x.caption.as_ref(),
                MessageData::Document(x) => x.caption.as_ref(),
                MessageData::Photo(x) => x.caption.as_ref(),
                MessageData::Video(x) => x.caption.as_ref(),
                MessageData::Voice(x) => x.caption.as_ref(),
                _ => None,
            };
            if let (Some(text), Some(caption)) = (message.get_text(), caption) {
                assert_eq!(text.data, caption.data);
            }
            if let MessageSender::Chat(sender_chat) = &message.sender {
                let sender_id = sender_chat.get_id();
                assert_eq!(sender_id, message.sender.get_chat().unwrap().get_id());
                assert_eq!(sender_id, message.sender.get_chat_id().unwrap());
                assert_eq!(sender_chat.get_username(), message.sender.get_chat_username());
            }
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Message::try_from(update).is_ok());
        }
    }),
    ("get-updates-message-reaction", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for (idx, update) in items.into_iter().enumerate() {
            assert!(matches!(update.update_type, UpdateType::MessageReaction(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            match idx {
                1 => {
                    assert_eq!(update.get_user_id().unwrap(), 1);
                    assert!(update.get_user_username().is_none());
                    assert!(update.get_user().is_some());
                }
                _ => {
                    assert!(update.get_user_id().is_none());
                    assert!(update.get_user_username().is_none());
                    assert!(update.get_user().is_none());
                }
            }
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(MessageReactionUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-message-reaction-count", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::MessageReactionCount(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user_id().is_none());
            assert!(update.get_user_username().is_none());
            assert!(update.get_user().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(MessageReactionCountUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-poll", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::Poll(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(Poll::try_from(update).is_ok());
        }
    }),
    ("get-updates-poll-answer", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for (idx, update) in items.into_iter().enumerate() {
            assert!(matches!(update.update_type, UpdateType::PollAnswer(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            if idx == 1 {
                assert_eq!(update.get_user_id().unwrap(), 1);
            } else {
                assert!(update.get_user_id().is_none());
            }
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(PollAnswer::try_from(update).is_ok());
        }
    }),
    ("get-updates-pre-checkout-query", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::PreCheckoutQuery(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(PreCheckoutQuery::try_from(update).is_ok());
        }
    }),
    ("get-updates-purchased-paid-media", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::PurchasedPaidMedia(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                ShippingQuery
            ]);
            assert!(PaidMediaPurchased::try_from(update).is_ok());
        }
    }),
    ("get-updates-shipping-query", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::ShippingQuery(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased
            ]);
            assert!(ShippingQuery::try_from(update).is_ok());
        }
    }),
    ("get-updates-stopped-message-generation", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::StoppedMessageGeneration(_)));
            assert!(update.get_chat_id().is_some());
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user_id().is_none());
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(MessageGenerationStopped::try_from(update).is_ok());
        }
    }),
    ("get-updates-subscription", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 3);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::Subscription(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(BotSubscriptionUpdated::try_from(update).is_ok());
        }
    }),
    ("get-updates-unknown", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 1);
        for update in items {
            let update_id = update.id;
            assert!(matches!(update.update_type, UpdateType::Unknown(_)));
            assert!(update.get_chat_id().is_none());
            assert!(update.get_chat_username().is_none());
            assert!(update.get_user_id().is_none());
            assert!(update.get_message().is_none());
            assert!(update.get_user_username().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                ChatMemberUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert_eq!(update.id, update_id);
        }
    }),
    ("get-updates-user-status", GetUpdates::new(), |items| {
        assert_eq!(items.len(), 2);
        for update in items {
            assert!(matches!(update.update_type, UpdateType::UserStatus(_)));
            assert_eq!(update.get_chat_id().unwrap(), 1);
            assert!(update.get_chat_username().is_none());
            assert_eq!(update.get_user_id().unwrap(), 1);
            assert!(update.get_user_username().is_none());
            assert!(update.get_message().is_none());
            let update = try_from_update!(update; [
                BotSubscriptionUpdated,
                BusinessConnection,
                BusinessMessagesDeleted,
                ChatBoostRemoved,
                ChatBoostUpdated,
                CallbackQuery,
                ChatJoinRequest,
                ChosenInlineResult,
                InlineQuery,
                ManagedBotUpdated,
                Message,
                MessageGenerationStopped,
                MessageReactionUpdated,
                MessageReactionCountUpdated,
                Poll,
                PollAnswer,
                PreCheckoutQuery,
                PaidMediaPurchased,
                ShippingQuery
            ]);
            assert!(ChatMemberUpdated::try_from(update).is_ok());
        }
    }),
];

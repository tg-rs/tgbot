# Changelog

## 0.42.0 (09.02.2026)

### Handlers

- Added `UpdateHandler` trait implementation for `Arc<T: UpdateHandler>`.

### Bot API

#### 9.4

- Added types: `GetUserProfileAudios`, `InlineKeyboardButtonStyle`, `KeyboardButtonStyle`,
  `SetBotProfilePhoto`, `RemoveBotProfilePhoto`, `UniqueGiftModelRarity`,
  `UserProfileAudios`, `VideoQuality`.
- Added enum variants:
  - `MessageData`: `ChatOwnerChanged`, `ChatOwnerLeft`.
- Added fields:
  - `Bot`: `allows_users_to_create_topics`.
  - `UniqueGift`: `is_burned`.
  - `UniqueGiftModel`: `rarity`.
  - `Video`: `qualities`.
- Added methods:
  - `Bot`: `with_allows_users_to_create_topics`.
  - `KeyboardButton`: `with_icon_custom_emoji_id`, `with_style`.
  - `InlineKeyboardButton`: `with_icon_custom_emoji_id`, `with_style`.
  - `UniqueGift`: `with_is_burned`.
  - `Video`: `with_qualities`.

## 0.41.0 (02.01.2026)

### Dependencies

- reqwest 0.13

### Bot API

#### 9.3

- Added types: `GetChatGifts`, `GetUserGifts`, `GiftBackground`, `RepostStory`, `SendMessageDraft`, `UniqueGiftColors`,
  `UserRating`.
- Added enum variants:
  - `MessageData`: `GiftUpgradeSent`.
  - `UniqueGiftOrigin`: `GiftedUpgrade`, `Offer`.
- Added fields:
  - `AcceptedGiftTypes`: `gifts_from_channels`.
  - `Bot`: `has_topics_enabled`.
  - `ChatFullInfo`: `paid_message_star_count`, `rating`, `unique_gift_colors`.
  - `ChecklistTask`: `completed_by_chat`.
  - `ForumTopic`: `is_name_implicit`.
  - `Gift`: `background`, `has_colors`, `is_premium`, `personal_remaining_count`, `personal_total_count`,
    `unique_gift_variant_count`.
  - `GiftInfo`: `is_upgrade_separate`, `unique_gift_number`.
  - `MessageDataForumTopicCreated`: `is_name_implicit`.
  - `OwnedGiftRegular`: `is_upgrade_separate`, `unique_gift_number`.
  - `UniqueGift`: `colors`, `gift_id`, `is_from_blockchain`, `is_premium`.
- Added methods:
  - `AcceptedGiftTypes`: `with_gifts_from_channels`.
  - `Bot`: `with_has_topics_enabled`.
  - `ChecklistTask`: `with_completed_by_chat`.
  - `CopyMessage`: `with_message_effect_id`.
  - `ForumTopic`: `with_is_name_implicit`.
  - `ForwardMessage`: `with_message_effect_id`.
  - `GetBusinessAccountGifts`: `with_exclude_from_blockchain`, `with_exclude_limited_non_upgradable`,
    `with_exclude_limited_upgradable`.
  - `Gift`: `with_background`, `with_has_colors`, `with_is_premium`, `with_personal_remaining_count`,
    `with_personal_total_count`, `with_unique_gift_variant_count`.
  - `GiftInfo`: `with_is_upgrade_separate`, `with_unique_gift_number`.
  - `MessageDataForumTopicCreated`: `with_is_name_implicit`.
  - `OwnedGiftRegular`: `with_is_upgrade_separate`, `with_unique_gift_number`.
  - `SendPaidMedia`: `with_message_thread_id`.
  - `UniqueGift`: `with_colors`, `with_is_from_blockchain`, `with_is_premium`.
- Removed methods:
  - `GetBusinessAccountGifts`: `with_exclude_limited`.

## 0.40.0 (16.08.2025)

### Bot API

#### 9.2

- Added types:
  `ApproveSuggestedPost`, `DeclineSuggestedPost`, `DirectMessagesTopic`, `SuggestedPostApprovalFailed`,
  `SuggestedPostApproved`, `SuggestedPostDeclined`, `SuggestedPostInfo`, `SuggestedPostPaid`,
  `SuggestedPostParameters`, `SuggestedPostPrice`, `SuggestedPostRefunded`, `SuggestedPostRefundReason`,
  `SuggestedPostState`.
- Added enum variants:
  - `MessageData`: `SuggestedPostApproved`, `SuggestedPostApprovalFailed`, `SuggestedPostDeclined`,
    `SuggestedPostPaid`, `SuggestedPostRefunded`.
- Added fields:
  - `ChatAdministratorRights`: `can_manage_direct_messages`.
  - `ChatFullInfo`: `is_direct_messages`, `parent_chat`.
  - `ChatMemberAdministrator`: `can_manage_direct_messages`.
  - `Gift`: `publisher_chat`.
  - `Message`: `direct_messages_topic`, `is_paid_post`, `reply_to_checklist_task_id`, `suggested_post_info`.
  - `SupergroupChat`: `is_direct_messages`.
  - `UniqueGift`: `publisher_chat`.
- Added methods:
  - `ChatAdministratorRights`: `with_can_manage_direct_messages`.
  - `ChatMemberAdministrator`: `with_can_manage_direct_messages`.
  - `CopyMessage`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`.
  - `CopyMessages`:`with_direct_messages_topic_id`.
  - `ForwardMessage`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`.
  - `ForwardMessages`:`with_direct_messages_topic_id`.
  - `Gift`: `with_publisher_chat`.
  - `Message`: `with_direct_messages_topic`, `with_is_paid_post`, `with_reply_to_checklist_task_id`,
    `with_suggested_post_info`.
  - `PromoteChatMember`: `with_can_manage_direct_messages`.
  - `ReplyParameters`: `with_checklist_task_id`.
  - `SendAnimation`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendAudio`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendContact`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendDice`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendDocument`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendInvoice`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendLocation`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendMediaGroup`:`with_direct_messages_topic_id`.
  - `SendMessage`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendPaidMedia`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendPhoto`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendSticker`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendVenue`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendVideo`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendVideoNote`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SendVoice`: `with_direct_messages_topic_id`, `with_suggested_post_parameters`,
  - `SupergroupChat`: `with_is_direct_messages`.
  - `UniqueGift`: `new`, `with_publisher_chat`.

## 0.39.1 (08.07.2025)

### Bot API

- Changed internal RawResponse type to struct to avoid obscure deserialization errors.
- Fixed custom emoji reaction type deserialization.

## 0.39.0 (05.07.2025)

### Dependencies

- Set bytes version to 1.
- Set derive_more version to 2.
- Set mime_guess version to 2.
- Set serde version to 1.
- Set serde_json version to 1.
- Set serde_with version to 3.
- Set shellwords version to 1.
- Set tokio version to 1.

### Bot API

#### 9.1

- Added types: `Checklist`, `ChecklistTask`, `ChecklistTasksAdded`, `ChecklistTasksDone`, `EditMessageChecklist`,
  `GetBotStarBalance`, `InputChecklist`, `InputChecklistTask`, `MessageDataDirectMessagePriceChanged`. `SendChecklist`.

- Added enum variants: `ExternalReplyData::Checklist`, `MessageData::Checklist`, `MessageData::ChecklistTasksAdded`,
  `MessageData::ChecklistTasksDone`, `MessageData::DirectMessagePriceChanged`, `UniqueGiftOrigin::Resale`.

- Added fields:
  - `OwnedGiftUnique`: `next_transfer_date`.
  - `UniqueGiftInfo`: `last_resale_star_count`, `next_transfer_date`.

- Added methods:
  - `OwnedGiftUnique`: `with_next_transfer_date`.
  - `UniqueGiftInfo`: `with_last_resale_star_count`, `with_next_transfer_date`.

## 0.38.0 (17.06.2025)

### Bot API

- Fixed parsing of `ChatFullInfo.photo` field.

## 0.37.0 (17.06.2025)

### Client

- Changed type of `Client::with_max_retries` argument to `u8`.
- Removed `ExecuteError::TooManyRequests` enum variant, use `ResponseError::retry_after()` method instead.
- Fixed: always sleep before sending a request in `Client::execute` method.

## 0.36.2 (17.06.2025)

### Client

- Added `with_max_retries` method.

## 0.36.1 (05.05.2025)

### Fixed

- Fixed incorrect response type for `GetBusinessConnection`.

## 0.36.0 (12.04.2025)

### Dependencies

- Updated:
  - tokio 1.44

### Bot API

#### 9.0

- Added types: `AcceptedGiftTypes`, `BusinessBotRights`, `ConvertGiftToStars`, `DeleteBusinessMessages`, `DeleteStory`,
  `EditStory`, `GetBusinessAccountGifts`, `GetBusinessAccountStarBalance`, `GiftInfo`, `GiftPremiumSubscription`,
  `InputProfilePhoto`, `InputProfilePhotoAnimated`, `InputProfilePhotoStatic`, `InputStoryContent`,
  `InputStoryContentPhoto`, `InputStoryContentVideo`, `LocationAddress`, `MessageDataPaidMessagePriceChanged`,
  `OwnedGift`, `OwnedGiftRegular`, `OwnedGifts`, `OwnedGiftUnique`, `PostStory`, `ReadBusinessMessage`,
  `RemoveBusinessAccountProfilePhoto`, `SetBusinessAccountBio`, `SetBusinessAccountGiftSettings`,
  `SetBusinessAccountName`, `SetBusinessAccountProfilePhoto`, `SetBusinessAccountUsername`, `StarAmount`, `StoryArea`,
  `StoryAreaPosition`, `StoryAreaType`, `StoryAreaTypeLink`, `StoryAreaTypeLocation`, `StoryAreaTypeSuggestedReaction`,
  `StoryAreaTypeUniqueGift`, `StoryAreaTypeWeather`, `TransactionPartnerUserType`, `TransferBusinessAccountStars`,
  `TransferGift`, `UniqueGift`, `UniqueGiftBackdrop`, `UniqueGiftBackdropColors`, `UniqueGiftInfo`, `UniqueGiftModel`,
  `UniqueGiftSymbol`, `UpgradeGift`.
- Added fields:
  - `BusinessConnection`: `rights`.
  - `ChatFullInfo`: `accepted_gift_types`.
  - `Message`: `paid_star_count`.
  - `TransactionPartnerUserParameters`: `premium_subscription_duration`, `transaction_type`.
- Added methods:
  - `BusinessConnection`: `with_rights`.
  - `Message`: `with_paid_star_count`.
  - `TransactionPartnerUserParameters`: `with_premium_subscription_duration`.
- Added enum variants:
  - `MessageData`: `Gift`, `PaidMessagePriceChanged`, `UniqueGift`.
- Removed fields:
  - `BusinessConnection`: `can_reply`.
  - `ChatFullInfo`: `can_send_gift`.
- Removed methods:
  - `BusinessConnection`: `with_can_reply`.
- Changed signature:
  - `TransactionPartnerUserParameters`: `new(User) -> Self` to `new(TransactionPartnerUserType, User) -> Self`.

## 0.35.0 (12.02.2025)

### Dependencies

- Added:
  - serde_with 3.12
- Updated:
  - bytes 1.10
  - derive_more 2.0
  - tokio 1.43

### Bot API

#### 8.3

- Removed methods:
  - `InputMedia`: `with_thumbnail`.
  - `SendGift`: `new`.
- Changed types:
  - `InputMediaType`: from enum to struct.
- Added types: `TransactionPartnerChatParameters`.
- Added fields:
  - `ChatFullInfo`: `can_send_gift`.
  - `Video`: `cover`, `start_timestamp`.
- Added methods:
  - `CopyMessage`: `with_video_start_timestamp`.
  - `ForwardMessage`: `with_video_start_timestamp`.
  - `InputMedia`: `with_cover`.
  - `InputMediaVideo`: `with_start_timestamp`.
  - `InputPaidMediaGroupItem`: `with_cover`.
  - `InputPaidMediaVideo`: `with_start_timestamp`.
  - `MediaGroupItem`: `with_cover`.
  - `SendGift`: `for_chat_id`, `for_user_id`.
  - `SendVideo`: `with_cover`, `with_start_timestamp`.
  - `Video`: `with_cover`, `with_start_timestamp`.
- Added enum variants:
  - `InputMediaError`: `CoverNotAcceptable`, `ThumbnailNotAcceptable`.
  - `TransactionPartner`: `Chat`.
- Changed signature:
  - `InputMedia`: `new` from `(file: A, media_type: B)` to `(mut media_type: InputMediaType)`.

## 0.34.0 (02.01.2025)

### Dependencies

- axum 0.8

### Bot API

#### 8.2

- Added types: `RemoveChatVerification`, `RemoveUserVerification`, `VerifyChat`, `VerifyUser`.
- Added fields:
  - `Gift`: `upgrade_star_count`.
- Added methods:
  - `Gift`: `with_upgrade_star_count`.
  - `SendGift`: `with_pay_for_upgrade`.
- Removed methods:
  - `InlineQueryResultArticle`: `with_hide_url`.

## 0.33.0 (04.12.2024)

### Dependencies

- bytes 1.9
- tokio 1.42

### Bot API

#### 8.1

- Added types: `AffiliateInfo`, `TransactionPartnerAffiliateProgramParameters`.
- Added fields:
  - `TransactionPartnerUserParameters`: `affiliate`.
- Added enum variants:
  - `TransactionPartner`: `AffiliateProgram`.
- Added methods:
  - `StarTransaction`: `with_nanostar_amount`.
  - `TransactionPartnerUserParameters`: `with_affiliate`.

## 0.32.0 (17.11.2024)

### Bot API

#### 8.0

- Added types: `EditUserStarSubscription`, `GetAvailableGifts`, `Gift`, `Gifts`, `PreparedInlineMessage`,
  `SavePreparedInlineMessage`, `SendGift`, `SetUserEmojiStatus`, `TransactionPartnerUserParameters`.
- Added fields:
  - `SuccessfulPayment`: `is_first_recurring`, `is_recurring`, `subscription_expiration_date`.
- Added methods:
  - `CreateInvoiceLink`: `with_business_connection_id`, `with_subscription_period`.
  - `SuccessfulPayment`: `with_is_first_recurring`, `with_is_recurring`, `with_subscription_expiration_date`.
- Changed enum variants:
  - `TransactionPartner::User`: Move all fields to `TransactionPartnerUserParameters`
     with additional `gift` and `subscription_period` fields introduced in Bot API 8.0.

## 0.31.0 (01.11.2024)

### Dependencies

- bytes 1.8
- tokio 1.41

### Bot API

#### 7.11

- Added enum variants:
  - `InlineKeyboardButtonType`: `CopyText`.
  - `TransactionPartner`: `TelegramApi`.
- Added methods:
  - `CopyMessage`: `allow_paid_broadcast`.
  - `InlineKeyboardButton`: `for_copy_text`.
  - `SendAnimation`: `allow_paid_broadcast`.
  - `SendAudio`: `allow_paid_broadcast`.
  - `SendContact`: `allow_paid_broadcast`.
  - `SendDice`: `allow_paid_broadcast`.
  - `SendDocument`: `allow_paid_broadcast`.
  - `SendInvoice`: `allow_paid_broadcast`.
  - `SendPhoto`: `allow_paid_broadcast`.
  - `SendVoice`: `allow_paid_broadcast`.
  - `SendGame`: `allow_paid_broadcast`.
  - `SendLocation`: `allow_paid_broadcast`.
  - `SendMediaGroup`: `allow_paid_broadcast`.
  - `SendMessage`: `allow_paid_broadcast`.
  - `SendPaidMedia`: `allow_paid_broadcast`.
  - `SendPoll`: `allow_paid_broadcast`.
  - `SendQuiz`: `allow_paid_broadcast`.
  - `SendSticker`: `allow_paid_broadcast`.
  - `SendVideo`: `allow_paid_broadcast`.
  - `SendVideoNote`: `allow_paid_broadcast`.
  - `SendVenue`: `allow_paid_broadcast`.

## 0.30.0 (07.09.2024)

### Dependencies

- tokio 1.40

### Bot API

#### 7.10

- Added fields:
  - `ChatBoostSourceGiveaway`: `prize_star_count`.
  - `Giveaway`: `prize_star_count`.
  - `GiveawayCompleted`: `is_star_giveaway`.
  - `GiveawayWinners`: `prize_star_count`.
  - `TransactionPartner::User`: `paid_media_payload`.
- Added methods:
  - `ChatBoostSourceGiveaway`: `with_prize_star_count`.
  - `Giveaway`: `with_prize_star_count`.
  - `GiveawayCompleted`: `with_is_star_giveaway`.
  - `GiveawayWinners`: `with_prize_star_count`.
  - `SendPaidMedia`: `with_payload`.
- Added types:
  - `GiveawayCreated`.
  - `PaidMediaPurchased`.
- Added enum variants:
  - `AllowedUpdate`: `PurchasedPaidMedia`.
  - `UpdateType`: `PurchasedPaidMedia`.
- Changed enum variants:
  - `MessageData`: `GiveawayCreated` to `GiveawayCreated(GiveawayCreated)`.

## 0.29.0 (18.08.2024)

### Dependencies

- derive_more 1.0

### Bot API

#### 7.9

- Added fields:
  - `TransactionPartner::User`: `paid_media`.
- Added methods:
  - `SendPaidMedia`: `with_business_connection_id`.
- Added types:
  - `CreateChatSubscriptionInviteLink`.
  - `EditChatSubscriptionInviteLink`.
- Added enum variants:
  - `ReactionType::Paid`.
- Changed enum variants:
  - `ChatMember::Member`: `Member { user: User, until_date: Option<Integer> }`.

## 0.28.0 (31.07.2024)

### Dependencies

- Updated tokio version to 1.39.
- Updated bytes version to 1.7.

### Bot API

#### 7.8

- Added fields:
  - `Bot`: `has_main_web_app`.
- Added methods:
  - `PinChatMessage`: `with_business_connection_id`.
  - `UnpinChatMessage`: `with_business_connection_id`.

## 0.27.0 (07.07.2024)

### Bot API

#### 7.7

- Added types: `RefundedPayment`.
- Added enum variants: `MessageData::RefundedPayment`.

## 0.26.0 (02.07.2024)

### Bot API

#### 7.6

- Added types: `InputPaidMediaGroup`, `InputPaidMediaGroupError`, `InputPaidMediaGroupItem`, `InputPaidMediaVideo`,
  `PaidMedia`, `PaidMediaInfo`, `PaidMediaPreview`, `SendPaidMedia`.
- Added enum variants: `ExternalReplyData::PaidMedia`, `MessageData::PaidMedia`, `TransactionPartner::TelegramAds`.
- Added fields:
  - `ChatFullInfo`: `can_send_paid_media`.
- Changed enum variants:
  - `TransactionPartner`: `User(User)` to `User { user: User, invoice_payload: Option<String> }`.

## 0.25.0 (18.06.2024)

### Dependencies

- Updated tokio version to 1.38.

### Bot API

#### 7.5

- Added types: `GetStarTransactions`, `RevenueWithdrawalState`, `StarTransaction`, `StarTransactions`, `TransactionPartner`.
- Added methods:
  - `EditMessageCaption`: `with_business_connection_id`.
  - `EditMessageLiveLocation`: `with_business_connection_id`.
  - `EditMessageMedia`: `with_business_connection_id`.
  - `EditMessageReplyMarkup`: `with_business_connection_id`.
  - `EditMessageText`: `with_business_connection_id`.
  - `StopMessageLiveLocation`: `with_business_connection_id`.
  - `StopPoll`: `with_business_connection_id`.

## 0.24.0 (29.05.2024)

### Bot API

#### 7.4

- Added types: `RefundStarPayment`.
- Added enum variants: `TextEntity::ExpandableBlockQuote`.
- Added fields:
  - `InvoiceParameters`: `provider_token`.
  - `Message`: `effect_id`, `show_caption_above_media`.
- Added methods:
  - `CopyMessage`: `with_show_caption_above_media`.
  - `EditMessageCaption`: `with_show_caption_above_media`.
  - `InlineQueryResultGif`: `with_show_caption_above_media`.
  - `InlineQueryResultMpeg4Gif`: `with_show_caption_above_media`.
  - `InlineQueryResultPhoto`: `with_show_caption_above_media`.
  - `InlineQueryResultVideo`: `with_show_caption_above_media`.
  - `InlineQueryResultCachedGif`: `with_show_caption_above_media`.
  - `InlineQueryResultCachedMpeg4Gif`: `with_show_caption_above_media`.
  - `InlineQueryResultCachedPhoto`: `with_show_caption_above_media`.
  - `InlineQueryResultCachedVideo`: `with_show_caption_above_media`.
  - `InputMediaAnimation`: `with_show_caption_above_media`.
  - `InputMediaPhoto`: `with_show_caption_above_media`.
  - `InputMediaVideo`: `with_show_caption_above_media`.
  - `InputMessageContentInvoice`: `with_provider_token`.
  - `InvoiceParameters`: `with_provider_token`.
  - `Message`: `with_effect_id`, `with_show_caption_above_media`.
  - `SendAnimation`: `with_message_effect_id`, `with_show_caption_above_media`.
  - `SendAudio`: `with_message_effect_id`.
  - `SendContact`: `with_message_effect_id`.
  - `SendDice`: `with_message_effect_id`.
  - `SendDocument`: `with_message_effect_id`.
  - `SendGame`: `with_message_effect_id`.
  - `SendInvoice`: `with_message_effect_id`.
  - `SendLocation`: `with_message_effect_id`.
  - `SendMediaGroup`: `with_message_effect_id`.
  - `SendMessage`: `with_message_effect_id`.
  - `SendPhoto`: `with_message_effect_id`, `with_show_caption_above_media`.
  - `SendPoll`: `with_message_effect_id`.
  - `SendSticker`: `with_message_effect_id`.
  - `SendVenue`: `with_message_effect_id`.
  - `SendVideo`: `with_message_effect_id`, `with_show_caption_above_media`.
  - `SendVideoNote`: `with_message_effect_id`.
  - `SendVoice`: `with_message_effect_id`.
  - `TextEntity`: `expandable_blockquote`.
- Changed signature:
  - `CreateInvoiceLink::new` to `(title: Into<String>, description: Into<String>, payload: Into<String>, currency: Into<String>, prices: IntoIterator<Item=LabeledPrice>)`.
  - `SendInvoice::new` to `(chat_id: Into<ChatId>, title: Into<String>, description: Into<String>, payload: Into<String>, currency: Into<String>, prices: IntoIterator<Item=LabeledPrice>)`.
  - `InputMessageContentInvoice::new` to `(currency: Into<String>, description: Into<String>, payload: Into<String>, prices: IntoIterator<Item=LabeledPrice>, title: Into<String>)`.

## 0.23.0 (07.05.2024)

### Bot API

#### 7.3

- Added types: `BackgroundFill`, `BackgroundType`, `ChatBackground`, `ChatFullInfo`, `ChatFullInfoType`, `InputPollOption`.
- Added enum variatns: `MessageData`::`ChatBackgroundSet`.
- Added fields:
  - `ChatMemberUpdated`: `via_join_request`.
- Added methods:
  - `ChatMemberUpdated`: `with_via_join_request`.
  - `EditMessageLiveLocation`: `with_live_period`.
  - `SendPoll`: `with_question_entities`, `with_question_parse_mode`.
  - `SendQuiz`: `with_question_entities`, `with_question_parse_mode`.
- Changed fields:
  - `RegularPoll`: `question` from `String` to `Text`.
  - `Quiz`: `question` from `String` to `Text`.
  - `PollOption`: `text` from `String` to `Text`.
- Changed signature:
  - `SendPoll::new`: `options` from `IntoIterator<Item=Into<String>>` to `IntoIterator<Item=Into<InputPollOption>>`.
  - `SendQuiz::new`: `options` from `IntoIterator<Item=Into<String>>` to `IntoIterator<Item=Into<InputPollOption>>`.
- Changed return type:
  - `GetChat`: from `Chat` to `ChatFullInfo`.
- Removed methods:
  - `ChannelChat`: `accent_color`, `active_usernames`, `available_reactions`, `background_custom_emoji_id`,
    `description`, `emoji_status_custom_emoji_id`, `emoji_status_expiration_date`, `has_protected_content`,
    `invite_link`, `linked_chat_id`, `message_auto_delete_time`, `photo`, `pinned_message`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `GroupChat`: `accent_color`, `available_reactions`, `background_custom_emoji_id`, `emoji_status_custom_emoji_id`,
    `emoji_status_expiration_date`, `has_hidden_members`, `has_protected_content`, `has_visible_history`,
    `invite_link`, `message_auto_delete_time`, `permissions`, `photo`, `pinned_message`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `PrivateChat`: `accent_color`, `active_usernames`, `background_custom_emoji_id`, `bio`, `birthdate`,
    `business_intro`, `business_location`, `business_opening_hours`, `emoji_status_custom_emoji_id`,
    `emoji_status_expiration_date`, `has_private_forwards`, `has_restricted_voice_and_video_messages`, `last_name`,
    `message_auto_delete_time`, `personal_chat`, `photo`, `pinned_message`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `SupergroupChat`: `accent_color`, `active_usernames`, `available_reactions`, `background_custom_emoji_id`,
    `can_set_sticker_set`, `custom_emoji_sticker_set_name`, `description`, `emoji_status_custom_emoji_id`,
    `emoji_status_expiration_date`, `has_aggressive_anti_spam_enabled`, `has_hidden_members`, `has_protected_content`,
    `has_visible_history`, `invite_link`, `is_forum`, `join_by_request`, `join_to_send_messages`, `linked_chat_id`,
    `location`, `message_auto_delete_time`, `permissions`, `photo`, `pinned_message`, `profile_accent_color`,
    `profile_background_custom_emoji_id`, `slow_mode_delay`, `sticker_set_name`, `unrestrict_boost_count`.
- Removed fields:
  - `ChannelChat`: `with_accent_color`, `with_active_usernames`, `with_available_reactions`,
    `with_background_custom_emoji_id`, `with_description`, `with_emoji_status_custom_emoji_id`,
    `with_emoji_status_expiration_date`, `with_has_protected_content`, `with_invite_link`, `with_linked_chat_id`,
    `with_message_auto_delete_time`, `with_photo`, `with_pinned_message`, `with_profile_accent_color`,
    `with_profile_background_custom_emoji_id`.
  - `GroupChat`: `with_accent_color`, `with_available_reactions`, `with_background_custom_emoji_id`,
    `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`, `with_has_hidden_members`,
    `with_has_protected_content`, `with_has_visible_history`, `with_invite_link`, `with_message_auto_delete_time`,
    `with_permissions`, `with_photo`, `with_pinned_message`, `with_profile_accent_color`,
    `with_profile_background_custom_emoji_id`.
  - `PrivateChat`: `with_accent_color`, `with_active_usernames`, `with_background_custom_emoji_id`, `with_bio`,
    `with_birthdate`, `with_business_intro`, `with_business_location`, `with_business_opening_hours`,
    `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`, `with_has_private_forwards`,
    `with_has_restricted_voice_and_video_messages`, `with_last_name`, `with_message_auto_delete_time`,
    `with_personal_chat`, `with_photo`, `with_pinned_message`, `with_profile_accent_color`,
    `with_profile_background_custom_emoji_id`.
  - `SupergroupChat`: `with_accent_color`, `with_active_usernames`, `with_available_reactions`,
    `with_background_custom_emoji_id`, `with_can_set_sticker_set`, `with_custom_emoji_sticker_set_name`,
    `with_description`, `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`,
    `with_has_aggressive_anti_spam_enabled`, `with_has_hidden_members`, `with_has_protected_content`,
    `with_has_visible_history`, `with_invite_link`, `with_is_forum`, `with_join_by_request`,
    `with_join_to_send_messages`, `with_linked_chat_id`, `with_location`, `with_message_auto_delete_time`,
    `with_permissions`, `with_photo`, `with_pinned_message`, `with_profile_accent_color`,
    `with_profile_background_custom_emoji_id`, `with_slow_mode_delay`, `with_sticker_set_name`,
    `with_unrestrict_boost_count`.

## 0.22.0 (01.04.2024)

### Dependencies

- Updated bytes version to 1.6.
- Updated reqwest version to 0.12.
- Updated tokio version to 1.37.

### Bot API

#### 7.2

- Added types: `Birthdate`, `BusinessConnection`, `BusinessMessagesDeleted`, `BusinessIntro`, `BusinessLocation`,
  `BusinessOpeningHours`, `BusinessOpeningHoursInterval`, `GetBusinessConnection`, `ReplaceStickerInSet`, `SharedUser`.
- Added enum variants:
  - `UpdateType`: `BusinessConnection`, `BusinessMessage`, `EditedBusinessMessage`, `DeletedBusinessMessages`.
  - `AllowedUpdate`: `BusinessConnection`, `BusinessMessage`, `EditedBusinessMessage`, `DeletedBusinessMessages`.
- Added fields:
  - `Bot`: `can_connect_to_business`.
  - `Message`: `business_connection_id`, `is_from_offline`, `sender_business_bot`.
  - `MessageDataChatShared`: `photo`, `title`, `username`.
  - `MessageDataUsersShared`: `users`.
  - `PrivateChat`: `birthdate`, `business_intro`, `business_location`, `business_opening_hours`, `personal_chat`.
- Added methods:
  - `Bot`: `with_can_connect_to_business`.
  - `KeyboardButtonRequestChat`: `with_request_photo`, `with_request_title`, `with_request_username`.
  - `KeyboardButtonRequestUsers`: `with_request_name`, `with_request_photo`, `with_request_username`.
  - `Message`: `with_business_connection_id`, `with_is_from_offline`, `with_sender_business_bot`.
  - `MessageDataChatShared`: `with_photo`, `with_title`, `with_username`.
  - `PrivateChat`: `with_birthdate`, `with_business_intro`, `with_business_location`, `with_business_opening_hours`,
    `with_personal_chat`.
  - `SendAnimation`: `with_business_connection_id`.
  - `SendAudio`: `with_business_connection_id`.
  - `SendChatAction`: `with_business_connection_id`.
  - `SendContact`: `with_business_connection_id`.
  - `SendDice`: `with_business_connection_id`.
  - `SendDocument`: `with_business_connection_id`.
  - `SendGame`: `with_business_connection_id`.
  - `SendLocation`: `with_business_connection_id`.
  - `SendMediaGroup`: `with_business_connection_id`.
  - `SendMessage`: `with_business_connection_id`.
  - `SendPhoto`: `with_business_connection_id`.
  - `SendPoll`: `with_business_connection_id`.
  - `SendQuiz`: `with_business_connection_id`.
  - `SendSticker`: `with_business_connection_id`.
  - `SendVenue`: `with_business_connection_id`.
  - `SendVideo`: `with_business_connection_id`.
  - `SendVideoNote`: `with_business_connection_id`.
  - `SendVoice`: `with_business_connection_id`.
- Changed signature:
  - `CreateNewStickerSet::new` to `(user_id: Integer, name: Into<String>, title: Into<String>, stickers: InputStickers) -> Result<Self, InputStickerError>`
  - `InputSticker::new` to `(sticker: Into<InputFile>, emoji_list: IntoIterator<Item=Into<String>>, format: StickerFormat) -> Self`
  - `SetStickerSetThumbnail::new` to `(name: Into<String>, user_id: Integer, format: StickerFormat) -> Self`
- Removed fields:
  - `MessageDataUsersShared`: `user_ids`.
  - `StickerSet`: `is_animated`, `is_video`.
- Removed methods:
  - `StickerSet`: `with_is_animated`, `with_is_video`.

## 0.21.0 (18.02.2024)

### Dependencies

- Updated tokio version to 1.36.

### Bot API

#### 7.1

- Added types: `ReplyTo`, `Story`.
- Added enum variants:
  - `MessageData`: `BoostAdded`.
- Added fields:
  - `Message`: `sender_boost_count`.
  - `SupergroupChat`: `custom_emoji_sticker_set_name`, `unrestrict_boost_count`.
- Changed enum variants:
  - `ExternalReplyInfo`: `Story` to `Story(Story)`.
  - `MessageData`: `Story` to `Story(Story)`.
- Changed fields:
  - `Message`: `reply_to` from `Option<Box<Message>>` to `Option<ReplyTo>`.

## 0.20.0 (01.01.2024)

### Dependencies

- Updated tokio version to 1.35.

### Client

- `Client::with_client` method renamed to `Client::with_http_client`.

### Handlers

- Removed `Future` associated type from `UpdateHandler` trait.

### Bot API

#### 7.0

- Added types: `AccentColor`, `ChatBoost`, `ChatBoostRemoved`, `ChatBoostSource`, `ChatBoostSourceGiveaway`,
  `ChatBoostUpdated`, `CopyMessages`, `DeleteMessages`, `ExternalReplyData`, `ExternalReplyInfo`, `ForwardMessages`,
  `GetUserChatBoosts`, `Giveaway`, `GiveawayCompleted`, `GiveawayWinners`, `InaccessibleMessage`, `LinkPreviewOptions`,
  `MaybeInaccessibleMessage`, `MessageOrigin`, `MessageOriginChannel`, `MessageOriginChat`, `MessageOriginHiddenUser`,
  `MessageOriginUser`, `MessageReactionCountUpdated`, `MessageReactionUpdated`, `ProfileAccentColor`, `ReactionCount`,
  `ReactionType`, `ReplyParameters`, `ReplyQuote`, `SetMessageReaction`, `TextQuote`, `UserChatBoosts`.
- Added enum variants:
  - `AllowedUpdate`: `ChatBoostRemoved`, `ChatBoostUpdated`, `MessageReaction`, `MessageReactionCount`.
  - `MessageData`: `Giveaway`, `GiveawayCreated`, `GiveawayCompleted`, `GiveawayWinners`.
  - `TextEntity`: `Blockquote`.
  - `UpdateType`: `ChatBoostRemoved`, `ChatBoostUpdated`, `MessageReaction`, `MessageReactionCount`.
- Added fields:
  - `ChannelChat`: `accent_color`, `available_reactions`, `background_custom_emoji_id`,
    `emoji_status_custom_emoji_id`, `emoji_status_expiration_date`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `GroupChat`: `accent_color`, `available_reactions`, `background_custom_emoji_id`, `emoji_status_custom_emoji_id`,
    `emoji_status_expiration_date`, `has_visible_history`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `Message`: `external_reply`, `forward_origin`, `link_preview_options`, `quote`.
  - `MessageDataUsersShared`: `user_ids`.
  - `PrivateChat`: `accent_color`, `background_custom_emoji_id`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
  - `SupergroupChat`: `accent_color`, `available_reactions`, `background_custom_emoji_id`,
    `emoji_status_custom_emoji_id`, `emoji_status_expiration_date`, `has_visible_history`, `profile_accent_color`,
    `profile_background_custom_emoji_id`.
- Added methods:
  - `ChannelChat`: `with_accent_color`, `with_available_reactions`, `with_background_custom_emoji_id`,
    `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`, `with_profile_accent_color`,
    `with_profile_background_custom_emoji_id`.
  - `CopyMessage`: `with_reply_parameters`.
  - `EditMessageText`: `with_link_preview_options`.
  - `GroupChat`: `with_accent_color`, `with_available_reactions`, `with_background_custom_emoji_id`,
    `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`, `with_has_visible_history`,
    `with_profile_accent_color`, `with_profile_background_custom_emoji_id`.
  - `InputMessageContentText`: `with_link_preview_options`.
  - `KeyboardButtonRequestUsers`: `with_max_quantity`.
  - `Message`: `with_external_reply`, `with_forward_origin`, `with_link_preview_options`, `with_quote`.
  - `PrivateChat`: `with_accent_color`, `with_background_custom_emoji_id`,
    `with_profile_accent_color`, `with_profile_background_custom_emoji_id`.
  - `SendAnimation`: `with_reply_parameters`.
  - `SendAudio`: `with_reply_parameters`.
  - `SendContact`: `with_reply_parameters`.
  - `SendDice`: `with_reply_parameters`.
  - `SendDocument`: `with_reply_parameters`.
  - `SendGame`: `with_reply_parameters`.
  - `SendInvoice`: `with_reply_parameters`.
  - `SendLocation`: `with_reply_parameters`.
  - `SendMediaGroup`: `with_reply_parameters`.
  - `SendMessage`: `with_link_preview_options`, `with_reply_parameters`.
  - `SendPhoto`: `with_reply_parameters`.
  - `SendPoll`: `with_reply_parameters`.
  - `SendQuiz`: `with_reply_parameters`.
  - `SendSticker`: `with_reply_parameters`.
  - `SendVenue`: `with_reply_parameters`.
  - `SendVideo`: `with_reply_parameters`.
  - `SendVideoNote`: `with_reply_parameters`.
  - `SendVoice`: `with_reply_parameters`.
  - `SupergroupChat`: `with_accent_color`, `with_available_reactions`, `with_background_custom_emoji_id`,
    `with_emoji_status_custom_emoji_id`, `with_emoji_status_expiration_date`, `with_has_visible_history`,
    `with_profile_accent_color`, `with_profile_background_custom_emoji_id`.
  - `TextEntity`: `blockquote`
- Changed enum variants:
  - `MessageData`: `PinnedMessage(Box<Message>)` to `PinnedMessage(MaybeInaccessibleMessage)`.
- Changed fields:
  - `CallbackQuery`: type of `message` from `Message` to `MaybeInaccessibleMessage`.
- Renamed types: `KeyboardButtonRequestUser` to `KeyboardButtonRequestUsers`,
  `MessageDataUserShared` to `MessageDataUsersShared`.
- Renamed enum variants:
  - `MessageData`: `UserShared` to `UsersShared`.
- Renamed methods:
  - `KeyboardButton`: `with_request_user` to `with_request_users`.
- Removed types: `Forward`, `ForwardFrom`.
- Removed fields:
  - `Message`: `forward`.
  - `MessageDataUsersShared`: `user_id`.
- Removed methods:
  - `CopyMessage`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `EditMessageText`: `with_disable_web_page_preview`.
  - `InputMessageContentText`: `with_disable_web_page_preview`.
  - `Message`: `with_forward`.
  - `SendAnimation`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendAudio`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendContact`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendDice`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendDocument`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendGame`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendInvoice`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendLocation`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendMediaGroup`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendPhoto`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendMessage`: `with_disable_web_page_preview`.
  - `SendMessage`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendPoll`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendQuiz`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendSticker`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendVenue`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendVideo`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendVideoNote`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.
  - `SendVoice`: `with_allow_sending_without_reply`, `with_reply_to_message_id`.

## 0.19.0 (05.12.2023)

### Dependencies

- Updated tokio version to 1.34.
- Updated bytes version to 1.5.
- Removed vec1 dependency.

### Client

- `Api` struct renamed to `api::Client`.
- `ApiError`, `DownloadFileError`, `ExecuteError` moved to `api` module.

### Handlers

- Merged `UpdateHandler`, `longpoll` and `webhook` into `handler` module.
- Webhooks:
  - Migrated from hyper to axum.
  - `webhook::run_server` and `webhook::SyncedUpdateHandler` replaced by `handler::WebhookServer` type.
  - Webhooks support is disabled by default and can be enabled using `webhook` feature.
- Long polling:
  - Added `#[must_use]` attribute to `LongPoll::get_handle`.
  - Renamed `LongPoll::options` to `LongPoll::with_options`.
  - Renamed methods of `LongPollOptions`: `allowed_update` to `with_allowed_update`,
    `error_timeout` to `with_error_timeout`, `limit` to `with_limit`,
    `poll_timeout` to `with_poll_timeout`.

### Types

- Merged `methods` module into `types` module.
- `serde::Deserialize` and `serde::Serialize` are implemented for all types.
- Moved `mime` reexport to `types` module.
- `Vec<TextEntity>` and `Vec1<TextEntity>` replaced with a new `TextEntities` struct.

### Bot API

#### 6.0

- Added types: `AnswerWebAppQuery`, `ChatAdministratorRights`, `GetBotDefaultAdministratorRights`, `GetChatMenuButton`,
  `MenuButton`, `SentWebAppMessage`, `SetBotDefaultAdministratorRights`, `SetChatMenuButton`, `WebAppInfo`, `WebAppData`.
- Added enum variants:
  - `InlineKeyboardButtonKind`: `WebApp`
  - `MessageData`: `WebAppData`.
- Added fields:
  - `WebhookInfo`: `last_synchronization_error_date`.
- Added methods:
  - `KeyboardButton`: `with_web_app`
  - `InlineKeyboardButton`: `with_web_app`.
- Renamed enum variants:
  - `MessageData`: `VoiceChatScheduled` to `VideoChatScheduled`, `VoiceChatStarted` to `VideoChatStarted`,
    `VoiceChatEnded` to `VideoChatEnded`, `VoiceChatParticipantsInvited` to `VideoChatParticipantsInvited`.
- Renamed fields:
  - `ChatMemberAdministrator`: `can_manage_voice_chats` to `can_manage_video_chats`.
- Renamed methods:
  - `PromoteChatMember`: `can_manage_voice_chats` to `can_manage_video_chats`.

#### 6.1

- Added types: `CreateInvoiceLink`.
- Added fields:
  - `SupergroupChat`: `join_to_send_messages`, `join_by_request`.
  - `User`: `is_premium`.
  - `Sticker`: `premium_animation`.
- Added methods:
  - `SetWebhook`: `with_secret_token`.
- Common fields from `CreateInvoiceLink` and `SendInvoice` extracted to `InvoiceParameters`.

#### 6.2

- Added types: `GetCustomEmojiStickers`.
- Added enum variants:
  - `TextEntity`: `CustomEmoji`.
- Added fields:
  - `Sticker`: `custom_emoji_id`.
  - `StickerSet`: `sticker_type`.
  - `PrivateChat`: `has_restricted_voice_and_video_messages`.
- Added methods:
  - `TextEntity`: `custom_emoji`.
  - `CreateNewStickerSet`: `with_sticker_type`.
- Removed fields:
  - `StickerSet`: `contains_masks`.
- Removed methods:
  - `CreateNewStickerSet`: `contains_masks`.

#### 6.3

- Added types: `MessageDataForumTopicCreated`, `CloseForumTopic`, `CreateForumTopic`,
  `DeleteForumTopic`, `EditForumTopic`, `GetForumTopicIconStickers`, `ReopenForumTopic`, `UnpinAllForumTopicMessages`.
- Added enum variants: `MessageData`: `ForumTopicClosed`, `ForumTopicCreated`, `ForumTopicReopened`.
- Added fields:
  - `ChatMemberAdministrator`: `can_manage_topics`.
  - `ChatMemberRestricted`: `can_manage_topics`.
  - `ChatPermissions`: `can_manage_topics`.
  - `ChannelChat`: `active_usernames`.
  - `SupergroupChat`: `active_usernames`, `is_forum`.
  - `Message`: `is_topic_message`, `message_thread_id`.
  - `PrivateChat`: `active_usernames`, `emoji_status_custom_emoji_id`.
- Added methods:
  - `CopyMessage`: `with_message_thread_id`.
  - `ForwardMessage`: `with_message_thread_id`.
  - `PromoteChatMember`: `with_can_manage_topics`.
  - `SendAnimation`: `with_message_thread_id`.
  - `SendAudio`: `with_message_thread_id`.
  - `SendContact`: `with_message_thread_id`.
  - `SendDice`: `with_message_thread_id`.
  - `SendGame`: `with_message_thread_id`.
  - `SendDocument`: `with_message_thread_id`.
  - `SendInvoice`: `with_message_thread_id`.
  - `SendLocation`: `with_message_thread_id`.
  - `SendMediaGroup`: `with_message_thread_id`.
  - `SendMessage`: `with_message_thread_id`.
  - `SendPhoto`: `with_message_thread_id`.
  - `SendPoll`: `with_message_thread_id`.
  - `SendQuiz`: `with_message_thread_id`.
  - `SendSticker`: `with_message_thread_id`.
  - `SendVenue`: `with_message_thread_id`.
  - `SendVideo`: `with_message_thread_id`.
  - `SendVideoNote`: `with_message_thread_id`.
  - `SendVoice`: `with_message_thread_id`.

#### 6.4

- Added types: `CloseGeneralForumTopic`, `EditGeneralForumTopic`, `MessageDataForumTopicEdited`,
  `HideGeneralForumTopic`, `ReopenGeneralForumTopic`, `UnhideGeneralForumTopic`,
  `MessageDataWriteAccess`.
- Added enum variants:
  - `MessageData`: `ForumTopicEdited`, `GeneralForumTopicHidden`, `GeneralForumTopicUnhidden`, `WriteAccessAllowed`.
- Added fields:
  - `GroupChat`: `has_hidden_members`.
  - `InputMediaAnimation`: `has_spoiler`.
  - `InputMediaPhoto`: `has_spoiler`.
  - `InputMediaVideo`: `has_spoiler`.
  - `Message`: `has_media_spoiler`.
  - `SupergroupChat`: `has_aggressive_anti_spam_enabled`, `has_hidden_members`.
- Added methods:
  - `ReplyKeyboardMarkup`: `with_is_persistent`.
  - `SendAnimation`: `with_has_spoiler`.
  - `SendChatAction`: `with_message_thread_id`.
  - `SendPhoto`: `with_has_spoiler`.
  - `SendVideo`: `with_has_spoiler`.

#### 6.5

- Added types: `KeyboardButtonRequestChat`, `KeyboardButtonRequestUser`, `MessageDataChatShared`,
  `MessageDataUserShared`.
- Added enum variants:
  - `MessageData`: `ChatShared`, `UserShared`.
- Added fields:
  - `ChatJoinRequest`: `user_chat_id`.
  - `ChatMemberRestricted`: `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`,
    `can_send_video_notes`, `can_send_voice_notes`.
  - `ChatPermissions`: `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`,
    `can_send_video_notes`, `can_send_voice_notes`.
- Added methods:
  - `KeyboardButton`: `wiht_request_chat`, `with_request_user`.
  - `RestrictChatMember`: `with_use_independent_chat_permissions`.
  - `SetChatPermissions` `with_use_independent_chat_permissions`.
- Removed fields:
  - `ChatMemberRestricted`: `can_send_media_messages`.
  - `ChatPermissions`: `can_send_media_messages`.

#### 6.6

- Added types: `BotDescription`, `BotShortDescription`, `GetBotDescription`, `GetBotShortDescription`,
  `SetBotDescription`, `SetBotShortDescription`, `StickerFormat`, `InputSticker`, `InputStickers`, `DeleteStickerSet`,
  `SetCustomEmojiStickerSetThumbnail`, `SetStickerSetTitle`, `SetStickerEmojiList`, `SetStickerKeywords`,
  `SetStickerMaskPosition`.
- Added fields:
  - `Sticker`: `sticker_type`, `needs_repainting`.
- Added methods:
  - `CreateNewStickerSet`: `with_needs_repainting`.
  - `SendSticker`: `with_emoji`.
- Renamed fields:
  - `Animation`: `thumb` to `thumbnail`.
  - `Audio`: `thumb` to `thumbnail`.
  - `Document`: `thumb` to `thumbnail`.
  - `InlineQueryResultArticle`: `thumb_url` to `thumbnail_url`, `thumb_width` to `thumbnail_width`,
    `thumb_height` to `thumbnail_height`.
  - `InlineQueryResultContact`: `thumb_url` to `thumbnail_url`, `thumb_width` to `thumbnail_width`,
    `thumb_height` to `thumbnail_height`.
  - `InlineQueryResultDocument`: `thumb_url` to `thumbnail_url`, `thumb_width` to `thumbnail_width`,
    `thumb_height` to `thumbnail_height`.
  - `InlineQueryResultGif`: `thumb_mime_type` to `thumbnail_mime_type`, `thumb_url` to `thumbnail_url`.
  - `InlineQueryResultLocation`: `thumb_url` to `thumbnail_url`, `thumb_width` to `thumbnail_width`,
    `thumb_height` to `thumbnail_height`.
  - `InlineQueryResultMpeg4Gif`: `thumb_mime_type` to `thumbnail_mime_type`, `thumb_url` to `thumbnail_url`.
  - `InlineQueryResultPhoto`: `thumb_url` to `thumbnail_url`.
  - `InlineQueryResultVenue`: `thumb_url` to `thumbnail_url`, `thumb_width` to `thumbnail_width`,
    `thumb_height` to `thumbnail_height`.
  - `InlineQueryResultVideo`: `thumb_url` to `thumbnail_url`.
  - `Sticker`: `thumb` to `thumbnail`.
  - `Video`: `thumb` to `thumbnail`.
  - `VideoNote`: `thumb` to `thumbnail`.
- Renamed methods:
  - `InputMedia`: `with_thumb` to `with_thumbnail`.
  - `MediaGroupItem`: `with_thumb` to `with_thumbnail`.
  - `SendAnimation`: `thumb` to `with_thumbnail`.
  - `SendAudio`: `thumb` to `with_thumbnail`.
  - `SendDocument`: `thumb` to `with_thumbnail`.
  - `SendVideo`: `thumb` to `with_thumbnail`.
  - `SendVideoNote`: `thumb` to `with_thumbnail`.
- Changed signature:
  - `AddStickerToSet::new` to `(user_id: Integer, name: Into<String>, sticker: InputSticker)`.
  - `CreateNewStickerSet::new`
    to `(user_id: Integer, name: Into<String>, title: Into<String>, InputStickers, StickerFormat)`.
  - `UploadStickerFile::new` to `(user_id: Integer, sticker: T, sticker_format: StickerFormat)`.
- Removed types: `NewSticker`.
- Removed methods:
  - `AddStickerToSet`: `mask_position`.
  - `InputMediaAnimation`: `thumb`.

#### 6.7

- Added types: `InlineQueryResultsButton`, `SwitchInlineQueryChosenChat`, `BotName`, `GetBotName`, `SetBotName`.
- Added enum variants:
  - `InlineKeyboardButtonKind`: `SwitchInlineQueryChosenChat`.
- Added fields:
  - `ChatMemberUpdated`: `via_chat_folder_invite_link`.
- Added methods:
  - `AnswerInlineQuery`: `with_button`.
  - `InlineKeyboardButton`: `for_switch_inline_query_chosen_chat`.
- Removed methods:
  - `AnswerInlineQuery`: `switch_pm_text`, `switch_pm_parameters`.

#### 6.8

- Added types: `PollAnswerVoter`, `Story`.
- Added enum variants:
  - `MessageData`: `Story`.
- Added fields:
  - `PollAnswer`: `voter`.
  - `PrivateChat`: `emoji_status_expiration_date`.
- Removed fields:
  - `PollAnswer`: `user`.

#### 6.9

- Added fields:
  - `ChatMemberAdministrator`: `can_post_stories`, `can_edit_stories`, `can_delete_stories`.
- Added methods:
  - `PromoteChatMember`: `with_can_post_stories`, `with_can_edit_stories`, `with_can_delete_stories`.

### Refactoring

#### Added fields

- `Message`: `sender`, `chat`, `author_signature`.

#### Added constructors and setters

`Animation`, `Audio`, `Bot`, `BotCommand`, `ChannelChat`, `ChatInviteLink`, `ChatJoinRequest`, `ChatLocation`,
`ChatMemberAdministrator`, `ChatMemberCreator`, `ChatMemberKicked`, `ChatMemberRestricted`, `ChatMemberUpdated`,
`ChatPhoto`, `ChosenInlineResult`, `Contact`, `Document`, `EncryptedCredentials`, `EncryptedPassportElementAddress`,
`EncryptedPassportElementBankStatement`, `EncryptedPassportElementDriverLicense`, `EncryptedPassportElementEmail`,
`EncryptedPassportElementIdentityCard`, `EncryptedPassportElementInternalPassport`,
`EncryptedPassportElementInternalPassport`, `EncryptedPassportElementPassport`,
`EncryptedPassportElementPassportRegistration`, `EncryptedPassportElementPersonalDetails`,
`EncryptedPassportElementPersonalDetails`, `EncryptedPassportElementPhoneNumber`,
`EncryptedPassportElementRentalAgreement`, `EncryptedPassportElementTemporaryRegistration`,
`EncryptedPassportElementUtilityBill`, `File`, `Forward`, `Game`, `GameHighScore`, `GroupChat`, `InlineQuery`,
`Invoice`, `Location`, `MaskPosition`, `Message`, `OrderInfo`, `PassportData`, `PassportFile`, `PhotoSize`,
`PollAnswer`, `PollOption`, `PreCheckoutQuery`, `PrivateChat`, `ProximityAlertTriggered`, `Quiz`, `RegularPoll`,
`ShippingAddress`, `ShippingQuery`, `Sticker`, `StickerSet`, `SuccessfulPayment`, `SupergroupChat`, `Text`, `Update`,
`User`, `UserProfilePhotos`, `Venue`, `Video`, `VideoNote`, `Voice`, `WebhookInfo`.

#### Added `From` implementations

- `bool` for `ForceReply`.
- `[[InlineKeyboardButton; B]; A]` for `ReplyMarkup`.
- `[[KeyboardButton; B]; A]` for `ReplyMarkup`.
- `Contact`, `Location`, `Venue`, `Text`, `String`, `&str` for `InputMessageContent`.

#### Added methods

- `MediaGroup`: `new`.
- `Update`: `get_chat`.
- `InputFileReader`: `with_file_name`, `file_name`, `with_mime_type`, `mime_type`.
- `InputMedia`: `default`.

#### Renamed types

- `Me` to `Bot`.
- `GetMe` to `GetBot`.
- `DeleteMyCommands` to `DeleteBotCommands`.
- `GetMyCommands` to `GetBotCommands`.
- `SetMyCommands` to `SetBotCommands`.
- `DiceKind` to `DiceType`.
- `PollKind` to `PollType`.
- `UpdateKind` to `UpdateType`.
- `ProximityAlertTriggered` to `MessageDataProximityAlert`.
- `EncryptedPassportElementKind` to `EncryptedPassportElementType`.
- `UnexpectedEncryptedPassportElementKind` to `UnexpectedEncryptedPassportElementType`.
- `InlineKeyboardButtonKind` to `InlineKeyboardButtonType`.

#### Renamed enum variants

- `AllowedUpdate::ChatMember` to `AllowedUpdate::UserStatus`.

#### Renamed fields

- `Update`: `kind` to `update_type`.
- `InlineKeyboardButton`: `kind` to `button_type`.

#### Changed types

- `MediaGroupItem` enum replaced with a new `MediaGroupItem` struct and documented.
- `InputMediaKind` replaced with a new `InputMediaType` enum and documented.
- `InputFile` converted into enum.

#### Changed enum variants

- `ChatId`: `Id(ChatPeerId)`, `Username(ChatUsername)`.
- `MessageData`: `AutoDeleteTimerChanged(MessageDataAutoDeleteTimer)`, `VideoChatEnded(MessageDataVideoChatEnded)`,
  `VideoChatParticipantsInvited(MessageDataVideoChatParticipantsInvited)`,
  `VideoChatScheduled(MessageDataVideoChatScheduled)`, `Audio(MessageDataAudio)`, `Document(MessageDataDocument)`,
  `Photo(MessageDataPhoto)`, `Video(MessageDataVideo)`, `Voice(MessageDataVoice)`,
  `Empty` to `Unknown(serde_json::Value)`.
- `UserId`: `Id(UserPeerId)`, `Username(UserUsername)`.

#### Renamed methods

- `DeleteBotCommands`: `scope` to `with_scope`, `language_code` to `with_language_code`.
- `GetBotCommands`: `scope` to `with_scope`, `language_code` to `with_language_code`.
- `SetBotCommands`: `scope` to `with_scope`, `language_code` to `with_language_code`.
- `CreateChatInviteLink`: `name` to `with_name`, `expire_date` to `with_expire_date`,
  `member_limit` to `with_member_limit`, `create_join_request` to `with_create_join_request`.
- `EditChatInviteLink`: `name` to `with_name`, `expire_date` to `with_expire_date`,
  `member_limit` to `with_member_limit`, `creates_join_request` to `with_creates_join_request`.
- `BanChatMember`: `until_date` to `with_until_date`, `revoke_messages` to `with_revoke_messages`.
- `PromoteChatMember`: `is_anonymous` to `with_is_anonymous`, `can_change_info` to `with_can_change_info`
  `can_delete_messages` to `with_can_delete_messages`, `can_edit_messages` to `with_can_edit_messages`,
  `can_invite_users` to `with_can_invite_users`, `can_manage_chat` to `with_can_manage_chat`,
  `can_manage_video_chat` to `with_can_manage_video_chat`, `can_pin_messages` to `with_can_pin_messages`,
  `can_post_messages` to `with_can_post_messages`, `can_promote_members` to `with_can_promote_members`,
  `can_restrict_members` to `with_can_restrict_members`, `can_manage_topics` to `with_can_manage_topics`,
  `can_post_stories` to `with_can_post_stories`, `can_edit_stories` to `with_can_edit_stories`,
  `can_delete_stories` to `with_can_delete_stories`.
- `RestrictChatMember`: `until_date` to `with_until_date`.
- `UnbanChatMember`: `only_if_banned` to `with_only_if_banned`.
- `PinChatMessage`: `disable_notification` to `with_disable_notification`.
- `UnpinChatMessage`: `message_id` to `with_message_id`.
- `ChatPermissions`: `with_send_messages` to `with_can_send_messages`, `with_send_polls` to `with_can_send_polls`,
  `with_send_other_messages` to `with_can_send_other_messages`,
  `with_add_web_page_previews` to `with_can_add_web_page_previews`, `with_change_info` to `with_can_change_info`,
  `with_invite_users` to `with_can_invite_users`, `with_pin_messages` to `with_can_pin_messages`.
- `SetChatDescription`: `description` to `with_description`.
- `SendContact`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `last_name` to `with_last_name`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`, `vcard` to `with_vcard`.
- `SendDice`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `SendAnimation`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `duration` to `with_duration`, `height` to `with_height`, `parse_mode` to `with_caption_parse_mode`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `SendAudio`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `duration` to `with_duration`, `parse_mode` to `with_caption_parse_mode`, `performer` to `with_performer`,
  `protect_content` to `with_protect_content`, `title` to `with_title`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `SendDocument`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`,
  `disable_content_type_detection` to `with_disable_content_type_detection`,
  `disable_notification` to `with_disable_notification`, `parse_mode` to `with_caption_parse_mode`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `SendPhoto`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `parse_mode` to `with_caption_parse_mode`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `SendVideo`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `duration` to `with_duration`, `height` to `with_height`, `parse_mode` to `with_caption_parse_mode`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`, `supports_streaming` to `with_supports_streaming`.
- `SendVideoNote`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `duration` to `with_duration`, `length` to `with_length`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `SendVoice`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `duration` to `with_duration`, `parse_mode` to `with_caption_parse_mode`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `InputMessageContentContact`: `last_name` to `with_last_name`, `vcard` to `with_vcard`.
- `InputMessageContentInvoice`: `is_flexible` to `with_is_flexible`, `max_tip_amount` to `with_max_tip_amount`,
  `need_email` to `with_need_email`, `need_name` to `with_need_name`,
  `need_phone_number` to `with_need_phone_number`, `need_shipping_address` to `with_need_shipping_address`,
  `photo_height` to `with_photo_height`, `photo_size` to `with_photo_size`, `photo_width` to `with_photo_width`,
  `photo_url` to `with_photo_url`, `provider_data` to `with_provider_data`,
  `send_email_to_provider` to `with_send_email_to_provider`,
  `send_phone_number_to_provider` to `with_send_phone_number_to_provider`,
  `suggested_tip_amounts` to `with_suggested_tip_amounts`.
- `InputMessageContentLocation`: `heading` to `with_heading`, `horizontal_accuracy` to `with_horizontal_accuracy`,
  `live_period` to `with_live_period`, `proximity_alert_radius` to `with_proximity_alert_radius`.
- `InputMessageContentText`: `disable_web_page_preview` to `with_disable_web_page_preview`,
  `entities` to `with_entities`, `parse_mode` to `with_parse_mode`.
- `InputMessageContentVenue`: `foursquare_id` to `with_foursquare_id`, `foursquare_type` to `with_foursquare_type`,
  `google_place_id` to `with_google_place_id`, `google_place_type` to `with_google_place_type`.
- `AnswerInlineQuery`: `button` to `with_button`, `cache_time` to `with_cache_time`,
  `is_personal` to `with_is_personal`, `next_offset` to `with_next_offset`.
- `InlineQueryResultArticle`: `description` to `with_description`, `hide_url` to `with_hide_url`,
  `reply_markup` to `with_reply_markup`, `url` to `with_url`.
- `InlineQueryResultAudio`: `audio_duration` to `with_audio_duration`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `performer` to `with_performer`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultCachedAudio`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `input_message_content` to `with_input_message_content`, `parse_mode` to `with_caption_parse_mode`,
  `reply_markup` to `with_reply_markup`.
- `InlineQueryResultContact`: `input_message_content` to `with_input_message_content`,
  `last_name` to `with_last_name`, `reply_markup` to `with_reply_markup`, `vcard` to `with_vcard`.
- `InlineQueryResultDocument`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultCachedDocument`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultGame`: `reply_markup` to `with_reply_markup`.
- `InlineQueryResultGif`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `gif_duration` to `with_gif_duration`, `gif_height` to `with_gif_height`, `gif_width` to `with_gif_width`,
  `input_message_content` to `with_input_message_content`, `parse_mode` to `with_caption_parse_mode`,
  `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultCachedGif`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `input_message_content` to `with_input_message_content`, `parse_mode` to `with_caption_parse_mode`,
  `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultLocation`: `heading` to `with_heading`, `horizontal_accuracy` to `with_horizontal_accuracy`,
  `input_message_content` to `with_input_message_content`, `live_period` to `with_live_period`,
  `proximity_alert_radius` to `with_proximity_alert_radius`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultMpeg4Gif`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `input_message_content` to `with_input_message_content`, `mpeg4_duration` to `with_mpeg4_duration`,
  `mpeg4_height` to `with_mpeg4_height`, `mpeg4_width` to `with_mpeg4_width`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultCachedMpeg4Gif`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `input_message_content` to `with_input_message_content`, `parse_mode` to `with_caption_parse_mode`,
  `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultPhoto`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `photo_height` to `with_photo_height`,
  `photo_width` to `with_photo_width`, `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultCachedPhoto`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`, `title` to `with_title`.
- `InlineQueryResultCachedSticker`: `input_message_content` to `with_input_message_content`,
  `reply_markup` to `with_reply_markup`.
- `InlineQueryResultVenue`: `foursquare_id` to `with_foursquare_id`, `foursquare_type` to `with_foursquare_type`,
  `google_place_id` to `with_google_place_id`, `google_place_type` to `with_google_place_type`,
  `input_message_content` to `with_input_message_content`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultVideo`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`,
  `video_duration` to `with_video_duration`, `video_height` to `with_video_height`,
  `video_width` to `with_video_width`.
- `InlineQueryResultCachedVideo`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `description` to `with_description`, `input_message_content` to `with_input_message_content`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`.
- `InlineQueryResultVoice`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `input_message_content` to `with_input_message_content`, `parse_mode` to `with_caption_parse_mode`,
  `reply_markup` to `with_reply_markup`, `voice_duration` to `with_voice_duration`.
- `InlineQueryResultCachedVoice`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`,
  `input_message_content` to `with_input_message_content`.
- `GetGameHighScores`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`.
- `SendGame`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `SetGameScore`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `disable_edit_message` to `with_disable_edit_message`, `force` to `with_force`.
- `InputMediaAnimation`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `duration` to `with_duration`, `height` to `with_height`, `parse_mode` to `with_caption_parse_mode`,
  `width` to `with_width`.
- `InputMediaAudio`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `duration` to `with_duration`, `parse_mode` to `with_caption_parse_mode`, `performer` to `with_performer`,
  `title` to `with_title`.
- `InputMediaDocument`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `disable_content_type_detection` to `with_disable_content_type_detection`,
  `parse_mode` to `with_caption_parse_mode`.
- `InputMediaPhoto`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `parse_mode` to `with_caption_parse_mode`.
- `InputMediaVideo`: `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `duration` to `with_duration`, `height` to `with_height`,
  `parse_mode` to `with_caption_parse_mode`, `supports_streaming` to `with_supports_streaming`, `width` to `with_width`.
- `SendLocation`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `heading` to `with_heading`,
  `horizontal_accuracy` to `with_horizontal_accuracy`, `live_period` to `with_live_period`,
  `proximity_alert_radius` to `with_proximity_alert_radius`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `MediaGroupItem`:`audio` to `for_audio`, `document` to `for_document`, `photo` to `for_photo`, `video` to `for_video`.
- `SendMediaGroup`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `protect_content` to `with_protect_content`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `InlineKeyboardMarkup`: `row` to `add_row`.
- `InlineKeyboardButton`: `callback_data` to `for_callback_data`, `callback_data_struct` to `for_callback_data_struct`,
  `callback_game` to `for_callback_game`, `login_url` to `for_login_url`, `pay` to `for_pay`, `url` to `for_url`,
  `web_app` to `for_web_app`.
- `DeleteWebhook`: `drop_pending_updates` to `with_drop_pending_updates`.
- `SetWebhook`: `allowed_updates` to `with_allowed_updates`, `certificate` to `with_certificate`,
  `drop_pending_updates` to `with_drop_pending_updates`, `ip_address` to `with_ip_address`,
  `max_connections` to `with_max_connections`.
- `SendVenue`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `foursquare_id` to `with_foursquare_id`,
  `foursquare_type` to `with_foursquare_type`, `google_place_id` to `with_google_place_id`,
  `google_place_type` to `with_google_place_type`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `GetUserProfilePhotos`: `limit` to `with_limit`, `offset` to `with_offset`.
- `GetUpdates`: `allowed_updates` to `with_allowed_updates`, `limit` to `with_limit`, `offset` to `with_offset`,
  `timeout` to `with_timeout`.
- `SendSticker`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `emoji` to `with_emoji`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `SetStickerMaskPosition`: `mask_position` to `with_mask_position`.
- `ReplyKeyboardMarkup`: `row` to `add_row`, `input_field_placeholder` to `with_input_field_placeholder`,
  `one_time_keyboard` to `with_one_time_keyboard`, `resize_keyboard` to `with_resize_keyboard`,
  `selective` to `with_selective`.
- `KeyboardButton`: `request_contact` to `with_request_contact`,
  `request_location` to `with_request_location`, `request_poll` to `with_request_poll`, `web_app` to `with_web_app`.
- `ReplyKeyboardRemove`: `selective` to `with_selective`.
- `LoginUrl`: `bot_username` to `with_bot_username`, `forward_text` to `with_forward_text`,
  `request_write_access` to `with_request_write_access`.
- `ForceReply`: `input_field_placeholder` to `with_input_field_placeholder`, `selective` to `with_selective`.
- `SendQuiz`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `close_date` to `with_close_date`,
  `disable_notification` to `with_disable_notification`, `explanation` to `with_explanation`,
  `explanation_entities` to `with_explanation_entities`, `explanation_parse_mode` to `with_explanation_parse_mode`,
  `is_anonymous` to `with_is_anonymous`, `is_closed` to `with_is_closed`, `open_period` to `with_open_period`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`.
- `SendPoll`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `allows_multiple_answers` to `with_allows_multiple_answers`, `close_date` to `with_close_date`,
  `disable_notification` to `with_disable_notification`, `is_anonymous` to `with_is_anonymous`,
  `is_closed` to `with_is_closed`, `open_period` to `with_open_period`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `StopPoll`: `reply_markup` to `with_reply_markup`.
- `CreateInvoiceLink`: `parameters` to `with_parameters`.
- `SendInvoice`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `parameters` to `with_parameters`,
  `protect_content` to `with_protect_content`, `reply_markup` to `with_reply_markup`,
  `reply_to_message_id` to `with_reply_to_message_id`, `start_parameter` to `with_start_parameter`.
- `CopyMessage`: `allow_sending_without_reply` to `with_allow_sending_without_reply`, `caption` to `with_caption`,
  `caption_entities` to `with_caption_entities`, `disable_notification` to `with_disable_notification`,
  `parse_mode` to `with_caption_parse_mode`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `EditMessageCaption`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `caption` to `with_caption`, `caption_entities` to `with_caption_entities`,
  `parse_mode` to `with_caption_parse_mode`, `reply_markup` to `with_reply_markup`.
- `EditMessageLiveLocation`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `horizontal_accuracy` to `with_horizontal_accuracy`, `heading` to `with_heading`,
  `proximity_alert_radius` to `with_proximity_alert_radius`, `reply_markup` to `with_reply_markup`.
- `EditMessageMedia`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `reply_markup` to `with_reply_markup`
- `EditMessageReplyMarkup`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `reply_markup` to `with_reply_markup`.
- `EditMessageText`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `disable_web_page_preview` to `with_disable_web_page_preview`, `entities` to `with_entities`,
  `parse_mode` to `with_parse_mode`, `reply_markup` to `with_reply_markup`,
- `ForwardMessage`: `disable_notification` to `with_disable_notification`, `protect_content` to `with_protect_content`.
- `SendMessage`: `allow_sending_without_reply` to `with_allow_sending_without_reply`,
  `disable_notification` to `with_disable_notification`, `disable_web_page_preview` to `with_disable_web_page_preview`,
  `entities` to `with_entities`, `parse_mode` to `with_parse_mode`, `protect_content` to `with_protect_content`,
  `reply_markup` to `with_reply_markup`, `reply_to_message_id` to `with_reply_to_message_id`.
- `StopMessageLiveLocation`: `new` to `for_chat_message`, `with_inline_message_id` to `for_inline_message`,
  `reply_markup` to `with_reply_markup`.
- `Dice`: `kind` to `dice_type`.

#### Removed types

- `MessageDataError` - unused.
- `InputFileInfo` - use `InputFileReader::file_name` and `InputFileReader::mime_type` methods instead.
- `MessageKind` - use `Message.chat`, `Message.sender` and `Message.author_signature` fields instead.

#### Removed enum variants

- `FormError::Io` - unused.
- `InlineKeyboardError::UnexpectedButtonKind` - unused.

#### Removed methods

- `ReplyKeyboardMarkup`:
  - `from_vec` - use `From`/`Into` instead.
- `InlineKeyboardMarkup`:
  - `from_vec` - use `From`/`Into` instead
  - `into_vec` - use `From`/`Into` instead.
- `Message`:
  - `get_user`, `get_user_id`, `get_user_username` - use `Message.sender` field instead.
  - `get_chat`, `get_chat_id`, `get_chat_username` - use `Message.chat` field instead.
- `InputFileReader`:
  - `info` - use `InputFileReader::file_name` and `InputFileReader::mime_type` instead.
- `InputFile`:
  - `reader` - use `From`/`Into` instead.

#### Changed signature

`Animation::new`, `SendContact::new`, `InputMessageContentContact::new`, `InputMessageContentInvoice::new`,
`InputMessageContentVenue::new`, `AnswerInlineQuery::new`, `AnswerWebAppQuery::new`, `InlineQueryResultArticle::new`,
`InlineQueryResultAudio::new`, `InlineQueryResultCachedAudio::new`, `InlineQueryResultContact::new`,
`InlineQueryResultDocument::new`, `InlineQueryResultCachedDocument::new`, `InlineQueryResultGame::new`,
`InlineQueryResultGif::new`, `InlineQueryResultCachedGif::new`, `InlineQueryResultVenue::new`,
`InlineQueryResultVideo::new`, `InlineQueryResultCachedVideo::new`, `InlineQueryResultVoice::new`,
`InlineQueryResultCachedVoice::new`, `InputMedia::new`, `InputMedia::with_thumbnail`, `SetPassportDataErrors::new`,
`LabeledPrice::new`, `ShippingOption::new`, `AnswerShippingQuery::ok`, `AnswerShippingQuery::error`, `SendQuiz::new`,
`SendPoll::new`.

#### Changed visibility

- `InlineKeyboardButton::new` method to private.
- `MediaGroup::add_item` method to private.

#### Fixed

- Use different type parameters for strings in `PassportElementError` factory methods and `SendContact::new` method.
- Added missing variants to the `AllowedUpdate` enum: `BotStatus`, `ChatJoinRequest`.

## 0.18.0 (10.02.2022)

- Updated tokio version to 1.16.
- Added Bot API 5.7 support.
  - Added `NewSticker::video` method.
  - Added `StickerSet.is_video` field.
  - Added `Sticker.is_video` field.
- `Update::get_user_username()` now returns `Option<&str>` instead of `Option<String>`.
- `Update::get_message()` now includes a message from `CallbackQuery`.
- Added `Message::get_user_id()` and `Message::get_user_username()` methods.

## 0.17.0 (02.02.2022)

- Removed `Config`.
- Added `Api::with_client` method.
- Spawn a new task for each incoming update in `LongPoll`.
- Retry a request on timeout error.
- Added `message_auto_delete_time` field to chat-related structs.

## 0.16.0 (01.01.2022)

- Added `Update::get_user_id` and `Update::get_user_username` methods.
- Added Bot API 5.6 support.
  - Added `protect_content` parameter to `send*` methods.
  - Added spoiler text entity support.
- Rust 2021 edition.

## 0.15.0 (27.12.2021)

- Updated tokio version to 1.15.
- Added Bot API 5.5 support.
  - Add `has_protected_content` parameter to `ChannelChat`, `GroupChat`, `SupergroupChat` and `Message` structs.
  - Add `has_private_forwards` parameter to `PrivateChat` struct.
  - Added `BanChatSenderChat` method.
  - Added `UnbanChatSenderChat` method.
- Added Bot API 5.4 support.
  - Added `ChatJoinRequest` struct.
  - Added `ApproveChatJoinRequest` method.
  - Added `DeclineChatJoinRequest` method.
  - Added `name` parameter to `EditChatInviteLink`, `CreateChatInviteLink` methods and `ChatInviteLink` struct.
  - Added `pending_join_request_count` parameter to `CharInviteLink` struct.
  - Added `creates_join_request` parameter to `EditChatInviteLink`, `CreateChatInviteLink` methods and `ChatInviteLink` struct.
  - Added `ChooseSticker` variant to `ChatAction` enum.

## 0.14.0 (06.09.2021)

- Added Bot API 5.3 support.
  - Added `BotCommandScope` enum.
  - Added `scope` and `language_code` parameters to `Set/GetMyCommands` methods.
  - Added `DeleteMyCommands` method.
  - Added `input_field_placeholder` parameter to `ReplyKeyboardMarkup` struct.
  - Added `input_field_placeholder` parameter to `ForceReply` struct.
  - Renamed `KickChatMember` method to `BanChatMember`.
  - Renamed `GetChatMembersCount` to `GetChatMemberCount`.
- Added Bot API 5.2 support.
  - Added `InputMessageContentInvoice` struct.
  - Changed type of `chat_id` argument from `Integer` to `ChatId` in `SendInvoice` method.
  - Added `max_tip_amount` and `suggested_tip_amounts` parameters to `SendInvoice` method.
  - Changed `SendInvoice::provider_data` method to accept a `&Serialize` type.
  - `SendInvoice.start_parameter` is optional now.
  - Added `InlineQuery.chat_type` field.
  - Added `MessageData::VoiceChatScheduled` variant.
  - Renamed `ChatAction::{RecordAudio, UploadAudio}` to `ChatAction::{RecordVoice, UploadVoice}`.
- Fixed parsing of command arguments when bot name is a part of the command.

## 0.13.0 (14.03.2021)

- Added Bot API 5.1 support.
  - Added `ChatMemberUpdated` struct.
  - Added `ChatInviteLink` struct.
  - Added `CreateChatInviteLink`, `EditChatInviteLink` and `RevokeChatInviteLink` methods.
  - Added voice chat service messages support.
    - `MessageData::VoiceChatStarted`.
    - `MessageData::VoiceChatEnded`.
    - `MessageData::VoiceChatParticipantsInvited`.
  - Added `AutoDeleteTimerChanged` service message support.
  - Added `ChatMemberAdministrator.can_manage_voice_chats` field.
  - Added `can_manage_voice_chats` parameter to `PromoteChatMember` method.
  - Added `revoke_messages` parameter to `KickChatMember` method.
  - Added `can_manage_chat` field to `ChatMemberAdministrator` struct.
  - Added `can_manage_chat` parameter to `PromoteChatMember` method.
  - Added `Bowling` dice.
- New `UpdateHandler` API.
  - Removed `async_trait` dependency.
  - `handle` method now takes an immutable reference to `self`.
  - Added `SyncedUpdateHandler` wrapper for non-sync handlers.
- New `TextEntity` API.
  - Use `Vec1` instead of `Vec` to guarantee that entities are not empty.
  - `TextEntityPosition` now contains `u32` values instead of `i64`.
  - You can convert ranges into `TextEntityPosition`.
- Added `Text::as_ref()` method to get data as `&str`.
- Implemented `String` to `Text` conversion.
- Added `UpdateKind::Unknown` for updates introduced in new Bot API versions.
- Added `Chat::get_id()` and `Chat::get_username()` methods.
- async-stream updated to 0.3 version.
- tokio updated to 1.3 version.
- Fixed error timeout in longpoll.

## 0.12.1 (11.01.2021)

- Fixed a panic when getting a list of bot commands.

## 0.12.0 (05.01.2021)

- Tokio 1.0 support

## 0.11.0 (15.11.2020)

- Added Bot API 5.0 support.
    - Allowed to specify text entities instead of specifying the `parse_mode`.
      - Added `entities` parameter to `SendMessage` and `EditMessageText` methods.
      - Added `explanation_entities` parameter to `SendQuiz` method
      - Added `caption_entities` parameter to
        `EditMessageCaption`, `SendAnimation`, `SendAudio`, `SendDocument`,
        `SendPhoto`, `SendVideo` and `SendVoice` methods.
      - Added `caption_entities` field to
        `InlineQueryResultAudio`, `InlineQueryResultDocument`, `InlineQueryResultGif`,
        `InlineQueryResultMpeg4Gif`, `InlineQueryResultPhoto`, `InlineQueryResultVideo`,
        `InlineQueryResultVoice`, `InlineQueryResultCachedAudio`, `InlineQueryResultCachedDocument`,
        `InlineQueryResultCachedMpeg4Gif`, `InlineQueryResultCachedPhoto`,
        `InlineQueryResultCachedMpeg4Gif`, `InlineQueryResultCachedVideo`,
        `InlineQueryResultCachedVoice`, `InputMediaAnimation`,
        `InputMediaAudio`, `InputMediaDocument`, `InputMediaPhoto` and `InputMediaVideo` structs.
      - Added `entities` field to `InputMessageContentText` struct.
    - Added football and slot machine dices.
    - Added `allow_sending_without_reply` parameter.
      Affected methods:
      - `SendMessage`
      - `SendPhoto`
      - `SendGame`
      - `SendPoll`
      - `SendAnimation`
      - `SendAudio`
      - `SendContact`
      - `SendDice`
      - `SendDocument`
      - `SendInvoice`
      - `SendLocation`
      - `SendMediaGroup`
      - `SendVenue`
      - `SendVideo`
      - `SendVideoNote`
      - `SendVoice`
      - `SendSticker`
    - Added `google_place_id` and `google_place_type` fields.
      Affected structs and methods:
      - `SendVenue` method.
      - `Venue` struct.
      - `InlineQueryResultVenue` struct.
      - `InputMessageContentVenue` struct.
    - Maximum poll question length increased to 300 characters.
    - Added `CopyMessage` method.
    - Reworked `TextEntity`.
      - Removed `Message.commands` field, use `Text::get_bot_commands()` instead.
      - Removed `TextEntityData`, `TextEntityMention` and `TextEntityLink` structs.
      - Added `TextEntityPosition` struct.
      - Added support of serializing `TextEntity`.
      - Added `Text::get_bot_commands()` method.
      - `User` struct now implements `serde::Serialize`
    - Added `author_signature` to message kinds for group chats.
    - Added `is_anonymous` parameter `PromoteChatMember` method.
    - Added `is_anonymous` field to `ChatMemberAdministrator` and `ChatMemberCreator` structs.
    - Added `sender_chat` field to `Message` struct.
    - Added horizontal accuracy support for location.
      - Added `horizontal_accuracy` field to
        `Location`, `InlineQueryResultLocation`
        and `InputMessageContentLocation` struct.
      - Added `horizontal_accuracy` parameter to `SendLocation` and `EditMessageLiveLocation` method.
    - Added proximity alert support for location.
      - Added `proximity_alert_radius` field to
        `Location`, `InlineQueryResultLocation`
        and `InputMessageContentLocation` struct.
      - Added `proximity_alert_radius` parameter to `SendLocation`
        and `EditMessageLiveLocation` method.
      - Added `ProximityAlertTriggered` service message.
    - Added heading support for location.
      - Added `heading` field to
        `Location`, `InlineQueryResultLocation` and `InputMessageContentLocation` struct.
      - Added `heading` parameter to `SendLocation` and `EditMessageLiveLocation` method.
    - Added `live_location` field to `Location` struct.
    - Added support for `Audio` and `Video` to `MediaGroup`.
    - Added `UnpinAllChatMessages` method.
    - Added `message_id` parameter to `UnpinChatMessage` method.
    - Added `disable_content_type_detection` field to `InputMediaDocument` struct.
    - Added `disable_content_type_detection` parameter to `SendDocument` method.
    - Added `file_name` to `Audio` and `Video` structs.
    - Added `only_if_banned` parameter to `UnbanChatMember` method.
    - Updated chat-related structs.
      - Added `ChatLocation` struct.
      - Added `linked_chat_id` field to `ChannelChat`.
      - Removed `all_members_are_administrators` field from `GroupChat`.
      - Added `bio` and `pinned_message` fields to `PrivateChat`.
      - Added `linked_chat_id` and `location` fields to `SupergroupChat`.
    - Added `drop_pending_updates` parameter to `SetWebhook` and `DeleteWebhook` methods.
    - Added information about IP address in webhooks.
      - Added `ip_address` parameter to `SetWebhook` method.
      - Added `ip_address` field to `WebhookInfo` struct.
    - Added `close` method.
    - Added `LogOut` method.

## 0.10.0 (20.06.2020)

- Added Bot API 4.9 support.
  * Added `via_bot` field to `Message` struct.
  * Added `thumb_mime_type` to `InlineQueryResultGif` and `InlineQueryResultMpeg4Gif` structs.
  * Added basketball dice. `DiceKind` is non-exhaustive now.
- Fixed a bug with proxy without username/password.

## 0.9.0 (26.04.2020)

- Added Bot API 4.8 support
  * `Poll` converted to enum.
  * Added support for quiz explanation.
  * Added `open_period` and `close_date` to poll/quiz.
  * `Dice.value` field is private now, use `value()` method instead.
  * Added DiceKind enum and `kind()` method to `Dice`.
- Public access to InlineKeyboard data.

## 0.8.0 (01.04.2020)

- Added Bot API 4.7 support
  * Added `SendDice` method.
  * Added `MessageData::Dice` variant with `Dice` type.
  * Added `BotCommand` type.
  * Added `SetMyCommands` and `GetMyCommands` methods.
  * Added TGS stickers support.
  * Added `thumb` field to `StickerSet` struct.
  * Added `SetStickerSetThumb` method.
- Renamed some message entity structs
  * `BotCommand` -> `TextEntityBotCommand`
  * `TextLink` -> `TextEntityLink`
  * `TextMention` -> `TextEntityMention`
- Added some missing trait derives

## 0.7.1 (16.03.2020)

- Impl `Eq` for `ChatId` and `UserId`.

## 0.7.0 (08.03.2020)

- `ChatId` type implements `Display` trait instead of `ToString`.
-  Added `Display` and `Serialize` trait implementations for `UserId` type.
- `ResponseError` fields are private now and `ResponseParameters` struct was removed,
   use corresponding methods instead.
- Added `Copy`, `Hash`, `PartialEq` and `PartialOrd` trait implementations for several types.
- Reduced memory usage when sending files.
  `InputFileReader` now takes `tokio::io::AsyncRead` instead of `std::io::Read`.
  `InputFile::path` is async and returns a `tokio::io::Result`.
- `download_file` method now returns a stream of bytes.
- Added `Command` type.

## 0.6.0 (27.01.2020)

- Added Bot API 4.6 support.
- Added `ResponseError::can_retry()` method.
- Added `ParseMode::escape()` method.
- Added `get_full_name`, `get_link` and `get_mention` methods to `User` type.
- Removed deprecated `RestrictChatMember` methods.

## 0.5.1 (10.01.2020)

- Return socks proxy support.
- Deprecate some `RestrictChatMember` methods (to be removed in 0.6.0).

## 0.5.0 (07.01.2020)

- Added async/await support.
- Added Bot API 4.5 support.
- Added rustls support.
- Fixed some errors when parsing update.
- Temporarily disabled socks proxy support.

## 0.4.4 (31.07.2019)

- Added Bot API 4.4 support.
- Added information about response data to poll request error.
- `RestrictChatMember` methods:
  `can_send_messages`,
  `can_send_media_messages`,
  `can_send_other_messages`,
  `can_add_web_page_previews` are now deprecated and will be removed in 0.5.0.
  Use `with_permissions` method instead.

## 0.4.3 (07.07.2019)

- Ignore invalid updates in webhook handler.

## 0.4.2 (01.06.2019)

- Added Bot API 4.3 support.

## 0.4.1 (28.05.2019)

- Fixed ChatMemberAdministrator parsing.

## 0.4.0 (07.05.2019)

- Added Bot API 4.2 support.
- Added `prelude` module.
- Added `ChatMember::is_member()` method.
- Added `UpdatesStream::should_retry()` method.
- Added `Api::download_file()` method.
- Added option to set custom base url for Telegram API (see `Config` struct).
- Added multipart support (you can upload files using multipart/form-data now).
- Added `UserId` type.
- You can (de)serialize callback data using `serde_json` (see `CallbackQuery` and `InlineKeyboardButton` types).
- `Api::new()` now takes a `Config` object.
- `Api::execute()` takes a method by value instead of reference.
- `Method::get_request(&self)` replaced by `Method::into_request(self)`
- `RequestBuilder` is now private.
- `WebhookServiceFactory` is now private.
- Removed use of `Mutex` in `WebhookService`.
- Removed `WebhookServiceFactoryError`.
- Removed `InlineKeyboardButton::new()` method as the user must use exactly one of the option fields.

## 0.3.0 (12.03.2019)

- Dispatcher moved to [carapax](https://github.com/tg-rs/carapax).
- Added `Update::get_chat_id()`, `Update::get_chat_username()` and `Update::get_user()` methods.
- Added `Message::is_edited()` and `Message::get_chat_username()` methods.
- Added `Message.commands` property.
- Added `UpdatesStreamOptions`.
- Removed `Api::create()` and `Api::with_proxy()` in favor of `Api::new()`.
- Removed `Api::get_updates()`, use `tgbot::handle_updates()` instead.
- `WebhookService` is public now.
- Respect `retry_after` parameter on polling error.

## 0.2.0 (27.02.2019)

- Migrated from curl to hyper.
- Added dispatcher.
- Added webhooks support.

## 0.1.0 (23.12.2018)

- First release.

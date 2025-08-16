use crate::types::*;

fn create_command(command: &str) -> Command {
    let len = command.split_whitespace().next().unwrap().len();
    let message: Message = serde_json::from_value(serde_json::json!(
        {
            "message_id": 1111,
            "date": 0,
            "from": {"id": 1, "is_bot": false, "first_name": "test"},
            "chat": {"id": 1, "type": "private", "first_name": "test"},
            "text": command,
            "entities": [
                {"type": "bot_command", "offset": 0, "length": len}
            ]
        }
    ))
    .unwrap();
    Command::try_from(message).unwrap()
}

#[test]
fn command() {
    let command = create_command("/test_command 'arg1 v' arg2");
    assert_eq!(command.get_name(), "/test_command");
    assert_eq!(command.get_args(), &["arg1 v", "arg2"]);
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_no_args() {
    let command = create_command("/test_command");
    assert_eq!(command.get_name(), "/test_command");
    assert!(command.get_args().is_empty());
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_bot_suffix() {
    let command = create_command("/test_command@bot 'arg1 v' arg2");
    assert_eq!(command.get_name(), "/test_command");
    assert_eq!(command.get_args(), &["arg1 v", "arg2"]);
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_bot_suffix_no_args() {
    let command = create_command("/test_command@abc");
    assert_eq!(command.get_name(), "/test_command");
    assert!(command.get_args().is_empty());
    assert_eq!(command.get_message().id, 1111);
}

fn create_message_struct() -> Message {
    Message::new(
        1,
        1,
        SupergroupChat::new(1, "Chat"),
        MessageData::Unknown(serde_json::json!({})),
        User::new(1, "User", false),
    )
}

#[test]
fn animation() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Animation(Animation::new(243, "file-id", "unique-id", 200, 200));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn audio() {
    let mut expected_struct = create_message_struct();
    let audio = MessageDataAudio::from(Audio::new(243, "file-id", "unique-id"));
    expected_struct.data = MessageData::Audio(audio.clone());
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::Audio(audio.with_caption(
        Text::from("test audio caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn auto_delete_timer_changed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::AutoDeleteTimerChanged(MessageDataAutoDeleteTimer::new(10000));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn boost_added() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::BoostAdded(1);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn channel_chat_created() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ChannelChatCreated;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_background_set() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ChatBackgroundSet(ChatBackground::from(BackgroundType::Wallpaper {
        dark_theme_dimming: 100,
        document: Document::new("file-id", "file-unique-id"),
        is_blurred: Some(true),
        is_moving: Some(false),
    }));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_shared() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ChatShared(MessageDataChatShared::new(1, 1));
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::ChatShared(
        MessageDataChatShared::new(1, 1)
            .with_photo([PhotoSize::new("file-id", "file-unique-id", 100, 100)])
            .with_title("title")
            .with_username("username"),
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn checklist() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Checklist(Checklist::new([], "test"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn checklist_tasks_added() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ChecklistTasksAdded(ChecklistTasksAdded::new([]));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn checklist_tasks_done() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ChecklistTasksDone(ChecklistTasksDone::default());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn connected_website() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ConnectedWebsite(String::from("http://example.com"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn contact() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Contact(Contact::new("User", "+79001231212"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn delete_chat_photo() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::DeleteChatPhoto;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn dice() {
    let mut expected_struct = create_message_struct();
    let raw_dice = serde_json::json!({"emoji": "🏀", "value": 1});
    expected_struct.data = MessageData::Dice(serde_json::from_value(raw_dice.clone()).unwrap());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn direct_message_price_changed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::DirectMessagePriceChanged(MessageDataDirectMessagePriceChanged::new(true));
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::DirectMessagePriceChanged(
        MessageDataDirectMessagePriceChanged::new(true).with_direct_message_star_count(200),
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn document() {
    let mut expected_struct = create_message_struct();
    let document = MessageDataDocument::from(Document::new("file-id", "unique-id"));
    expected_struct.data = MessageData::Document(document.clone());
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::Document(document.with_caption(
        Text::from("test document caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn forum_topic_closed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ForumTopicClosed;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn forum_topic_created() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ForumTopicCreated(MessageDataForumTopicCreated::new(
        ForumTopicIconColor::LightGreen,
        "topic-name",
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn forum_topic_edited() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ForumTopicEdited(MessageDataForumTopicEdited::default().with_name("new-name"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn forum_topic_reopened() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ForumTopicReopened;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn game() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Game(Game::new("Description", [], "Game"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn gift() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Gift(GiftInfo::new(Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    )));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn general_forum_topic_hidden() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GeneralForumTopicHidden;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn general_forum_topic_unhidden() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GeneralForumTopicUnhidden;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn giveaway() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Giveaway(Giveaway::new([ChannelChat::new(1, "test")], 0, 1));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn giveaway_created() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GiveawayCreated(GiveawayCreated::default());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn giveaway_completed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GiveawayCompleted(GiveawayCompleted::new(1));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn giveaway_winners() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GiveawayWinners(GiveawayWinners::new(
        ChannelChat::new(1, "test"),
        1,
        1,
        [User::new(1, "test", false)],
        0,
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn group_chat_created() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::GroupChatCreated;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn invoice() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Invoice(Invoice::new(
        "RUB",
        "invoice description",
        "invoice start parameter",
        "invoice title",
        100,
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn left_chat_member() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::LeftChatMember(User::new(1, "User", false));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn location() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Location(Location {
        longitude: 3.0,
        latitude: 2.0,
        horizontal_accuracy: None,
        live_period: None,
        heading: None,
        proximity_alert_radius: None,
    });
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn migrate_from_chat_id() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::MigrateFromChatId(124);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn migrate_to_chat_id() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::MigrateToChatId(124);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn new_chat_members() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::NewChatMembers(vec![User::new(1, "User", false)]);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn new_chat_photo() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::NewChatPhoto(vec![PhotoSize::new("photo file id", "unique-id", 300, 200)]);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn new_chat_title() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::NewChatTitle(String::from("new chat title"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn paid_media() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::PaidMedia(PaidMediaInfo::new(
        100,
        [PaidMedia::Preview(PaidMediaPreview::default())],
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn paid_message_price_changed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::PaidMessagePriceChanged(MessageDataPaidMessagePriceChanged {
        paid_message_star_count: 1,
    });
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn passport_data() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::PassportData(PassportData {
        data: vec![],
        credentials: EncryptedCredentials {
            data: String::from("data"),
            hash: String::from("hash"),
            secret: String::from("secret"),
        },
    });
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn photo() {
    let mut expected_struct = create_message_struct();
    let photos = MessageDataPhoto::from([PhotoSize::new("photo-id", "unique-id", 300, 200)]);
    expected_struct.data = MessageData::Photo(photos.clone());
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::Photo(photos.with_caption(
        Text::from("test photo caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn pinned_message() {
    let mut expected_struct = create_message_struct();
    let mut pinned_message = create_message_struct();
    pinned_message.data = MessageData::Text(Text::from("text"));
    expected_struct.data = MessageData::PinnedMessage(MaybeInaccessibleMessage::Message(Box::new(pinned_message)));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn poll() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Poll(Poll::Regular(RegularPoll {
        id: String::from("poll-id"),
        question: Text::from("Rust?"),
        options: vec![PollOption::new("Yes", 1000), PollOption::new("No", 0)],
        total_voter_count: 100,
        is_closed: true,
        is_anonymous: true,
        allows_multiple_answers: false,
        open_period: None,
        close_date: None,
    }));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn proximity_alert_triggered() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::ProximityAlertTriggered(MessageDataProximityAlert::new(
        100,
        User::new(1, "User 1", false),
        User::new(2, "User 2", false),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn refunded_payment() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::RefundedPayment(RefundedPayment::new("RUB", "payload", "charge-id", 100));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn sticker() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Sticker(
        Sticker::new("sticker-id", "unique-id", StickerType::Regular, 512, 512).with_is_animated(true),
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn story() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Story(Story::new(PrivateChat::new(1, "test"), 1));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn successful_payment() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::SuccessfulPayment(SuccessfulPayment::new(
        "RUB",
        "invoice payload",
        "provider-charge-id",
        "tg-charge-id",
        145,
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn supergroup_chat_created() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::SupergroupChatCreated;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn text() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Text(Text::from("text"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn unique_gift() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::UniqueGift(UniqueGiftInfo::new(
        UniqueGift::new(
            UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            String::from("base-name"),
            UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            String::from("name"),
            7,
            UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        ),
        UniqueGiftOrigin::Transfer,
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn unknown() {
    let expected_struct = create_message_struct();
    insta::assert_json_snapshot!(expected_struct);

    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Unknown(serde_json::json!({"unknown_value": {"key": "value"}}));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn users_shared() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::UsersShared(MessageDataUsersShared::new(1, [SharedUser::new(1)]));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn venue() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Venue(Venue::new("venue title", "venue address", Location::new(2.0, 1.0)));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video() {
    let mut expected_struct = create_message_struct();
    let video = MessageDataVideo::from(Video::new(3, "video-id", "unique-id", 2, 1));
    expected_struct.data = MessageData::Video(video.clone());
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::Video(video.with_caption(
        Text::from("test video caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video_note() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::VideoNote(VideoNote::new(1234, "video-note-id", "unique-id", 124));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn voice() {
    let mut expected_struct = create_message_struct();
    let voice = MessageDataVoice::from(Voice::new(123, "voice-id", "unique-id"));
    expected_struct.data = MessageData::Voice(voice.clone());
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.data = MessageData::Voice(voice.with_caption(
        Text::from("test voice caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video_chat_ended() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::VideoChatEnded(MessageDataVideoChatEnded::new(100));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video_chat_participants_invited() {
    let mut expected_struct = create_message_struct();
    expected_struct.data =
        MessageData::VideoChatParticipantsInvited(MessageDataVideoChatParticipantsInvited::from([User::new(
            1, "User", false,
        )]));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video_chat_scheduled() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::VideoChatScheduled(MessageDataVideoChatScheduled::new(100));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn video_chat_started() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::VideoChatStarted;
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn web_app_data() {
    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::WebAppData(WebAppData::new("web-app-data", "web-app-button-text"));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn write_access_allowed() {
    let mut expected_struct = create_message_struct();
    expected_struct.data =
        MessageData::WriteAccessAllowed(MessageDataWriteAccess::default().with_from_attachment_menu(true));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn copy_message() {
    let method = CopyMessage::new(1, 2, 3);
    assert_payload_eq!(POST JSON "copyMessage" => method.clone());
    let method = method
        .clone()
        .with_allow_paid_broadcast(true)
        .with_caption("caption")
        .with_disable_notification(true)
        .with_message_thread_id(1)
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_show_caption_above_media(true)
        .with_video_start_timestamp(200);
    assert_payload_eq!(POST JSON "copyMessage" => method.clone());
    let method = method.with_caption_entities([TextEntity::bold(1..2)]);
    assert_payload_eq!(POST JSON "copyMessage" => method);
}

#[test]
fn copy_messages() {
    let method = CopyMessages::new(1, 2, [3]);
    assert_payload_eq!(POST JSON "copyMessages" => method.clone());
    let method = method
        .clone()
        .with_disable_notification(true)
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_remove_caption(true);
    assert_payload_eq!(POST JSON "copyMessages" => method);
}

#[test]
fn delete_message() {
    assert_payload_eq!(POST JSON "deleteMessage" => DeleteMessage::new(1, 2));
}

#[test]
fn delete_messages() {
    assert_payload_eq!(POST JSON "deleteMessages" => DeleteMessages::new(1, [2]));
}

#[test]
fn edit_message_caption() {
    let method = EditMessageCaption::for_chat_message(1, 2);
    assert_payload_eq!(POST JSON "editMessageCaption" => method);
    let method = EditMessageCaption::for_chat_message(1, 2)
        .with_business_connection_id("c-id")
        .with_caption("caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
        .with_show_caption_above_media(true);
    assert_payload_eq!(POST JSON "editMessageCaption" => method);
    let method = EditMessageCaption::for_inline_message("msg-id");
    assert_payload_eq!(POST JSON "editMessageCaption" => method);
    let method = EditMessageCaption::for_inline_message("msg-id").with_caption_entities([TextEntity::bold(0..10)]);
    assert_payload_eq!(POST JSON "editMessageCaption" => method);
}

#[test]
fn edit_message_live_location() {
    let method = EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0);
    assert_payload_eq!(POST JSON "editMessageLiveLocation" => method);
    let method = EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0)
        .with_business_connection_id("c-id")
        .with_heading(100)
        .with_horizontal_accuracy(5.0)
        .with_live_period(10)
        .with_proximity_alert_radius(200)
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]);
    assert_payload_eq!(POST JSON "editMessageLiveLocation" => method);
    let method = EditMessageLiveLocation::for_inline_message("msg-id", 3.0, 4.0);
    assert_payload_eq!(POST JSON "editMessageLiveLocation" => method);
}

#[test]
fn edit_message_media() {
    let method = EditMessageMedia::for_chat_message(
        1,
        2,
        InputMedia::new(InputMediaType::for_photo(
            InputFile::file_id("file-id"),
            InputMediaPhoto::default(),
        ))
        .unwrap(),
    )
    .with_business_connection_id("c-id")
    .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
    .unwrap();
    assert_payload_eq!(POST FORM "editMessageMedia" => method);
    let method = EditMessageMedia::for_inline_message(
        "msg-id",
        InputMedia::new(InputMediaType::for_photo(
            InputFile::file_id("file-id"),
            InputMediaPhoto::default(),
        ))
        .unwrap(),
    );
    assert_payload_eq!(POST FORM "editMessageMedia" => method);
}

#[test]
fn edit_message_reply_markup() {
    let method = EditMessageReplyMarkup::for_chat_message(1, 2);
    assert_payload_eq!(POST JSON "editMessageReplyMarkup" => method);
    let markup = [[InlineKeyboardButton::for_url("text", "url")]];
    let method = EditMessageReplyMarkup::for_chat_message(1, 2).with_reply_markup(markup.clone());
    assert_payload_eq!(POST JSON "editMessageReplyMarkup" => method);
    let method = EditMessageReplyMarkup::for_inline_message("msg-id");
    assert_payload_eq!(POST JSON "editMessageReplyMarkup" => method);
    let method = EditMessageReplyMarkup::for_inline_message("msg-id")
        .with_business_connection_id("c-id")
        .with_reply_markup(markup);
    assert_payload_eq!(POST JSON "editMessageReplyMarkup" => method);
}

#[test]
fn edit_message_text() {
    let method = EditMessageText::for_chat_message(1, 2, "text");
    assert_payload_eq!(POST JSON "editMessageText" => method);
    let method = EditMessageText::for_chat_message(1, 2, "text")
        .with_business_connection_id("c-id")
        .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
        .with_parse_mode(ParseMode::Markdown)
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]);
    assert_payload_eq!(POST JSON "editMessageText" => method);
    let method = EditMessageText::for_inline_message("msg-id", "text");
    assert_payload_eq!(POST JSON "editMessageText" => method);
    let method = EditMessageText::for_inline_message("msg-id", "text").with_entities([TextEntity::bold(0..4)]);
    assert_payload_eq!(POST JSON "editMessageText" => method);
}

#[test]
fn forward_message() {
    let method = ForwardMessage::new(1, 2, 3);
    assert_payload_eq!(POST JSON "forwardMessage" => method);
    let method = ForwardMessage::new(1, 2, 3)
        .with_disable_notification(true)
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_video_start_timestamp(200);
    assert_payload_eq!(POST JSON "forwardMessage" => method);
}

#[test]
fn forward_messages() {
    let method = ForwardMessages::new(1, 2, [3]);
    assert_payload_eq!(POST JSON "forwardMessages" => method);
    let method = ForwardMessages::new(1, 2, [3])
        .with_disable_notification(true)
        .with_message_thread_id(1)
        .with_protect_content(true);
    assert_payload_eq!(POST JSON "forwardMessages" => method);
}

#[test]
fn send_message() {
    let method = SendMessage::new(1, "text");
    assert_payload_eq!(POST JSON "sendMessage" => method);
    let method = SendMessage::new(1, "text")
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_disable_notification(true)
        .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_parse_mode(ParseMode::Markdown)
        .with_entities(vec![TextEntity::bold(0..2)])
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1));
    assert_payload_eq!(POST JSON "sendMessage" => method);
}

#[test]
fn stop_message_live_location() {
    let method = StopMessageLiveLocation::for_chat_message(1, 2);
    assert_payload_eq!(POST JSON "stopMessageLiveLocation" => method);
    let method = StopMessageLiveLocation::for_chat_message(1, 2)
        .with_business_connection_id("c-id")
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]);
    assert_payload_eq!(POST JSON "stopMessageLiveLocation" => method);
    let method = StopMessageLiveLocation::for_inline_message("msg-id");
    assert_payload_eq!(POST JSON "stopMessageLiveLocation" => method);
}

#[test]
fn message_origin_channel() {
    insta::assert_json_snapshot!(MessageOrigin::from(MessageOriginChannel::new(
        ChannelChat::new(1, "test"),
        0,
        1
    )));
}

#[test]
fn message_origin_chat() {
    insta::assert_json_snapshot!(MessageOrigin::from(MessageOriginChat::new(
        0,
        GroupChat::new(1, "test")
    )));
}

#[test]
fn message_origin_hidden_user() {
    insta::assert_json_snapshot!(MessageOrigin::from(MessageOriginHiddenUser::new(0, "test")));
}

#[test]
fn message_origin_user() {
    insta::assert_json_snapshot!(MessageOrigin::from(MessageOriginUser::new(
        0,
        User::new(1, "test", false)
    )));
}

#[test]
fn text_quote() {
    insta::assert_json_snapshot!(TextQuote::new(0, "test"));
    insta::assert_json_snapshot!(
        TextQuote::new(
            0,
            Text::from("test").with_entities(TextEntities::from_iter([TextEntity::bold(0..2)])),
        )
        .with_is_manual(true)
    );
}

fn create_origin() -> MessageOriginHiddenUser {
    MessageOriginHiddenUser::new(1, "test")
}

#[test]
fn external_reply_info_animation() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Animation::new(10, "file-id", "file-unique-id", 20, 30),
        origin
    ));
}

#[test]
fn external_reply_info_audio() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Audio::new(10, "file-id", "file-unique-id"),
        origin
    ));
}

#[test]
fn external_reply_info_checklist() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(Checklist::new([], "test"), origin));
}

#[test]
fn external_reply_info_contact() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Contact::new("first-name", "+79001230099"),
        origin
    ));
}

#[test]
fn external_reply_info_dice() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(Dice::new(DiceType::Basketball, 1), origin));
}

#[test]
fn external_reply_info_document() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Document::new("file-id", "file-unique-id"),
        origin
    ));
}

#[test]
fn external_reply_info_game() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Game::new(
            "description",
            [PhotoSize::new("file-id", "file-unique-id", 10, 20)],
            "title",
        ),
        origin,
    ));
}

#[test]
fn external_reply_info_giveaway() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Giveaway::new([ChannelChat::new(1, "test")], 1, 1),
        origin
    ));
}

#[test]
fn external_reply_info_giveaway_winners() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        GiveawayWinners::new(ChannelChat::new(1, "test"), 1, 1, [User::new(1, "test", false)], 1),
        origin,
    ));
}

#[test]
fn external_reply_info_invoice() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Invoice::new("RUB", "description", "start-parameter", "title", 1),
        origin,
    ));
}

#[test]
fn external_reply_info_location() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(Location::new(1.0, 2.0), origin));
}

#[test]
fn external_reply_info_paid_media() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        PaidMediaInfo::new(100, [PaidMedia::Preview(PaidMediaPreview::default())]),
        origin,
    ));
}

#[test]
fn external_reply_info_photo() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        vec![PhotoSize::new("file-id", "file-unique-id", 10, 20)],
        origin
    ));
}

#[test]
fn external_reply_info_poll() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Poll::from(
            RegularPoll::new("poll-id", "Rust?")
                .with_is_anonymous(true)
                .with_is_closed(true)
                .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                .with_total_voter_count(1000),
        ),
        origin,
    ));
}

#[test]
fn external_reply_info_sticker() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 10, 20),
        origin,
    ));
}

#[test]
fn external_reply_info_story() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        ExternalReplyData::Story(Story::new(PrivateChat::new(1, "test"), 1)),
        origin,
    ));
}

#[test]
fn external_reply_info_venue() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Venue::new("title", "address", Location::new(1.0, 2.0)),
        origin
    ));
}

#[test]
fn external_reply_info_video() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Video::new(10, "file-id", "file-unique-id", 20, 30),
        origin
    ));
}

#[test]
fn external_reply_info_video_note() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        VideoNote::new(10, "file-id", "file-unique-id", 20),
        origin.clone()
    ));
}

#[test]
fn external_reply_info_voice() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        Voice::new(10, "file-id", "file-unique-id"),
        origin.clone()
    ));
}

#[test]
fn external_reply_info_unknown() {
    let origin = create_origin();
    insta::assert_json_snapshot!(ExternalReplyInfo::new(
        ExternalReplyData::Unknown(serde_json::json!({"key": "value"})),
        origin.clone(),
    ));
    insta::assert_json_snapshot!(
        ExternalReplyInfo::new(ExternalReplyData::Unknown(serde_json::json!({"key": "value"})), origin)
            .with_chat(ChannelChat::new(1, "test"))
            .with_has_media_spoiler(true)
            .with_link_preview_options(LinkPreviewOptions::default())
            .with_message_id(1)
    );
}

fn create_message_struct_with_chat(chat: Chat) -> Message {
    Message::new(
        1,
        0,
        chat,
        MessageData::Unknown(serde_json::json!({})),
        MessageSender::Unknown,
    )
}

#[test]
fn channel_chat() {
    let chat = Chat::Channel(ChannelChat::new(1, "Channel"));
    let mut expected_struct = create_message_struct_with_chat(chat);
    expected_struct.author_signature = Some(String::from("test"));
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_none());
    assert!(expected_struct.sender.get_user_id().is_none());
    assert!(expected_struct.sender.get_user_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn group_chat() {
    let chat = Chat::Group(GroupChat::new(1, "Group"));
    let mut expected_struct = create_message_struct_with_chat(chat.clone());
    expected_struct.sender = User::new(1, "User", false).with_username("test").into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
    assert_eq!(expected_struct.sender.get_user_username().unwrap(), "test");
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct.sender = chat.clone().into();
    assert_eq!(expected_struct.sender.get_chat().unwrap(), &chat);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn private_chat() {
    let chat = Chat::Private(PrivateChat::new(1, "Target"));
    let mut expected_struct = create_message_struct_with_chat(chat);
    expected_struct.sender = User::new(1, "Target", false).into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
    assert!(expected_struct.sender.get_user_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn supergroup_chat() {
    let chat = Chat::Supergroup(SupergroupChat::new(1, "Chat"));
    let mut expected_struct = create_message_struct_with_chat(chat);
    expected_struct.sender = User::new(1, "User", false).into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
    assert!(expected_struct.sender.get_user_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn maybe_inaccesible() {
    let msg: MaybeInaccessibleMessage = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 0,
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
    }))
    .unwrap();
    match msg {
        MaybeInaccessibleMessage::InaccessibleMessage(InaccessibleMessage { chat, message_id }) => {
            assert_eq!(chat.get_id(), 1);
            assert_eq!(message_id, 1);
        }
        MaybeInaccessibleMessage::Message(_) => panic!("Unexpected message variant"),
    };

    let msg: MaybeInaccessibleMessage = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
    }))
    .unwrap();
    match msg {
        MaybeInaccessibleMessage::InaccessibleMessage(_) => panic!("Unexpected message variant"),
        MaybeInaccessibleMessage::Message(msg) => {
            assert_eq!(msg.id, 1);
        }
    };
}

#[test]
fn external_reply() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text",
        "external_reply": {
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "animation": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 20,
                "width": 30
            }
        }
    }))
    .unwrap();
    assert!(msg.external_reply.is_some());
}

#[test]
fn quote() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text",
        "quote": {
            "position": 0,
            "text": "test"
        }
    }))
    .unwrap();
    assert!(msg.quote.is_some());
}

#[test]
fn get_text() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text"
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "text");
}

#[test]
fn get_text_from_audio() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test audio caption",
        "audio": {
            "file_id": "file-id",
            "file_unique_id": "unique-id",
            "duration": 243
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test audio caption");
}

#[test]
fn get_text_from_document() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test document caption",
        "document": {
            "file_id": "file-id",
            "file_unique_id": "unique-id",
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test document caption");
}

#[test]
fn get_text_from_photo() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test photo caption",
        "photo": [{
            "file_id": "photo-id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }]
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test photo caption");
}

#[test]
fn get_text_from_video() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test video caption",
        "video": {
            "file_id": "video-id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test video caption");
}

#[test]
fn get_text_from_voice() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test voice caption",
        "voice": {
            "file_id": "voice-id",
            "file_unique_id": "unique-id",
            "duration": 123
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test voice caption");
}

#[test]
fn get_text_returns_none() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "group_chat_created": true
    }))
    .unwrap();
    assert!(msg.get_text().is_none());
}

#[test]
fn is_edited() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "edit_date": 1
    }))
    .unwrap();
    assert!(msg.is_edited());

    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test"
    }))
    .unwrap();
    assert!(!msg.is_edited());
}

#[test]
fn link_peview_options() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "link_preview_options": {
            "is_disabled": true
        }
    }))
    .unwrap();
    let options = data
        .link_preview_options
        .as_ref()
        .expect("link_preview_options is empty");
    assert!(options.is_disabled.unwrap());
    let data = data.with_link_preview_options(LinkPreviewOptions::default());
    assert!(data.link_preview_options.as_ref().unwrap().is_disabled.is_none());
}

#[test]
fn reply_to_checklist_task_id() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_checklist_task_id": 1,
    }))
    .unwrap();
    if let Some(x) = msg.reply_to_checklist_task_id {
        assert_eq!(x, 1);
    } else {
        panic!(
            "Unexpected reply_to_checklist_task_id data: {:?}",
            msg.reply_to_checklist_task_id
        );
    }
}

#[test]
fn reply_to_message() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "text": "test"
        }
    }))
    .unwrap();
    if let Some(ReplyTo::Message(msg)) = msg.reply_to {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn reply_to_message_with_empty_data() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        }
    }))
    .unwrap();
    assert!(data.reply_to.is_some());
}

#[test]
fn reply_to_story() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_story": {
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "id": 1
        }
    }))
    .unwrap();
    if let Some(ReplyTo::Story(story)) = msg.reply_to {
        assert_eq!(story.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn sender_boost_count() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "sender_boost_count": 1
    }))
    .unwrap();
    assert_eq!(data.sender_boost_count.unwrap(), 1);
}

#[test]
fn via_bot() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "via_bot": {
            "id": 3,
            "is_bot": true,
            "first_name": "example",
            "last_name": "bot",
            "username": "example_bot"
        }
    }))
    .unwrap();
    let bot = data.via_bot.expect("via_bot is empty");
    assert_eq!(bot.id, 3);
    assert_eq!(bot.first_name, "example");
    assert_eq!(bot.last_name.unwrap(), "bot");
    assert_eq!(&bot.username.unwrap(), "example_bot");
}

#[test]
fn edit_message_result() {
    let expected_struct = EditMessageResult::Message(Message::new(
        1,
        0,
        SupergroupChat::new(1, "test"),
        MessageData::Unknown(serde_json::json!({})),
        User::new(1, "test", false),
    ));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = EditMessageResult::Bool(true);
    insta::assert_json_snapshot!(expected_struct);
}

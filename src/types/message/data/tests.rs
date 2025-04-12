use crate::types::{
    Animation,
    Audio,
    BackgroundType,
    ChannelChat,
    ChatBackground,
    Contact,
    Document,
    EncryptedCredentials,
    ForumTopicIconColor,
    Game,
    Gift,
    GiftInfo,
    Giveaway,
    GiveawayCompleted,
    GiveawayCreated,
    GiveawayWinners,
    Invoice,
    Location,
    MaybeInaccessibleMessage,
    Message,
    MessageData,
    MessageDataAudio,
    MessageDataAutoDeleteTimer,
    MessageDataChatShared,
    MessageDataDocument,
    MessageDataForumTopicCreated,
    MessageDataForumTopicEdited,
    MessageDataPaidMessagePriceChanged,
    MessageDataPhoto,
    MessageDataProximityAlert,
    MessageDataUsersShared,
    MessageDataVideo,
    MessageDataVideoChatEnded,
    MessageDataVideoChatParticipantsInvited,
    MessageDataVideoChatScheduled,
    MessageDataVoice,
    MessageDataWriteAccess,
    PaidMedia,
    PaidMediaInfo,
    PaidMediaPreview,
    PassportData,
    PhotoSize,
    Poll,
    PollOption,
    PrivateChat,
    RefundedPayment,
    RegularPoll,
    SharedUser,
    Sticker,
    StickerType,
    Story,
    SuccessfulPayment,
    SupergroupChat,
    Text,
    TextEntities,
    TextEntity,
    UniqueGift,
    UniqueGiftBackdrop,
    UniqueGiftBackdropColors,
    UniqueGiftInfo,
    UniqueGiftModel,
    UniqueGiftOrigin,
    UniqueGiftSymbol,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
    tests::assert_json_eq,
};

fn create_message_struct() -> Message {
    Message::new(
        1,
        1,
        SupergroupChat::new(1, "Chat"),
        MessageData::Unknown(serde_json::json!({})),
        User::new(1, "User", false),
    )
}

fn create_message_value() -> serde_json::Value {
    serde_json::json!({
        "message_id": 1,
        "date": 1,
        "from": {"id": 1, "first_name": "User", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "Chat"},
        "has_protected_content": false,
        "is_automatic_forward": false
    })
}

#[test]
fn animation() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Animation(Animation::new(243, "file-id", "unique-id", 200, 200));
    expected_value["animation"] = serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "width": 200,
        "height": 200,
        "duration": 243
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn audio() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let audio = MessageDataAudio::from(Audio::new(243, "file-id", "unique-id"));
    expected_struct.data = MessageData::Audio(audio.clone());
    expected_value["audio"] = serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "duration": 243
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Audio(audio.with_caption(
        Text::from("test audio caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    expected_value["caption"] = serde_json::json!("test audio caption");
    expected_value["caption_entities"] = serde_json::json!(
        [
            {
                "type": "bold",
                "offset": 0,
                "length": 4
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn auto_delete_timer_changed() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::AutoDeleteTimerChanged(MessageDataAutoDeleteTimer::new(10000));
    expected_value["message_auto_delete_timer_changed"] = serde_json::json!({
        "message_auto_delete_time": 10000,
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn boost_added() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::BoostAdded(1);
    expected_value["boost_added"] = serde_json::json!({
        "boost_count": 1
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn channel_chat_created() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ChannelChatCreated;
    expected_value["channel_chat_created"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn chat_background_set() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ChatBackgroundSet(ChatBackground::from(BackgroundType::Wallpaper {
        dark_theme_dimming: 100,
        document: Document::new("file-id", "file-unique-id"),
        is_blurred: Some(true),
        is_moving: Some(false),
    }));
    expected_value["chat_background_set"] = serde_json::json!({
        "type": {
            "type": "wallpaper",
            "dark_theme_dimming": 100,
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
            },
            "is_blurred": true,
            "is_moving": false,
        }
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());
}

#[test]
fn chat_shared() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ChatShared(MessageDataChatShared::new(1, 1));
    expected_value["chat_shared"] = serde_json::json!({
        "request_id": 1,
        "chat_id": 1,
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::ChatShared(
        MessageDataChatShared::new(1, 1)
            .with_photo([PhotoSize::new("file-id", "file-unique-id", 100, 100)])
            .with_title("title")
            .with_username("username"),
    );
    expected_value["chat_shared"] = serde_json::json!({
        "request_id": 1,
        "chat_id": 1,
        "photo": [
            {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 100,
                "width": 100
            }
        ],
        "title": "title",
        "username": "username"
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn connected_website() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ConnectedWebsite(String::from("http://example.com"));
    expected_value["connected_website"] = serde_json::json!("http://example.com");
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn contact() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Contact(Contact::new("User", "+79001231212"));
    expected_value["contact"] = serde_json::json!({
        "phone_number": "+79001231212",
        "first_name": "User"
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn delete_chat_photo() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::DeleteChatPhoto;
    expected_value["delete_chat_photo"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn dice() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let raw_dice = serde_json::json!({
        "value": 1,
        "emoji": "🎯"
    });
    expected_struct.data = MessageData::Dice(serde_json::from_value(raw_dice.clone()).unwrap());
    expected_value["dice"] = raw_dice;
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn document() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let document = MessageDataDocument::from(Document::new("file-id", "unique-id"));
    expected_struct.data = MessageData::Document(document.clone());
    expected_value["document"] = serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Document(document.with_caption(
        Text::from("test document caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    expected_value["caption"] = serde_json::json!("test document caption");
    expected_value["caption_entities"] = serde_json::json!(
        [
            {
                "type": "bold",
                "offset": 0,
                "length": 4
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn forum_topic_closed() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ForumTopicClosed;
    expected_value["forum_topic_closed"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn forum_topic_created() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ForumTopicCreated(MessageDataForumTopicCreated::new(
        ForumTopicIconColor::LightGreen,
        "topic-name",
    ));
    expected_value["forum_topic_created"] = serde_json::json!({
        "name": "topic-name",
        "icon_color": 9367192,
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn forum_topic_edited() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ForumTopicEdited(MessageDataForumTopicEdited::default().with_name("new-name"));
    expected_value["forum_topic_edited"] = serde_json::json!({
        "name": "new-name"
    });

    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn forum_topic_reopened() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ForumTopicReopened;
    expected_value["forum_topic_reopened"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn game() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Game(Game::new("Description", [], "Game"));
    expected_value["game"] = serde_json::json!({
        "title": "Game",
        "description": "Description",
        "photo": []
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn gift() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Gift(GiftInfo::new(Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    )));
    expected_value["gift"] = serde_json::json!({
        "gift": {
            "id": "id",
            "sticker": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "type": "regular",
                "is_animated": false,
                "is_video": false,
                "height": 512,
                "width": 512,
            },
            "star_count": 100,
        }
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn general_forum_topic_hidden() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GeneralForumTopicHidden;
    expected_value["general_forum_topic_hidden"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn general_forum_topic_unhidden() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GeneralForumTopicUnhidden;
    expected_value["general_forum_topic_unhidden"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn giveaway() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Giveaway(Giveaway::new([ChannelChat::new(1, "test")], 0, 1));
    expected_value["giveaway"] = serde_json::json!({
        "chats": [
            {
                "type": "channel",
                "id": 1,
                "title": "test"
            }
        ],
        "winners_selection_date": 0,
        "winner_count": 1
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn giveaway_created() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GiveawayCreated(GiveawayCreated::default());
    expected_value["giveaway_created"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn giveaway_completed() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GiveawayCompleted(GiveawayCompleted::new(1));
    expected_value["giveaway_completed"] = serde_json::json!({
        "winner_count": 1
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn giveaway_winners() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GiveawayWinners(GiveawayWinners::new(
        ChannelChat::new(1, "test"),
        1,
        1,
        [User::new(1, "test", false)],
        0,
    ));
    expected_value["giveaway_winners"] = serde_json::json!({
        "chat": {
            "type": "channel",
            "id": 1,
            "title": "test"
        },
        "giveaway_message_id": 1,
        "winner_count": 1,
        "winners": [
            {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        ],
        "winners_selection_date": 0
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn group_chat_created() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::GroupChatCreated;
    expected_value["group_chat_created"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn invoice() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Invoice(Invoice::new(
        "RUB",
        "invoice description",
        "invoice start parameter",
        "invoice title",
        100,
    ));
    expected_value["invoice"] = serde_json::json!({
        "title": "invoice title",
        "description": "invoice description",
        "start_parameter": "invoice start parameter",
        "currency": "RUB",
        "total_amount": 100
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn left_chat_member() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::LeftChatMember(User::new(1, "User", false));
    expected_value["left_chat_member"] = serde_json::json!({
        "id": 1,
        "first_name": "User",
        "is_bot": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn location() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Location(Location {
        longitude: 3.0,
        latitude: 2.0,
        horizontal_accuracy: None,
        live_period: None,
        heading: None,
        proximity_alert_radius: None,
    });
    expected_value["location"] = serde_json::json!({
        "latitude": 2.0,
        "longitude": 3.0
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn migrate_from_chat_id() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::MigrateFromChatId(124);
    expected_value["migrate_from_chat_id"] = serde_json::json!(124);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn migrate_to_chat_id() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::MigrateToChatId(124);
    expected_value["migrate_to_chat_id"] = serde_json::json!(124);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn new_chat_members() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::NewChatMembers(vec![User::new(1, "User", false)]);
    expected_value["new_chat_members"] = serde_json::json!(
        [
            {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn new_chat_photo() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::NewChatPhoto(vec![PhotoSize::new("photo file id", "unique-id", 300, 200)]);
    expected_value["new_chat_photo"] = serde_json::json!(
        [
            {
                "file_id": "photo file id",
                "file_unique_id": "unique-id",
                "width": 200,
                "height": 300
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn new_chat_title() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::NewChatTitle(String::from("new chat title"));
    expected_value["new_chat_title"] = serde_json::json!("new chat title");
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn paid_media() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::PaidMedia(PaidMediaInfo::new(
        100,
        [PaidMedia::Preview(PaidMediaPreview::default())],
    ));
    expected_value["paid_media"] = serde_json::json!({
        "star_count": 100,
        "paid_media": [
            {
                "type": "preview"
            }
        ]
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn paid_message_price_changed() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::PaidMessagePriceChanged(MessageDataPaidMessagePriceChanged {
        paid_message_star_count: 1,
    });
    expected_value["paid_message_price_changed"] = serde_json::json!({
        "paid_message_star_count": 1,

    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn passport_data() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::PassportData(PassportData {
        data: vec![],
        credentials: EncryptedCredentials {
            data: String::from("data"),
            hash: String::from("hash"),
            secret: String::from("secret"),
        },
    });
    expected_value["passport_data"] = serde_json::json!({
        "data": [],
        "credentials": {
            "data": "data",
            "hash": "hash",
            "secret": "secret"
        }
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn photo() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let photos = MessageDataPhoto::from([PhotoSize::new("photo-id", "unique-id", 300, 200)]);

    expected_struct.data = MessageData::Photo(photos.clone());
    expected_value["photo"] = serde_json::json!([{
        "file_id": "photo-id",
        "file_unique_id": "unique-id",
        "width": 200,
        "height": 300
    }]);
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Photo(photos.with_caption(
        Text::from("test photo caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    expected_value["caption"] = serde_json::json!("test photo caption");
    expected_value["caption_entities"] = serde_json::json!(
        [
            {
                "type": "bold",
                "offset": 0,
                "length": 4
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn pinned_message() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let mut pinned_message = create_message_struct();
    pinned_message.data = MessageData::Text(Text::from("text"));
    expected_struct.data = MessageData::PinnedMessage(MaybeInaccessibleMessage::Message(Box::new(pinned_message)));
    expected_value["pinned_message"] = serde_json::json!({
        "message_id": 1,
        "date": 1,
        "from": {"id": 1, "first_name": "User", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "Chat"},
        "text": "text",
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn poll() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

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
    expected_value["poll"] = serde_json::json!({
        "id": "poll-id",
        "question": "Rust?",
        "options": [
            {"text": "Yes", "voter_count": 1000},
            {"text": "No", "voter_count": 0}
        ],
        "is_closed": true,
        "total_voter_count": 100,
        "is_anonymous": true,
        "type": "regular",
        "allows_multiple_answers": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn proximity_alert_triggered() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::ProximityAlertTriggered(MessageDataProximityAlert::new(
        100,
        User::new(1, "User 1", false),
        User::new(2, "User 2", false),
    ));
    expected_value["proximity_alert_triggered"] = serde_json::json!({
        "traveler": {
            "id": 1,
            "first_name": "User 1",
            "is_bot": false
        },
        "watcher": {
            "id": 2,
            "first_name": "User 2",
            "is_bot": false
        },
        "distance": 100,
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn refunded_payment() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::RefundedPayment(RefundedPayment::new("RUB", "payload", "charge-id", 100));
    expected_value["refunded_payment"] = serde_json::json!({
        "currency": "RUB",
        "invoice_payload": "payload",
        "telegram_payment_charge_id": "charge-id",
        "total_amount": 100,
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn sticker() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Sticker(
        Sticker::new("sticker-id", "unique-id", StickerType::Regular, 512, 512).with_is_animated(true),
    );
    expected_value["sticker"] = serde_json::json!({
        "file_id": "sticker-id",
        "file_unique_id": "unique-id",
        "type": "regular",
        "width": 512,
        "height": 512,
        "is_animated": true,
        "is_video": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn story() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Story(Story::new(PrivateChat::new(1, "test"), 1));
    expected_value["story"] = serde_json::json!({
        "chat": {
            "first_name": "test",
            "id": 1,
            "type": "private"
        },
        "id": 1
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn successful_payment() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::SuccessfulPayment(SuccessfulPayment::new(
        "RUB",
        "invoice payload",
        "provider-charge-id",
        "tg-charge-id",
        145,
    ));
    expected_value["successful_payment"] = serde_json::json!({
        "currency": "RUB",
        "total_amount": 145,
        "invoice_payload": "invoice payload",
        "telegram_payment_charge_id": "tg-charge-id",
        "provider_payment_charge_id": "provider-charge-id"
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn supergroup_chat_created() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::SupergroupChatCreated;
    expected_value["supergroup_chat_created"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn text() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Text(Text::from("text"));
    expected_value["text"] = serde_json::json!("text");
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn unique_gift() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::UniqueGift(UniqueGiftInfo::new(
        UniqueGift {
            backdrop: UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            base_name: String::from("base-name"),
            model: UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            name: String::from("name"),
            number: 7,
            symbol: UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        },
        UniqueGiftOrigin::Transfer,
    ));
    expected_value["unique_gift"] = serde_json::json!({
        "gift": {
            "backdrop": {
                "colors": {
                    "center_color": 1,
                    "edge_color": 2,
                    "symbol_color": 3,
                    "text_color": 4,
                },
                "name": "name",
                "rarity_per_mille": 5,
            },
            "base_name": "base-name",
            "model": {
                "name": "name",
                "rarity_per_mille": 6,
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
            },
            "name": "name",
            "number": 7,
            "symbol": {
                "name": "name",
                "rarity_per_mille": 8,
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
            },
        },
        "origin": "transfer",
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn unknown() {
    let expected_struct = create_message_struct();
    let expected_value = create_message_value();
    assert_json_eq(expected_struct, expected_value);

    let mut expected_struct = create_message_struct();
    expected_struct.data = MessageData::Unknown(serde_json::json!({"unknown_value": {"key": "value"}}));
    let mut expected_value = create_message_value();
    expected_value["unknown_value"] = serde_json::json!({"key": "value"});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn users_shared() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::UsersShared(MessageDataUsersShared::new(1, [SharedUser::new(1)]));
    expected_value["users_shared"] = serde_json::json!({
        "request_id": 1,
        "users": [
            {
                "user_id": 1
            }
        ],
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn venue() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Venue(Venue::new("venue title", "venue address", Location::new(2.0, 1.0)));
    expected_value["venue"] = serde_json::json!({
        "location": {
            "latitude": 2.0,
            "longitude": 1.0
        },
        "title": "venue title",
        "address": "venue address"
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let video = MessageDataVideo::from(Video::new(3, "video-id", "unique-id", 2, 1));
    expected_struct.data = MessageData::Video(video.clone());
    expected_value["video"] = serde_json::json!({
        "file_id": "video-id",
        "file_unique_id": "unique-id",
        "width": 1,
        "height": 2,
        "duration": 3
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Video(video.with_caption(
        Text::from("test video caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    expected_value["caption"] = serde_json::json!("test video caption");
    expected_value["caption_entities"] = serde_json::json!(
        [
            {
                "type": "bold",
                "offset": 0,
                "length": 4
            }
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video_note() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VideoNote(VideoNote::new(1234, "video-note-id", "unique-id", 124));
    expected_value["video_note"] = serde_json::json!({
        "file_id": "video-note-id",
        "file_unique_id": "unique-id",
        "length": 124,
        "duration": 1234
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn voice() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    let voice = MessageDataVoice::from(Voice::new(123, "voice-id", "unique-id"));
    expected_struct.data = MessageData::Voice(voice.clone());
    expected_value["voice"] = serde_json::json!({
        "file_id": "voice-id",
        "file_unique_id": "unique-id",
        "duration": 123
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Voice(voice.with_caption(
        Text::from("test voice caption").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
    ));
    expected_value["caption"] = serde_json::json!("test voice caption");
    expected_value["caption_entities"] = serde_json::json!(
        [
            {
                "type": "bold",
                "offset": 0,
                "length": 4
            },
        ]
    );
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video_chat_ended() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VideoChatEnded(MessageDataVideoChatEnded::new(100));
    expected_value["video_chat_ended"] = serde_json::json!({"duration": 100});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video_chat_participants_invited() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data =
        MessageData::VideoChatParticipantsInvited(MessageDataVideoChatParticipantsInvited::from([User::new(
            1, "User", false,
        )]));
    expected_value["video_chat_participants_invited"] = serde_json::json!({
        "users": [
            {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            }
        ]
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video_chat_scheduled() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VideoChatScheduled(MessageDataVideoChatScheduled::new(100));
    expected_value["video_chat_scheduled"] = serde_json::json!({"start_date": 100});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn video_chat_started() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VideoChatStarted;
    expected_value["video_chat_started"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn web_app_data() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::WebAppData(WebAppData::new("web-app-data", "web-app-button-text"));
    expected_value["web_app_data"] = serde_json::json!({
        "data": "web-app-data",
        "button_text": "web-app-button-text"
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn write_access_allowed() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data =
        MessageData::WriteAccessAllowed(MessageDataWriteAccess::default().with_from_attachment_menu(true));
    expected_value["write_access_allowed"] = serde_json::json!({
        "from_attachment_menu": true
    });
    assert_json_eq(expected_struct, expected_value);
}

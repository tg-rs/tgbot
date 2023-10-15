use crate::types::{
    tests::assert_json_eq,
    Animation,
    Audio,
    Chat,
    Contact,
    Document,
    EncryptedCredentials,
    Game,
    Invoice,
    Location,
    Message,
    MessageData,
    MessageSender,
    PassportData,
    PhotoSize,
    Poll,
    PollOption,
    ProximityAlertTriggered,
    RegularPoll,
    Sticker,
    SuccessfulPayment,
    SupergroupChat,
    Text,
    TextEntities,
    TextEntity,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
};

fn create_message_struct() -> Message {
    Message {
        id: 1,
        date: 0,
        edit_date: None,
        sender: MessageSender::User(User {
            id: 1,
            is_bot: false,
            first_name: String::from("User"),
            last_name: None,
            username: None,
            language_code: None,
        }),
        chat: Chat::Supergroup(SupergroupChat {
            id: 1,
            title: String::from("Chat"),
            username: None,
            photo: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            linked_chat_id: None,
            location: None,
            has_protected_content: None,
        }),
        author_signature: None,
        has_protected_content: false,
        forward: None,
        is_automatic_forward: false,
        reply_to: None,
        via_bot: None,
        media_group_id: None,
        reply_markup: None,
        data: MessageData::Empty,
    }
}

fn create_message_value() -> serde_json::Value {
    serde_json::json!({
        "message_id": 1,
        "date": 0,
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

    expected_struct.data = MessageData::Animation(Animation {
        file_id: String::from("file-id"),
        file_unique_id: String::from("unique-id"),
        width: 200,
        height: 200,
        duration: 243,
        thumb: None,
        file_name: None,
        mime_type: None,
        file_size: None,
    });
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

    let audio = Audio {
        file_id: String::from("file-id"),
        file_unique_id: String::from("unique-id"),
        duration: 243,
        performer: None,
        title: None,
        file_name: None,
        mime_type: None,
        file_size: None,
        thumb: None,
    };
    expected_struct.data = MessageData::Audio {
        caption: None,
        data: audio.clone(),
    };
    expected_value["audio"] = serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "duration": 243
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Audio {
        caption: Some(Text {
            data: String::from("test audio caption"),
            entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
        }),
        data: audio,
    };
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

    expected_struct.data = MessageData::AutoDeleteTimerChanged { time: 10000 };
    expected_value["message_auto_delete_timer_changed"] = serde_json::json!({
        "message_auto_delete_time": 10000,
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

    expected_struct.data = MessageData::Contact(Contact {
        phone_number: String::from("+79001231212"),
        first_name: String::from("User"),
        last_name: None,
        user_id: None,
        vcard: None,
    });
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

    let document = Document {
        file_id: String::from("file-id"),
        file_unique_id: String::from("unique-id"),
        thumb: None,
        file_name: None,
        mime_type: None,
        file_size: None,
    };
    expected_struct.data = MessageData::Document {
        caption: None,
        data: document.clone(),
    };
    expected_value["document"] = serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Document {
        caption: Some(Text {
            data: String::from("test document caption"),
            entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
        }),
        data: document,
    };
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
fn game() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Game(Game {
        title: String::from("game"),
        description: String::from("description"),
        photo: vec![],
        text: None,
        animation: None,
    });
    expected_value["game"] = serde_json::json!({
        "title": "game",
        "description": "description",
        "photo": []
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

    expected_struct.data = MessageData::Invoice(Invoice {
        title: String::from("invoice title"),
        description: String::from("invoice description"),
        start_parameter: String::from("invoice start parameter"),
        currency: String::from("RUB"),
        total_amount: 100,
    });
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

    expected_struct.data = MessageData::LeftChatMember(User {
        id: 1234,
        is_bot: false,
        first_name: String::from("test"),
        last_name: None,
        username: None,
        language_code: None,
    });
    expected_value["left_chat_member"] = serde_json::json!({
        "id": 1234,
        "first_name": "test",
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

    expected_struct.data = MessageData::NewChatMembers(vec![User {
        id: 1234,
        is_bot: false,
        first_name: String::from("test"),
        last_name: None,
        username: None,
        language_code: None,
    }]);
    expected_value["new_chat_members"] = serde_json::json!(
        [
            {
                "id": 1234,
                "first_name": "test",
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

    expected_struct.data = MessageData::NewChatPhoto(vec![PhotoSize {
        file_id: String::from("photo file id"),
        file_unique_id: String::from("unique-id"),
        width: 200,
        height: 300,
        file_size: None,
    }]);
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

    let photos = vec![PhotoSize {
        file_id: String::from("photo-id"),
        file_unique_id: String::from("unique-id"),
        width: 200,
        height: 300,
        file_size: None,
    }];
    expected_struct.data = MessageData::Photo {
        caption: None,
        data: photos.clone(),
    };
    expected_value["photo"] = serde_json::json!([{
        "file_id": "photo-id",
        "file_unique_id": "unique-id",
        "width": 200,
        "height": 300
    }]);
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Photo {
        caption: Some(Text {
            data: String::from("test photo caption"),
            entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
        }),
        data: photos.clone(),
    };
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
    pinned_message.data = MessageData::Text(Text {
        data: String::from("text"),
        entities: None,
    });
    expected_struct.data = MessageData::PinnedMessage(Box::new(pinned_message));
    expected_value["pinned_message"] = serde_json::json!({
        "message_id": 1,
        "date": 0,
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
        question: String::from("Rust?"),
        options: vec![
            PollOption {
                text: String::from("Yes"),
                voter_count: 1000,
            },
            PollOption {
                text: String::from("No"),
                voter_count: 0,
            },
        ],
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

    expected_struct.data = MessageData::ProximityAlertTriggered(ProximityAlertTriggered {
        traveler: User {
            id: 1,
            is_bot: false,
            first_name: String::from("firstname1"),
            last_name: None,
            username: None,
            language_code: None,
        },
        watcher: User {
            id: 2,
            is_bot: false,
            first_name: String::from("firstname2"),
            last_name: None,
            username: None,
            language_code: None,
        },
        distance: 100,
    });
    expected_value["proximity_alert_triggered"] = serde_json::json!({
        "traveler": {
            "id": 1,
            "first_name": "firstname1",
            "is_bot": false
        },
        "watcher": {
            "id": 2,
            "first_name": "firstname2",
            "is_bot": false
        },
        "distance": 100,
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn sticker() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Sticker(Sticker {
        file_id: String::from("sticker-id"),
        file_unique_id: String::from("unique-id"),
        width: 512,
        height: 512,
        thumb: None,
        emoji: None,
        set_name: None,
        mask_position: None,
        file_size: None,
        is_animated: true,
        is_video: false,
    });
    expected_value["sticker"] = serde_json::json!({
        "file_id": "sticker-id",
        "file_unique_id": "unique-id",
        "width": 512,
        "height": 512,
        "is_animated": true,
        "is_video": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn successful_payment() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::SuccessfulPayment(SuccessfulPayment {
        currency: String::from("RUB"),
        total_amount: 145,
        invoice_payload: String::from("invoice payload"),
        shipping_option_id: None,
        order_info: None,
        telegram_payment_charge_id: String::from("tg-charge-id"),
        provider_payment_charge_id: String::from("provider-charge-id"),
    });
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

    expected_struct.data = MessageData::Text(Text {
        data: String::from("text"),
        entities: None,
    });
    expected_value["text"] = serde_json::json!("text");
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn venue() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::Venue(Venue {
        location: Location {
            longitude: 1.0,
            latitude: 2.0,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        },
        title: String::from("venue title"),
        address: String::from("venue address"),
        foursquare_id: None,
        foursquare_type: None,
        google_place_id: None,
        google_place_type: None,
    });
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

    let video = Video {
        file_id: String::from("video-id"),
        file_unique_id: String::from("unique-id"),
        width: 1,
        height: 2,
        duration: 3,
        thumb: None,
        file_name: None,
        mime_type: None,
        file_size: None,
    };
    expected_struct.data = MessageData::Video {
        caption: None,
        data: video.clone(),
    };
    expected_value["video"] = serde_json::json!({
        "file_id": "video-id",
        "file_unique_id": "unique-id",
        "width": 1,
        "height": 2,
        "duration": 3
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Video {
        caption: Some(Text {
            data: String::from("test video caption"),
            entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
        }),
        data: video,
    };
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

    expected_struct.data = MessageData::VideoNote(VideoNote {
        file_id: String::from("video-note-id"),
        file_unique_id: String::from("unique-id"),
        length: 124,
        duration: 1234,
        thumb: None,
        file_size: None,
    });
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

    let voice = Voice {
        file_id: String::from("voice-id"),
        file_unique_id: String::from("unique-id"),
        duration: 123,
        mime_type: None,
        file_size: None,
    };
    expected_struct.data = MessageData::Voice {
        caption: None,
        data: voice.clone(),
    };
    expected_value["voice"] = serde_json::json!({
        "file_id": "voice-id",
        "file_unique_id": "unique-id",
        "duration": 123
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.data = MessageData::Voice {
        caption: Some(Text {
            data: String::from("test voice caption"),
            entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..4)])),
        }),
        data: voice.clone(),
    };
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
fn voice_chat_ended() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VoiceChatEnded { duration: 100 };
    expected_value["voice_chat_ended"] = serde_json::json!({"duration": 100});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn voice_chat_participants_invited() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VoiceChatParticipantsInvited {
        users: vec![User {
            id: 1,
            is_bot: false,
            first_name: String::from("firstname"),
            last_name: None,
            username: None,
            language_code: None,
        }],
    };
    expected_value["voice_chat_participants_invited"] = serde_json::json!({
        "users": [
            {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            }
        ]
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn voice_chat_scheduled() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VoiceChatScheduled { start_date: 100 };
    expected_value["voice_chat_scheduled"] = serde_json::json!({"start_date": 100});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn voice_chat_started() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::VoiceChatStarted;
    expected_value["voice_chat_started"] = serde_json::json!({});
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn web_app_data() {
    let mut expected_struct = create_message_struct();
    let mut expected_value = create_message_value();

    expected_struct.data = MessageData::WebAppData(WebAppData {
        data: String::from("web-app-data"),
        button_text: String::from("web-app-button-text"),
    });
    expected_value["web_app_data"] = serde_json::json!({
        "data": "web-app-data",
        "button_text": "web-app-button-text"
    });
    assert_json_eq(expected_struct, expected_value);
}

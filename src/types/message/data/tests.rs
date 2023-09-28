use crate::types::{DiceKind, Message, MessageData, Poll, TextEntity, TextEntityPosition};

#[test]
fn deserialize_animation() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "animation": {
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200,
            "duration": 243
        }
    }))
    .unwrap();
    if let MessageData::Animation(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
        assert_eq!(data.file_unique_id, "unique-id");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_audio_partial() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "audio": {
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
            "file_unique_id": "unique-id",
            "duration": 243
        }
    }))
    .unwrap();
    if let MessageData::Audio { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
        assert_eq!(data.file_unique_id, "unique-id");
        assert!(caption.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_audio_full() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test audio caption",
        "caption_entities": [
            {"type": "bold", "offset": 0, "length": 4},
        ],
        "audio": {
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
            "file_unique_id": "unique-id",
            "duration": 243
        }
    }))
    .unwrap();
    if let MessageData::Audio { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
        assert_eq!(data.file_unique_id, "unique-id");
        let caption = caption.unwrap();
        assert_eq!(caption.data, "test audio caption");
        assert_eq!(
            Vec::from(caption.entities.unwrap()),
            vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_auto_delete_timer_changed() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "message_auto_delete_timer_changed": {
            "message_auto_delete_time": 10000,
        }
    }))
    .unwrap();
    if let MessageData::AutoDeleteTimerChanged { time } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(time, 10000);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_channel_chat_created() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "channel_chat_created": true
    }))
    .unwrap();
    if let MessageData::ChannelChatCreated = msg.data {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_connected_website() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "connected_website": "http://example.com"
    }))
    .unwrap();
    if let MessageData::ConnectedWebsite(url) = msg.data {
        assert_eq!(url, "http://example.com");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_contact() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "contact": {
            "phone_number": "+79001231212",
            "first_name": "First name"
        }
    }))
    .unwrap();
    if let MessageData::Contact(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.phone_number, "+79001231212");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_delete_chat_photo() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "delete_chat_photo": true
    }))
    .unwrap();
    if let MessageData::DeleteChatPhoto = msg.data {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_dice() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "dice": {
            "value": 1,
            "emoji": "🎯"
        }
    }))
    .unwrap();
    if let MessageData::Dice(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.value(), 1);
        assert_eq!(data.kind(), DiceKind::Darts);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_document_partial() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "document": {
            "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
            "file_unique_id": "unique-id",
        }
    }))
    .unwrap();
    if let MessageData::Document { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA");
        assert_eq!(data.file_unique_id, "unique-id");
        assert!(caption.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_document_full() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test document caption",
        "caption_entities": [
            {"type": "bold", "offset": 0, "length": 4},
        ],
        "document": {
            "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
            "file_unique_id": "unique-id",
        }
    }))
    .unwrap();
    if let MessageData::Document { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA");
        assert_eq!(data.file_unique_id, "unique-id");
        let caption = caption.unwrap();
        assert_eq!(caption.data, "test document caption");
        assert_eq!(
            Vec::from(caption.entities.unwrap()),
            vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_game() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "game": {
            "title": "game",
            "description": "description",
            "photo": []
        }
    }))
    .unwrap();
    if let MessageData::Game(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.title, "game");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_group_chat_created() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "group_chat_created": true
    }))
    .unwrap();
    if let MessageData::GroupChatCreated = msg.data {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_invoice() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "invoice": {
            "title": "invoice title",
            "description": "invoice description",
            "start_parameter": "invoice start parameter",
            "currency": "RUB",
            "total_amount": 100
        }
    }))
    .unwrap();
    if let MessageData::Invoice(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.title, "invoice title");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_left_chat_member() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "left_chat_member": {
            "id": 1234,
            "first_name": "test",
            "is_bot": false
        }
    }))
    .unwrap();
    if let MessageData::LeftChatMember(data) = msg.data {
        assert_eq!(data.id, 1234);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[allow(clippy::float_cmp)]
#[test]
fn deserialize_location() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "location": {
            "latitude": 2.0,
            "longitude": 3.0
        }
    }))
    .unwrap();
    if let MessageData::Location(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.latitude, 2.0);
        assert_eq!(data.longitude, 3.0);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_migrate_from_chat_id() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "migrate_from_chat_id": 124
    }))
    .unwrap();
    if let MessageData::MigrateFromChatId(chat_id) = msg.data {
        assert_eq!(chat_id, 124);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_migrate_to_chat_id() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "migrate_to_chat_id": 124
    }))
    .unwrap();
    if let MessageData::MigrateToChatId(chat_id) = msg.data {
        assert_eq!(chat_id, 124);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_new_chat_members() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "new_chat_members": [{
            "id": 1234,
            "first_name": "test",
            "is_bot": false
        }]
    }))
    .unwrap();
    if let MessageData::NewChatMembers(users) = msg.data {
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 1234);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_new_chat_photo() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "new_chat_photo": [{
            "file_id": "photo file id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }]
    }))
    .unwrap();
    if let MessageData::NewChatPhoto(photos) = msg.data {
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].file_id, "photo file id");
        assert_eq!(photos[0].file_unique_id, "unique-id");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_new_chat_title() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "new_chat_title": "new chat title"
    }))
    .unwrap();
    if let MessageData::NewChatTitle(title) = msg.data {
        assert_eq!(title, "new chat title");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_passport_data() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "passport_data": {
            "data": [],
            "credentials": {
                "data": "data",
                "hash": "hash",
                "secret": "secret"
            }
        }
    }))
    .unwrap();
    if let MessageData::PassportData(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert!(data.data.is_empty());
        assert_eq!(data.credentials.data, "data");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_photo_partial() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "photo": [{
            "file_id": "photo-id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }]
    }))
    .unwrap();
    if let MessageData::Photo { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.len(), 1);
        let photo = &data[0];
        assert_eq!(photo.file_id, "photo-id");
        assert_eq!(photo.file_unique_id, "unique-id");
        assert!(caption.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_photo_full() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test photo caption",
        "caption_entities": [
            {"type": "bold", "offset": 0, "length": 4},
        ],
        "photo": [{
            "file_id": "photo-id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }]
    }))
    .unwrap();
    if let MessageData::Photo { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.len(), 1);
        let photo = &data[0];
        assert_eq!(photo.file_id, "photo-id");
        assert_eq!(photo.file_unique_id, "unique-id");
        let caption = caption.unwrap();
        assert_eq!(caption.data, "test photo caption");
        assert_eq!(
            Vec::from(caption.entities.unwrap()),
            vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_pinned_message() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "pinned_message": {
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        }
    }))
    .unwrap();
    if let MessageData::PinnedMessage(pinned_msg) = msg.data {
        assert_eq!(pinned_msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_poll() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "poll": {
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
        }
    }))
    .unwrap();
    if let MessageData::Poll(data) = msg.data {
        assert_eq!(msg.id, 1);
        if let Poll::Regular(data) = data {
            assert_eq!(data.id, "poll-id");
        } else {
            panic!("Unexpected poll kind")
        }
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_proximity_alert_triggered() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname1", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "proximity_alert_triggered": {
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
        }
    }))
    .unwrap();
    if let MessageData::ProximityAlertTriggered(alert) = msg.data {
        assert_eq!(alert.traveler.id, 1);
        assert_eq!(alert.watcher.id, 2);
        assert_eq!(alert.distance, 100);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_sticker() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "sticker": {
            "file_id": "sticker-id",
            "file_unique_id": "unique-id",
            "width": 512,
            "height": 512,
            "is_animated": true,
            "is_video": false
        }
    }))
    .unwrap();
    if let MessageData::Sticker(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "sticker-id");
        assert_eq!(data.file_unique_id, "unique-id");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_successful_payment() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "successful_payment": {
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }
    }))
    .unwrap();
    if let MessageData::SuccessfulPayment(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.currency, "RUB");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_supergroup_chat_created() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "supergroup_chat_created": true
    }))
    .unwrap();
    if let MessageData::SupergroupChatCreated = msg.data {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_text() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "text"
    }))
    .unwrap();
    if let MessageData::Text(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.data, "text");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_venue() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "venue": {
            "location": {
                "latitude": 1.1,
                "longitude": 2.0
            },
            "title": "venue title",
            "address": "venue address"
        }
    }))
    .unwrap();
    if let MessageData::Venue(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.title, "venue title");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_video_partial() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "video": {
            "file_id": "video-id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3
        }
    }))
    .unwrap();
    if let MessageData::Video { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "video-id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert!(caption.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_video_full() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test video caption",
        "caption_entities": [
            {"type": "bold", "offset": 0, "length": 4},
        ],
        "video": {
            "file_id": "video-id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3
        }
    }))
    .unwrap();
    if let MessageData::Video { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "video-id");
        assert_eq!(data.file_unique_id, "unique-id");
        let caption = caption.unwrap();
        assert_eq!(caption.data, "test video caption");
        assert_eq!(
            Vec::from(caption.entities.unwrap()),
            vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_video_note() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "video_note": {
            "file_id": "video-note-id",
            "file_unique_id": "unique-id",
            "length": 124,
            "duration": 1234
        }
    }))
    .unwrap();
    if let MessageData::VideoNote(data) = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "video-note-id");
        assert_eq!(data.file_unique_id, "unique-id");
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_partial() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "voice": {
            "file_id": "voice-id",
            "file_unique_id": "unique-id",
            "duration": 123
        }
    }))
    .unwrap();
    if let MessageData::Voice { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "voice-id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert!(caption.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_full() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test voice caption",
        "caption_entities": [
            {"type": "bold", "offset": 0, "length": 4},
        ],
        "voice": {
            "file_id": "voice-id",
            "file_unique_id": "unique-id",
            "duration": 123
        }
    }))
    .unwrap();
    if let MessageData::Voice { data, caption } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(data.file_id, "voice-id");
        assert_eq!(data.file_unique_id, "unique-id");
        let caption = caption.unwrap();
        assert_eq!(caption.data, "test voice caption");
        assert_eq!(
            Vec::from(caption.entities.unwrap()),
            vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_chat_ended() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "voice_chat_ended": { "duration": 100 }
    }))
    .unwrap();
    if let MessageData::VoiceChatEnded { duration } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(duration, 100);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_chat_participants_invited() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "voice_chat_participants_invited": {
            "users": [
                {"id": 1, "first_name": "firstname", "is_bot": false}
            ]
        }
    }))
    .unwrap();
    if let MessageData::VoiceChatParticipantsInvited { users } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_chat_scheduled() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "voice_chat_scheduled": {"start_date": 100}
    }))
    .unwrap();
    if let MessageData::VoiceChatScheduled { start_date } = msg.data {
        assert_eq!(msg.id, 1);
        assert_eq!(start_date, 100);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_voice_chat_started() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "voice_chat_started": {}
    }))
    .unwrap();
    if let MessageData::VoiceChatStarted = msg.data {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

use crate::types::{
    animation::Animation,
    audio::Audio,
    contact::Contact,
    dice::Dice,
    document::Document,
    game::Game,
    location::{Location, ProximityAlertTriggered},
    message::{raw::RawMessageData, Message, Text},
    passport::PassportData,
    payments::{Invoice, SuccessfulPayment},
    photo_size::PhotoSize,
    poll::Poll,
    primitive::Integer,
    stickers::Sticker,
    user::User,
    venue::Venue,
    video::Video,
    video_note::VideoNote,
    voice::Voice,
    TextEntityError,
};
use serde::Deserialize;
use std::{
    convert::TryFrom,
    error::{Error as StdError, Error},
    fmt,
};

/// Contains message data
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(try_from = "RawMessageData")]
pub enum MessageData {
    /// Message is an animation, information about the animation
    Animation(Animation),
    /// Audio message
    Audio {
        /// Audio caption
        caption: Option<Text>,
        /// Audio data
        data: Audio,
    },
    /// Service message: the channel has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel
    ChannelChatCreated,
    /// The domain name of the website on which the user has logged in
    ConnectedWebsite(String),
    /// Message is a shared contact, information about the contact
    Contact(Contact),
    /// Service message: the chat photo was deleted
    DeleteChatPhoto,
    /// Message is a dice with random value from 1 to 6
    Dice(Dice),
    /// Document message
    Document {
        /// Document caption
        caption: Option<Text>,
        /// Document data
        data: Document,
    },
    /// Message has no data
    Empty,
    /// Message is a game, information about the game
    Game(Game),
    /// Service message: the group has been created
    GroupChatCreated,
    /// Message is an invoice for a payment, information about the invoice
    Invoice(Invoice),
    /// A member was removed from the group
    /// (this member may be the bot itself)
    LeftChatMember(User),
    /// Message is a shared location, information about the location
    Location(Location),
    /// The supergroup has been migrated from a group with the specified identifier
    MigrateFromChatId(Integer),
    /// The group has been migrated to a supergroup with the specified identifier
    MigrateToChatId(Integer),
    /// New members that were added to the group or supergroup
    /// (the bot itself may be one of these members)
    NewChatMembers(Vec<User>),
    /// A chat photo was change to this value
    NewChatPhoto(Vec<PhotoSize>),
    /// A chat title was changed to this value
    NewChatTitle(String),
    /// Telegram Passport data
    PassportData(PassportData),
    /// Specified message was pinned
    /// Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply
    PinnedMessage(Box<Message>),
    /// Message is a photo, available sizes of the photo
    Photo {
        /// Photo caption
        caption: Option<Text>,
        /// Photos
        data: Vec<PhotoSize>,
    },
    /// Message is a native poll, information about the poll
    Poll(Poll),
    /// Service message
    ///
    /// A user in the chat triggered another user's proximity alert while sharing Live Location
    ProximityAlertTriggered(ProximityAlertTriggered),
    /// Message is a sticker, information about the sticker
    Sticker(Sticker),
    /// Message is a service message about a successful payment, information about the payment
    SuccessfulPayment(SuccessfulPayment),
    /// Service message: the supergroup has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message
    /// in a directly created supergroup
    SupergroupChatCreated,
    /// The actual UTF-8 text of the message, 0-4096 characters
    Text(Text),
    /// Message is a venue, information about the venue
    Venue(Venue),
    /// Message is a video, information about the video
    Video {
        /// Video caption
        caption: Option<Text>,
        /// Video data
        data: Video,
    },
    /// Message is a video note, information about the video message
    VideoNote(VideoNote),
    /// Message is a voice message, information about the file
    Voice {
        /// Voice caption
        caption: Option<Text>,
        /// Voice data
        data: Voice,
    },
}

impl TryFrom<RawMessageData> for MessageData {
    type Error = MessageDataError;

    fn try_from(raw: RawMessageData) -> Result<Self, Self::Error> {
        Ok(match raw {
            RawMessageData::Animation { animation } => MessageData::Animation(animation),
            RawMessageData::Audio {
                caption,
                caption_entities,
                audio,
            } => MessageData::Audio {
                caption: Text::from_raw_opt(caption, caption_entities)?,
                data: audio,
            },
            RawMessageData::ChannelChatCreated { .. } => MessageData::ChannelChatCreated,
            RawMessageData::ConnectedWebsite { connected_website } => MessageData::ConnectedWebsite(connected_website),
            RawMessageData::Contact { contact } => MessageData::Contact(contact),
            RawMessageData::DeleteChatPhoto { .. } => MessageData::DeleteChatPhoto,
            RawMessageData::Dice { dice } => MessageData::Dice(dice),
            RawMessageData::Document {
                caption,
                caption_entities,
                document,
            } => MessageData::Document {
                caption: Text::from_raw_opt(caption, caption_entities)?,
                data: document,
            },
            RawMessageData::Empty {} => MessageData::Empty,
            RawMessageData::Game { game } => MessageData::Game(game),
            RawMessageData::GroupChatCreated { .. } => MessageData::GroupChatCreated,
            RawMessageData::Invoice { invoice } => MessageData::Invoice(invoice),
            RawMessageData::LeftChatMember { left_chat_member } => MessageData::LeftChatMember(left_chat_member),
            RawMessageData::Location { location } => MessageData::Location(location),
            RawMessageData::MigrateFromChatId { migrate_from_chat_id } => {
                MessageData::MigrateFromChatId(migrate_from_chat_id)
            }
            RawMessageData::MigrateToChatId { migrate_to_chat_id } => MessageData::MigrateToChatId(migrate_to_chat_id),
            RawMessageData::NewChatMembers { new_chat_members } => MessageData::NewChatMembers(new_chat_members),
            RawMessageData::NewChatPhoto { new_chat_photo } => MessageData::NewChatPhoto(new_chat_photo),
            RawMessageData::NewChatTitle { new_chat_title } => MessageData::NewChatTitle(new_chat_title),
            RawMessageData::PassportData { passport_data } => MessageData::PassportData(passport_data),
            RawMessageData::PinnedMessage { pinned_message } => MessageData::PinnedMessage(pinned_message),
            RawMessageData::Photo {
                caption,
                caption_entities,
                photo,
            } => MessageData::Photo {
                caption: Text::from_raw_opt(caption, caption_entities)?,
                data: photo,
            },
            RawMessageData::Poll { poll } => MessageData::Poll(poll),
            RawMessageData::ProximityAlertTriggered {
                proximity_alert_triggered,
            } => MessageData::ProximityAlertTriggered(proximity_alert_triggered),
            RawMessageData::Sticker { sticker } => MessageData::Sticker(sticker),
            RawMessageData::SuccessfulPayment { successful_payment } => {
                MessageData::SuccessfulPayment(successful_payment)
            }
            RawMessageData::SupergroupChatCreated { .. } => MessageData::SupergroupChatCreated,
            RawMessageData::Text { text, entities } => MessageData::Text(Text::from_raw(text, entities)?),
            RawMessageData::Venue { venue } => MessageData::Venue(venue),
            RawMessageData::Video {
                caption,
                caption_entities,
                video,
            } => MessageData::Video {
                caption: Text::from_raw_opt(caption, caption_entities)?,
                data: video,
            },
            RawMessageData::VideoNote { video_note } => MessageData::VideoNote(video_note),
            RawMessageData::Voice {
                caption,
                caption_entities,
                voice,
            } => MessageData::Voice {
                caption: Text::from_raw_opt(caption, caption_entities)?,
                data: voice,
            },
        })
    }
}

/// A message data error when parsing message data
#[derive(Debug)]
pub enum MessageDataError {
    /// Error when parsing text entities
    TextEntity(TextEntityError),
}

impl StdError for MessageDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MessageDataError::TextEntity(err) => Some(err),
        }
    }
}

impl fmt::Display for MessageDataError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageDataError::TextEntity(err) => err.fmt(out),
        }
    }
}

impl From<TextEntityError> for MessageDataError {
    fn from(err: TextEntityError) -> Self {
        Self::TextEntity(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        dice::DiceKind,
        text::{TextEntity, TextEntityPosition},
    };

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
    fn deserialize_audio() {
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
                caption.entities.unwrap(),
                vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
            );
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
    fn deserialize_document() {
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
                caption.entities.unwrap(),
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
    fn deserialize_photo() {
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
                caption.entities.unwrap(),
                vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
            );
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
                "is_animated": true
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
    fn deserialize_video() {
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
                caption.entities.unwrap(),
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
    fn deserialize_voice() {
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
                caption.entities.unwrap(),
                vec![TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 })]
            );
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }
}

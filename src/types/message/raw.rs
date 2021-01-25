use crate::types::{
    animation::Animation,
    audio::Audio,
    chat::{ChannelChat, Chat},
    contact::Contact,
    dice::Dice,
    document::Document,
    game::Game,
    location::{Location, ProximityAlertTriggered},
    message::{Forward, Message, MessageData},
    passport::PassportData,
    payments::{Invoice, SuccessfulPayment},
    photo_size::PhotoSize,
    poll::Poll,
    primitive::{Integer, True},
    reply_markup::InlineKeyboardMarkup,
    stickers::Sticker,
    text::RawTextEntity,
    user::User,
    venue::Venue,
    video::Video,
    video_note::VideoNote,
    voice::Voice,
};
use serde::Deserialize;
use vec1::Vec1;

#[derive(Debug, Deserialize)]
pub(super) struct RawMessage {
    pub message_id: Integer,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: Integer,
    pub chat: Chat,
    #[serde(flatten)]
    pub forward: Option<Forward>,
    pub reply_to_message: Option<Message>,
    pub via_bot: Option<User>,
    pub edit_date: Option<Integer>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    #[serde(flatten)]
    pub data: MessageData,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct RawForward {
    pub forward_date: Integer,
    #[serde(flatten)]
    pub forward_from: RawForwardFrom,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum RawForwardFrom {
    User {
        forward_from: User,
    },
    HiddenUser {
        forward_sender_name: String,
    },
    Channel {
        forward_from_chat: ChannelChat,
        forward_from_message_id: Integer,
        forward_signature: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub(super) enum RawMessageData {
    Animation {
        animation: Animation,
    },
    Audio {
        caption: Option<String>,
        caption_entities: Option<Vec1<RawTextEntity>>,
        audio: Audio,
    },
    ChannelChatCreated {
        channel_chat_created: True,
    },
    ConnectedWebsite {
        connected_website: String,
    },
    Contact {
        contact: Contact,
    },
    DeleteChatPhoto {
        delete_chat_photo: True,
    },
    Dice {
        dice: Dice,
    },
    Document {
        caption: Option<String>,
        caption_entities: Option<Vec1<RawTextEntity>>,
        document: Document,
    },
    Game {
        game: Game,
    },
    GroupChatCreated {
        group_chat_created: True,
    },
    Invoice {
        invoice: Invoice,
    },
    LeftChatMember {
        left_chat_member: User,
    },
    Location {
        location: Location,
    },
    MigrateFromChatId {
        migrate_from_chat_id: Integer,
    },
    MigrateToChatId {
        migrate_to_chat_id: Integer,
    },
    NewChatMembers {
        new_chat_members: Vec<User>,
    },
    NewChatPhoto {
        new_chat_photo: Vec<PhotoSize>,
    },
    NewChatTitle {
        new_chat_title: String,
    },
    PassportData {
        passport_data: PassportData,
    },
    PinnedMessage {
        pinned_message: Box<Message>,
    },
    Photo {
        caption: Option<String>,
        caption_entities: Option<Vec1<RawTextEntity>>,
        photo: Vec<PhotoSize>,
    },
    Poll {
        poll: Poll,
    },
    ProximityAlertTriggered {
        proximity_alert_triggered: ProximityAlertTriggered,
    },
    Sticker {
        sticker: Sticker,
    },
    SuccessfulPayment {
        successful_payment: SuccessfulPayment,
    },
    SupergroupChatCreated {
        supergroup_chat_created: True,
    },
    Text {
        text: String,
        entities: Option<Vec1<RawTextEntity>>,
    },
    Venue {
        venue: Venue,
    },
    Video {
        caption: Option<String>,
        caption_entities: Option<Vec1<RawTextEntity>>,
        video: Video,
    },
    VideoNote {
        video_note: VideoNote,
    },
    Voice {
        caption: Option<String>,
        caption_entities: Option<Vec1<RawTextEntity>>,
        voice: Voice,
    },
    Empty {}, // must be last because all variants below won't be deserialized
}

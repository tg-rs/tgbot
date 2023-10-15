use std::{convert::TryFrom, error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{
    Animation,
    Audio,
    Contact,
    Dice,
    Document,
    Game,
    Integer,
    Invoice,
    Location,
    Message,
    PassportData,
    PhotoSize,
    Poll,
    ProximityAlertTriggered,
    Sticker,
    SuccessfulPayment,
    Text,
    TextEntities,
    TextEntityError,
    True,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
};

#[cfg(test)]
mod tests;

/// Contains message data
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(into = "RawMessageData")]
#[serde(try_from = "RawMessageData")]
pub enum MessageData {
    /// Message is an animation, information about the animation
    Animation(Animation),
    /// Audio message
    Audio {
        /// Audio caption
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        /// Audio data
        data: Audio,
    },
    /// A service message about a change in auto-delete timer settings
    AutoDeleteTimerChanged {
        /// New auto-delete time for messages in the chat
        time: Integer,
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
        #[serde(skip_serializing_if = "Option::is_none")]
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
        #[serde(skip_serializing_if = "Option::is_none")]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        /// Video data
        data: Video,
    },
    /// Message is a video note, information about the video message
    VideoNote(VideoNote),
    /// Message is a voice message, information about the file
    Voice {
        /// Voice caption
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        /// Voice data
        data: Voice,
    },
    /// A service message about a voice chat scheduled in the chat
    VoiceChatScheduled {
        /// Point in time (Unix timestamp) when the voice chat
        /// is supposed to be started by a chat administrator
        start_date: Integer,
    },
    /// A service message about a voice chat started in the chat
    VoiceChatStarted,
    /// A service message about a voice chat ended in the chat
    VoiceChatEnded {
        /// Voice chat duration; in seconds
        duration: Integer,
    },
    /// A service message about new members invited to a voice chat
    VoiceChatParticipantsInvited {
        /// New members that were invited to the voice chat
        users: Vec<User>,
    },
    /// Service message: data sent by a Web App
    WebAppData(WebAppData),
}

impl TryFrom<RawMessageData> for MessageData {
    type Error = MessageDataError;

    fn try_from(raw: RawMessageData) -> Result<Self, Self::Error> {
        Ok(match raw {
            RawMessageData::Animation { animation } => MessageData::Animation(animation),
            RawMessageData::Audio { caption, audio } => MessageData::Audio { caption, data: audio },
            RawMessageData::ChannelChatCreated { .. } => MessageData::ChannelChatCreated,
            RawMessageData::ConnectedWebsite { connected_website } => MessageData::ConnectedWebsite(connected_website),
            RawMessageData::Contact { contact } => MessageData::Contact(contact),
            RawMessageData::DeleteChatPhoto { .. } => MessageData::DeleteChatPhoto,
            RawMessageData::Dice { dice } => MessageData::Dice(dice),
            RawMessageData::Document {
                caption,
                document: data,
            } => MessageData::Document { caption, data },
            RawMessageData::Empty {} => MessageData::Empty,
            RawMessageData::Game { game } => MessageData::Game(game),
            RawMessageData::GroupChatCreated { .. } => MessageData::GroupChatCreated,
            RawMessageData::Invoice { invoice } => MessageData::Invoice(invoice),
            RawMessageData::LeftChatMember { left_chat_member } => MessageData::LeftChatMember(left_chat_member),
            RawMessageData::Location { location } => MessageData::Location(location),
            RawMessageData::MessageAutoDeleteTimerChanged {
                message_auto_delete_timer_changed,
            } => MessageData::AutoDeleteTimerChanged {
                time: message_auto_delete_timer_changed.message_auto_delete_time,
            },
            RawMessageData::MigrateFromChatId { migrate_from_chat_id } => {
                MessageData::MigrateFromChatId(migrate_from_chat_id)
            }
            RawMessageData::MigrateToChatId { migrate_to_chat_id } => MessageData::MigrateToChatId(migrate_to_chat_id),
            RawMessageData::NewChatMembers { new_chat_members } => MessageData::NewChatMembers(new_chat_members),
            RawMessageData::NewChatPhoto { new_chat_photo } => MessageData::NewChatPhoto(new_chat_photo),
            RawMessageData::NewChatTitle { new_chat_title } => MessageData::NewChatTitle(new_chat_title),
            RawMessageData::PassportData { passport_data } => MessageData::PassportData(passport_data),
            RawMessageData::PinnedMessage { pinned_message } => MessageData::PinnedMessage(pinned_message),
            RawMessageData::Photo { caption, photo: data } => MessageData::Photo { caption, data },
            RawMessageData::Poll { poll } => MessageData::Poll(poll),
            RawMessageData::ProximityAlertTriggered {
                proximity_alert_triggered,
            } => MessageData::ProximityAlertTriggered(proximity_alert_triggered),
            RawMessageData::Sticker { sticker } => MessageData::Sticker(sticker),
            RawMessageData::SuccessfulPayment { successful_payment } => {
                MessageData::SuccessfulPayment(successful_payment)
            }
            RawMessageData::SupergroupChatCreated { .. } => MessageData::SupergroupChatCreated,
            RawMessageData::Text { text: data, entities } => MessageData::Text(Text { data, entities }),
            RawMessageData::Venue { venue } => MessageData::Venue(venue),
            RawMessageData::Video { caption, video: data } => MessageData::Video { caption, data },
            RawMessageData::VideoNote { video_note } => MessageData::VideoNote(video_note),
            RawMessageData::Voice { caption, voice: data } => MessageData::Voice { caption, data },
            RawMessageData::VoiceChatScheduled { voice_chat_scheduled } => MessageData::VoiceChatScheduled {
                start_date: voice_chat_scheduled.start_date,
            },
            RawMessageData::VoiceChatStarted { .. } => MessageData::VoiceChatStarted,
            RawMessageData::VoiceChatEnded { voice_chat_ended } => MessageData::VoiceChatEnded {
                duration: voice_chat_ended.duration,
            },
            RawMessageData::VoiceChatParticipantsInvited {
                voice_chat_participants_invited,
            } => MessageData::VoiceChatParticipantsInvited {
                users: voice_chat_participants_invited.users.unwrap_or_default(),
            },
            RawMessageData::WebAppData { web_app_data } => MessageData::WebAppData(web_app_data),
        })
    }
}

impl From<MessageData> for RawMessageData {
    fn from(value: MessageData) -> Self {
        match value {
            MessageData::Animation(animation) => Self::Animation { animation },
            MessageData::Audio { data: audio, caption } => Self::Audio { caption, audio },
            MessageData::AutoDeleteTimerChanged { time } => Self::MessageAutoDeleteTimerChanged {
                message_auto_delete_timer_changed: RawMessageAutoDeleteTimerChanged {
                    message_auto_delete_time: time,
                },
            },
            MessageData::ChannelChatCreated => Self::ChannelChatCreated {
                channel_chat_created: True,
            },
            MessageData::ConnectedWebsite(connected_website) => Self::ConnectedWebsite { connected_website },
            MessageData::Contact(contact) => Self::Contact { contact },
            MessageData::DeleteChatPhoto => Self::DeleteChatPhoto {
                delete_chat_photo: True,
            },
            MessageData::Dice(dice) => Self::Dice { dice },
            MessageData::Document {
                data: document,
                caption,
            } => Self::Document { caption, document },
            MessageData::Empty => Self::Empty {},
            MessageData::Game(game) => Self::Game { game },
            MessageData::GroupChatCreated => Self::GroupChatCreated {
                group_chat_created: True,
            },
            MessageData::Invoice(invoice) => Self::Invoice { invoice },
            MessageData::LeftChatMember(left_chat_member) => Self::LeftChatMember { left_chat_member },
            MessageData::Location(location) => Self::Location { location },
            MessageData::MigrateFromChatId(migrate_from_chat_id) => Self::MigrateFromChatId { migrate_from_chat_id },
            MessageData::MigrateToChatId(migrate_to_chat_id) => Self::MigrateToChatId { migrate_to_chat_id },
            MessageData::NewChatMembers(new_chat_members) => Self::NewChatMembers { new_chat_members },
            MessageData::NewChatPhoto(new_chat_photo) => Self::NewChatPhoto { new_chat_photo },
            MessageData::NewChatTitle(new_chat_title) => Self::NewChatTitle { new_chat_title },
            MessageData::PassportData(passport_data) => Self::PassportData { passport_data },
            MessageData::PinnedMessage(pinned_message) => Self::PinnedMessage { pinned_message },
            MessageData::Photo { caption, data: photo } => Self::Photo { caption, photo },
            MessageData::Poll(poll) => Self::Poll { poll },
            MessageData::ProximityAlertTriggered(proximity_alert_triggered) => Self::ProximityAlertTriggered {
                proximity_alert_triggered,
            },
            MessageData::Sticker(sticker) => Self::Sticker { sticker },
            MessageData::SuccessfulPayment(successful_payment) => Self::SuccessfulPayment { successful_payment },
            MessageData::SupergroupChatCreated => Self::SupergroupChatCreated {
                supergroup_chat_created: True,
            },
            MessageData::Text(text) => Self::Text {
                text: text.data,
                entities: text.entities,
            },
            MessageData::Venue(venue) => Self::Venue { venue },
            MessageData::Video { caption, data: video } => Self::Video { caption, video },
            MessageData::VideoNote(video_note) => Self::VideoNote { video_note },
            MessageData::Voice { caption, data: voice } => Self::Voice { caption, voice },
            MessageData::VoiceChatScheduled { start_date } => Self::VoiceChatScheduled {
                voice_chat_scheduled: RawVoiceChatScheduled { start_date },
            },
            MessageData::VoiceChatStarted => Self::VoiceChatStarted {
                voice_chat_started: RawVoiceChatStarted {},
            },
            MessageData::VoiceChatEnded { duration } => Self::VoiceChatEnded {
                voice_chat_ended: RawVoiceChatEnded { duration },
            },
            MessageData::VoiceChatParticipantsInvited { users } => Self::VoiceChatParticipantsInvited {
                voice_chat_participants_invited: RawVoiceChatParticipantsInvited {
                    users: if users.is_empty() { None } else { Some(users) },
                },
            },
            MessageData::WebAppData(web_app_data) => Self::WebAppData { web_app_data },
        }
    }
}

#[derive(Deserialize, Serialize)]
struct MessageCaption {
    caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
}

impl MessageCaption {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<MessageCaption>::deserialize(deserializer).map(|wrapper| {
            wrapper.map(
                |MessageCaption {
                     caption: data,
                     caption_entities: entities,
                 }| Text { data, entities },
            )
        })
    }

    fn serialize_value<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.clone().map(|value| MessageCaption {
            caption: value.data,
            caption_entities: value.entities,
        });
        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
enum RawMessageData {
    Animation {
        animation: Animation,
    },
    Audio {
        #[serde(flatten)]
        #[serde(deserialize_with = "MessageCaption::deserialize_value")]
        #[serde(serialize_with = "MessageCaption::serialize_value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        audio: Audio,
    },
    ChannelChatCreated {
        #[allow(dead_code)]
        channel_chat_created: True,
    },
    ConnectedWebsite {
        connected_website: String,
    },
    Contact {
        contact: Contact,
    },
    DeleteChatPhoto {
        #[allow(dead_code)]
        delete_chat_photo: True,
    },
    Dice {
        dice: Dice,
    },
    Document {
        #[serde(flatten)]
        #[serde(deserialize_with = "MessageCaption::deserialize_value")]
        #[serde(serialize_with = "MessageCaption::serialize_value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        document: Document,
    },
    Game {
        game: Game,
    },
    GroupChatCreated {
        #[allow(dead_code)]
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
    MessageAutoDeleteTimerChanged {
        message_auto_delete_timer_changed: RawMessageAutoDeleteTimerChanged,
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
        #[serde(flatten)]
        #[serde(deserialize_with = "MessageCaption::deserialize_value")]
        #[serde(serialize_with = "MessageCaption::serialize_value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
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
        #[allow(dead_code)]
        supergroup_chat_created: True,
    },
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        entities: Option<TextEntities>,
    },
    Venue {
        venue: Venue,
    },
    Video {
        #[serde(flatten)]
        #[serde(deserialize_with = "MessageCaption::deserialize_value")]
        #[serde(serialize_with = "MessageCaption::serialize_value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        video: Video,
    },
    VideoNote {
        video_note: VideoNote,
    },
    Voice {
        #[serde(flatten)]
        #[serde(deserialize_with = "MessageCaption::deserialize_value")]
        #[serde(serialize_with = "MessageCaption::serialize_value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Text>,
        voice: Voice,
    },
    VoiceChatScheduled {
        voice_chat_scheduled: RawVoiceChatScheduled,
    },
    VoiceChatEnded {
        voice_chat_ended: RawVoiceChatEnded,
    },
    VoiceChatParticipantsInvited {
        voice_chat_participants_invited: RawVoiceChatParticipantsInvited,
    },
    VoiceChatStarted {
        #[allow(dead_code)]
        voice_chat_started: RawVoiceChatStarted,
    },
    WebAppData {
        web_app_data: WebAppData,
    },
    Empty {}, // must be last because all variants below won't be deserialized
}

#[derive(Debug, Deserialize, Serialize)]
struct RawMessageAutoDeleteTimerChanged {
    message_auto_delete_time: Integer,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawVoiceChatScheduled {
    start_date: Integer,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawVoiceChatEnded {
    duration: Integer,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawVoiceChatParticipantsInvited {
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<User>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawVoiceChatStarted {}

/// A message data error when parsing message data
#[derive(Debug)]
pub enum MessageDataError {
    /// Error when parsing text entities
    TextEntity(TextEntityError),
}

impl Error for MessageDataError {
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

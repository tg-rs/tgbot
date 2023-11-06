use std::{error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;

use crate::types::{
    Animation,
    Audio,
    ChatShared,
    Contact,
    Dice,
    Document,
    ForumTopicClosed,
    ForumTopicCreated,
    ForumTopicEdited,
    ForumTopicReopened,
    Game,
    GeneralForumTopicHidden,
    GeneralForumTopicUnhidden,
    Integer,
    Invoice,
    Location,
    Message,
    PassportData,
    PhotoSize,
    Poll,
    ProximityAlertTriggered,
    Sticker,
    Story,
    SuccessfulPayment,
    Text,
    TextEntities,
    TextEntityError,
    True,
    User,
    UserShared,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
    WriteAccessAllowed,
};

#[cfg(test)]
mod tests;

/// Contains message data
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum MessageData {
    /// Message is an animation, information about the animation
    Animation(Animation),
    /// A service message about a change in auto-delete timer settings
    #[serde(rename = "message_auto_delete_timer_changed")]
    AutoDeleteTimerChanged {
        /// New auto-delete time for messages in the chat
        #[serde(rename = "message_auto_delete_time")]
        time: Integer,
    },
    /// Service message: the channel has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    ChannelChatCreated,
    /// Service message: a chat was shared with the bot
    ChatShared(ChatShared),
    /// The domain name of the website on which the user has logged in
    ConnectedWebsite(String),
    /// Message is a shared contact, information about the contact
    Contact(Contact),
    /// Service message: the chat photo was deleted
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    DeleteChatPhoto,
    /// Message is a dice with random value from 1 to 6
    Dice(Dice),
    /// Service message: forum topic closed
    ForumTopicClosed(ForumTopicClosed),
    /// Service message: forum topic created
    ForumTopicCreated(ForumTopicCreated),
    /// Service message: forum topic edited
    ForumTopicEdited(ForumTopicEdited),
    /// Service message: forum topic reopened
    ForumTopicReopened(ForumTopicReopened),
    /// Message is a game, information about the game
    Game(Game),
    /// Service message: the 'General' forum topic hidden
    GeneralForumTopicHidden(GeneralForumTopicHidden),
    /// Service message: the 'General' forum topic unhidden
    GeneralForumTopicUnhidden(GeneralForumTopicUnhidden),
    /// Service message: the group has been created
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
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
    /// Message is a native poll, information about the poll
    Poll(Poll),
    /// Service message
    ///
    /// A user in the chat triggered another user's proximity alert while sharing Live Location
    ProximityAlertTriggered(ProximityAlertTriggered),
    /// Message is a sticker, information about the sticker
    Sticker(Sticker),
    /// Message is a forwarded story
    Story(Story),
    /// Message is a service message about a successful payment, information about the payment
    SuccessfulPayment(SuccessfulPayment),
    /// Service message: the supergroup has been created
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created
    /// It can only be found in reply_to_message if someone replies to a very first message
    /// in a directly created supergroup
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    SupergroupChatCreated,
    /// Service message: a user was shared with the bot
    UserShared(UserShared),
    /// Message is a venue, information about the venue
    Venue(Venue),
    /// Message is a video note, information about the video message
    VideoNote(VideoNote),
    /// A service message about a voice chat ended in the chat
    VideoChatEnded {
        /// Voice chat duration; in seconds
        duration: Integer,
    },
    /// A service message about new members invited to a voice chat
    VideoChatParticipantsInvited {
        /// New members that were invited to the voice chat
        #[serde(skip_serializing_if = "Option::is_none")]
        users: Option<Vec<User>>,
    },
    /// A service message about a voice chat scheduled in the chat
    VideoChatScheduled {
        /// Point in time (Unix timestamp) when the voice chat
        /// is supposed to be started by a chat administrator
        start_date: Integer,
    },
    /// A service message about a voice chat started in the chat
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    VideoChatStarted,
    /// Service message: data sent by a Web App
    WebAppData(WebAppData),
    /// Service message: the user allowed the bot to write messages
    /// after adding it to the attachment or side menu,
    /// launching a Web App from a link,
    /// or accepting an explicit request from a Web App
    /// sent by the method requestWriteAccess
    WriteAccessAllowed(WriteAccessAllowed),
    /// Audio message
    #[serde(untagged)]
    Audio {
        /// Audio caption
        #[serde(
            flatten,
            deserialize_with = "MessageCaption::deserialize_value",
            serialize_with = "MessageCaption::serialize_value",
            skip_serializing_if = "Option::is_none"
        )]
        caption: Option<Text>,
        /// Audio data
        #[serde(rename = "audio")]
        data: Audio,
    },
    /// Document message
    #[serde(untagged)]
    Document {
        /// Document caption
        #[serde(
            flatten,
            deserialize_with = "MessageCaption::deserialize_value",
            serialize_with = "MessageCaption::serialize_value",
            skip_serializing_if = "Option::is_none"
        )]
        caption: Option<Text>,
        /// Document data
        #[serde(rename = "document")]
        data: Document,
    },
    /// Message is a photo, available sizes of the photo
    #[serde(untagged)]
    Photo {
        /// Photo caption
        #[serde(
            flatten,
            deserialize_with = "MessageCaption::deserialize_value",
            serialize_with = "MessageCaption::serialize_value",
            skip_serializing_if = "Option::is_none"
        )]
        caption: Option<Text>,
        /// Photos
        #[serde(rename = "photo")]
        data: Vec<PhotoSize>,
    },
    /// The actual UTF-8 text of the message, 0-4096 characters
    #[serde(
        deserialize_with = "RawDataText::deserialize_value",
        serialize_with = "RawDataText::serialize_value",
        untagged
    )]
    Text(Text),
    /// Message is a video, information about the video
    #[serde(untagged)]
    Video {
        /// Video caption
        #[serde(
            flatten,
            deserialize_with = "MessageCaption::deserialize_value",
            serialize_with = "MessageCaption::serialize_value",
            skip_serializing_if = "Option::is_none"
        )]
        caption: Option<Text>,
        /// Video data
        #[serde(rename = "video")]
        data: Video,
    },
    /// Message is a voice message, information about the file
    #[serde(untagged)]
    Voice {
        /// Voice caption
        #[serde(
            flatten,
            deserialize_with = "MessageCaption::deserialize_value",
            serialize_with = "MessageCaption::serialize_value",
            skip_serializing_if = "Option::is_none"
        )]
        caption: Option<Text>,
        /// Voice data
        #[serde(rename = "voice")]
        data: Voice,
    },
    /// Message has no data
    #[serde(untagged)]
    Unknown(JsonValue),
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
struct RawDataEmpty {}

impl RawDataEmpty {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataEmpty::deserialize(deserializer).map(|_| ())
    }

    fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataEmpty {}.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct RawDataFlag;

impl RawDataFlag {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        True::deserialize(deserializer).map(|_| ())
    }

    fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        True.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct RawDataText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
}

impl RawDataText {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Text, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataText::deserialize(deserializer).map(|x| Text {
            data: x.text,
            entities: x.entities,
        })
    }

    fn serialize_value<S>(value: &Text, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataText {
            text: value.data.clone(),
            entities: value.entities.clone(),
        }
        .serialize(serializer)
    }
}

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

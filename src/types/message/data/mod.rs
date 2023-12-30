use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::types::{
    Animation,
    Audio,
    Contact,
    Dice,
    Document,
    ForumTopicIconColor,
    Game,
    Giveaway,
    GiveawayCompleted,
    GiveawayWinners,
    Integer,
    Invoice,
    Location,
    MaybeInaccessibleMessage,
    PassportData,
    PhotoSize,
    Poll,
    Sticker,
    SuccessfulPayment,
    Text,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
};

use self::raw::*;

#[cfg(test)]
mod tests;

mod raw;

/// Represents a message data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum MessageData {
    /// Information about the animation.
    Animation(Animation),
    /// Auto-delete timer settings changed.
    #[serde(rename = "message_auto_delete_timer_changed")]
    AutoDeleteTimerChanged(MessageDataAutoDeleteTimer),
    /// The channel has been created.
    ///
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created.
    /// It can only be found in the `reply_to` field of the [`Message`] struct
    /// if someone replies to a very first message in a channel.
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    ChannelChatCreated,
    /// A chat was shared with the bot.
    ChatShared(MessageDataChatShared),
    /// The domain name of the website on which the user has logged in.
    ConnectedWebsite(String),
    /// Information about the shared contact.
    Contact(Contact),
    /// The chat photo was deleted.
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    DeleteChatPhoto,
    /// A dice with a random value.
    Dice(Dice),
    /// Forum topic closed.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    ForumTopicClosed,
    /// Forum topic created.
    ForumTopicCreated(MessageDataForumTopicCreated),
    /// Forum topic edited.
    ForumTopicEdited(MessageDataForumTopicEdited),
    /// Forum topic reopened.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    ForumTopicReopened,
    /// Information about the game.
    Game(Game),
    /// The 'General' forum topic hidden.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    GeneralForumTopicHidden,
    /// The 'General' forum topic unhidden.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    GeneralForumTopicUnhidden,
    /// A scheduled giveaway.
    Giveaway(Giveaway),
    /// Service message: a scheduled giveaway was created.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    GiveawayCreated,
    /// Service message: a giveaway without public winners was completed.
    GiveawayCompleted(GiveawayCompleted),
    /// A giveaway with public winners was completed.
    GiveawayWinners(GiveawayWinners),
    /// The group has been created.
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    GroupChatCreated,
    /// Information about the invoice for a payment.
    Invoice(Invoice),
    /// A member was removed from the group.
    ///
    /// This member may be the bot itself.
    LeftChatMember(User),
    /// Information about the shared location.
    Location(Location),
    /// The supergroup has been migrated from a group with the specified identifier.
    MigrateFromChatId(Integer),
    /// The group has been migrated to a supergroup with the specified identifier.
    MigrateToChatId(Integer),
    /// New members that were added to the group or supergroup.
    ///
    /// The bot itself may be one of these members.
    NewChatMembers(Vec<User>),
    /// A chat photo was change to this value.
    NewChatPhoto(Vec<PhotoSize>),
    /// A chat title was changed to this value.
    NewChatTitle(String),
    /// Telegram Passport data.
    PassportData(PassportData),
    /// Specified message was pinned.
    ///
    /// Note that the Message object in variant will not contain
    /// further `reply_to` field even if it is itself a reply.
    PinnedMessage(MaybeInaccessibleMessage),
    /// Information about the native poll.
    Poll(Poll),
    /// A user in the chat triggered another user's proximity alert while sharing Live Location.
    ProximityAlertTriggered(MessageDataProximityAlert),
    /// Information about the sticker.
    Sticker(Sticker),
    /// A forwarded story.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    Story,
    /// Information about the successful payment.
    SuccessfulPayment(SuccessfulPayment),
    /// The supergroup has been created.
    ///
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created
    /// It can only be found in the `reply_to` field of the [`Message`] struct
    /// if someone replies to a very first message
    /// in a directly created supergroup.
    #[serde(
        deserialize_with = "RawDataFlag::deserialize_value",
        serialize_with = "RawDataFlag::serialize_value"
    )]
    SupergroupChatCreated,
    /// A user was shared with the bot.
    UsersShared(MessageDataUsersShared),
    /// Information about the venue.
    Venue(Venue),
    /// Information about the video note.
    VideoNote(VideoNote),
    /// A video chat ended in the chat.
    VideoChatEnded(MessageDataVideoChatEnded),
    /// New members invited to a video chat.
    VideoChatParticipantsInvited(MessageDataVideoChatParticipantsInvited),
    /// A video chat scheduled in the chat.
    VideoChatScheduled(MessageDataVideoChatScheduled),
    /// A video chat started in the chat.
    #[serde(
        deserialize_with = "RawDataEmpty::deserialize_value",
        serialize_with = "RawDataEmpty::serialize_value"
    )]
    VideoChatStarted,
    /// Data sent by a Web App.
    WebAppData(WebAppData),
    /// The user allowed the bot to write messages
    /// after adding it to the attachment or side menu,
    /// launching a Web App from a link,
    /// or accepting an explicit request from a Web App
    /// sent by the method `requestWriteAccess`.
    WriteAccessAllowed(MessageDataWriteAccess),
    /// Describes the audio.
    #[serde(untagged)]
    Audio(MessageDataAudio),
    /// Describes the document.
    #[serde(untagged)]
    Document(MessageDataDocument),
    /// Available sizes of the photo.
    #[serde(untagged)]
    Photo(MessageDataPhoto),
    /// The actual UTF-8 text of the message; 0-4096 characters.
    #[serde(
        deserialize_with = "RawDataText::deserialize_value",
        serialize_with = "RawDataText::serialize_value",
        untagged
    )]
    Text(Text),
    /// Describes the video.
    #[serde(untagged)]
    Video(MessageDataVideo),
    /// Describes the voice.
    #[serde(untagged)]
    Voice(MessageDataVoice),
    /// Contains arbitrary data for future variants.
    #[serde(untagged)]
    Unknown(JsonValue),
}

/// Represents a service message about a change in auto-delete timer settings.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataAutoDeleteTimer {
    /// New auto-delete time for messages in the chat; in seconds.
    #[serde(rename = "message_auto_delete_time")]
    pub time: Integer,
}

impl MessageDataAutoDeleteTimer {
    /// Creates a new `MessageDataAutoDeleteTimer`.
    ///
    /// # Arguments
    ///
    /// * `time` - Time in seconds.
    pub fn new(time: Integer) -> Self {
        Self { time }
    }
}

/// Represents an audio message data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataAudio {
    /// Audio data.
    #[serde(rename = "audio")]
    pub data: Audio,
    /// Audio caption.
    #[serde(
        flatten,
        deserialize_with = "RawCaption::deserialize_value",
        serialize_with = "RawCaption::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<Text>,
}

impl From<Audio> for MessageDataAudio {
    fn from(value: Audio) -> Self {
        Self {
            data: value,
            caption: None,
        }
    }
}

impl MessageDataAudio {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.caption = Some(value.into());
        self
    }
}

/// Represents an information about the chat
/// whose identifier was shared with the bot
/// using a [`crate::types::KeyboardButtonRequestChat`] button.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataChatShared {
    /// Identifier of the shared chat.
    ///
    /// The bot may not have access to the chat and could be unable to use this identifier,
    /// unless the chat is already known to the bot by some other means.
    pub chat_id: Integer,
    /// Identifier of the request.
    pub request_id: Integer,
}

impl MessageDataChatShared {
    /// Creates a new `MessageDataChatShared`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Identifier of the shared chat.
    /// * `request_id` - Identifier of the request.
    pub fn new(chat_id: Integer, request_id: Integer) -> Self {
        Self { chat_id, request_id }
    }
}

/// Represents an document message data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataDocument {
    /// Document data.
    #[serde(rename = "document")]
    pub data: Document,
    /// Document caption.
    #[serde(
        flatten,
        deserialize_with = "RawCaption::deserialize_value",
        serialize_with = "RawCaption::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<Text>,
}

impl From<Document> for MessageDataDocument {
    fn from(value: Document) -> Self {
        Self {
            data: value,
            caption: None,
        }
    }
}

impl MessageDataDocument {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.caption = Some(value.into());
        self
    }
}

/// Represents a service message about a new forum topic created in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataForumTopicCreated {
    /// Color of the icon of the topic.
    pub icon_color: ForumTopicIconColor,
    /// Name of the topic.
    pub name: String,
    /// Unique identifier of the custom emoji shown as the topic icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl MessageDataForumTopicCreated {
    /// Creates a new `MessageDataForumTopicCreated`.
    ///
    /// # Arguments
    ///
    /// * `icon_color` - Color of the icon.
    /// * `name` - Name of the topic.
    pub fn new<A, B>(icon_color: A, name: B) -> Self
    where
        A: Into<ForumTopicIconColor>,
        B: Into<String>,
    {
        Self {
            icon_color: icon_color.into(),
            name: name.into(),
            icon_custom_emoji_id: None,
        }
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji ID.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }
}

/// Represents a service message about an edited forum topic.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataForumTopicEdited {
    /// New name, if it was edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon,
    /// if it was edited; an empty string if the icon was removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl MessageDataForumTopicEdited {
    /// Sets a new name.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the topic.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji ID.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }
}

/// Represents a list of available sizes of the photo.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataPhoto {
    /// Photo sizes.
    #[serde(rename = "photo")]
    pub data: Vec<PhotoSize>,
    /// Photo caption.
    #[serde(
        flatten,
        deserialize_with = "RawCaption::deserialize_value",
        serialize_with = "RawCaption::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<Text>,
}

impl<T> From<T> for MessageDataPhoto
where
    T: IntoIterator<Item = PhotoSize>,
{
    fn from(value: T) -> Self {
        Self {
            data: value.into_iter().collect(),
            caption: None,
        }
    }
}

impl MessageDataPhoto {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.caption = Some(value.into());
        self
    }
}

/// Represents a content of a service message,
/// sent whenever a user in the chat triggers
/// a proximity alert set by another user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataProximityAlert {
    /// The distance between the users.
    pub distance: Integer,
    /// User that triggered the alert.
    pub traveler: User,
    /// User that set the alert.
    pub watcher: User,
}

impl MessageDataProximityAlert {
    /// Creates a new `MessageDataProximityAlert`.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance between users.
    /// * `traveler` - User that triggered the alert.
    /// * `watcher` - User that set the alert.
    pub fn new(distance: Integer, traveler: User, watcher: User) -> Self {
        Self {
            distance,
            traveler,
            watcher,
        }
    }
}

/// Contains information about the users
/// whose identifiers were shared with the bot
/// using a [`crate::types::KeyboardButton::with_request_users`] button.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataUsersShared {
    /// Identifier of the request.
    pub request_id: Integer,
    /// Identifiers of the shared users.
    ///
    /// The bot may not have access to the users and could be unable to use these identifiers,
    /// unless the users are already known to the bot by some other means.
    pub user_ids: Vec<Integer>,
}

impl MessageDataUsersShared {
    /// Creates a new `MessageDataUsersShared`.
    ///
    /// # Arguments
    ///
    /// * `request_id` - Identifier of the request.
    /// * `user_ids` - Identifiers of the shared users.
    pub fn new<T>(request_id: Integer, user_ids: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        Self {
            request_id,
            user_ids: user_ids.into_iter().collect(),
        }
    }
}

/// Represents a video message data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideo {
    /// Video data.
    #[serde(rename = "video")]
    pub data: Video,
    /// Video caption.
    #[serde(
        flatten,
        deserialize_with = "RawCaption::deserialize_value",
        serialize_with = "RawCaption::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<Text>,
}

impl From<Video> for MessageDataVideo {
    fn from(value: Video) -> Self {
        Self {
            data: value,
            caption: None,
        }
    }
}

impl MessageDataVideo {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.caption = Some(value.into());
        self
    }
}

/// Represents a service message about a video chat ended in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatEnded {
    /// Video chat duration; in seconds.
    pub duration: Integer,
}

impl MessageDataVideoChatEnded {
    /// Creates a new `MessageDataVideoChatEnded`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Video chat duration; in seconds.
    pub fn new(duration: Integer) -> Self {
        Self { duration }
    }
}

/// A service message about new members invited to a video chat.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatParticipantsInvited {
    /// New members that were invited to the voice chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

impl<T> From<T> for MessageDataVideoChatParticipantsInvited
where
    T: IntoIterator<Item = User>,
{
    fn from(value: T) -> Self {
        Self::default().with_users(value)
    }
}

impl MessageDataVideoChatParticipantsInvited {
    /// Sets a new list of users.
    ///
    /// # Arguments
    ///
    /// * `value` - New members that were invited to the voice chat.
    pub fn with_users<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = User>,
    {
        self.users = Some(value.into_iter().collect());
        self
    }
}

/// A service message about a video chat scheduled in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat
    /// is supposed to be started by a chat administrator.
    pub start_date: Integer,
}

impl MessageDataVideoChatScheduled {
    /// Creates a new `MessageDataVideoChatScheduled`
    ///
    /// # Arguments
    ///
    /// * `start_date` - Point in time (Unix timestamp).
    pub fn new(start_date: Integer) -> Self {
        Self { start_date }
    }
}

/// Message is a voice message, information about the file.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVoice {
    /// Voice data.
    #[serde(rename = "voice")]
    pub data: Voice,
    /// Voice caption.
    #[serde(
        flatten,
        deserialize_with = "RawCaption::deserialize_value",
        serialize_with = "RawCaption::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<Text>,
}

impl From<Voice> for MessageDataVoice {
    fn from(value: Voice) -> Self {
        Self {
            data: value,
            caption: None,
        }
    }
}

impl MessageDataVoice {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.caption = Some(value.into());
        self
    }
}

/// Represents a service message about a user allowing a bot to write messages
/// after adding it to the attachment menu,
/// launching a Web App from a link,
/// or accepting an explicit request from a Web App
/// sent by the method `requestWriteAccess`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataWriteAccess {
    /// Indicates whether access was granted when the bot was added to the attachment or side menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
    /// Indicates whether access was granted after the user accepted an explicit request
    /// from a Web App sent by the method `requestWriteAccess`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    /// Name of the Web App,
    /// if the access was granted when the Web App was launched from a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
}

impl MessageDataWriteAccess {
    /// Sets a new value of the `from_attachment_menu` flag.
    ///
    /// * `value` - Indicates whether access was granted
    ///             when the bot was added to the attachment
    ///             or side menu.
    pub fn with_from_attachment_menu(mut self, value: bool) -> Self {
        self.from_attachment_menu = Some(value);
        self
    }

    /// Sets a new value of the `from_request` flag.
    ///
    /// * value - Indicates whether access was granted after the user accepted an explicit request
    ///           from a Web App sent by the method `requestWriteAccess`.
    pub fn with_from_request(mut self, value: bool) -> Self {
        self.from_request = Some(value);
        self
    }

    /// Sets a new name of the Web App.
    ///
    /// # Arguments
    ///
    /// * value - Name of the Web App.
    pub fn with_web_app_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.web_app_name = Some(value.into());
        self
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::types::{
    Animation,
    Audio,
    ChatBackground,
    Checklist,
    ChecklistTasksAdded,
    ChecklistTasksDone,
    CommunityChatAdded,
    CommunityChatJoined,
    CommunityChatRemoved,
    Contact,
    Dice,
    Document,
    ForumTopicIconColor,
    Game,
    GiftInfo,
    Giveaway,
    GiveawayCompleted,
    GiveawayCreated,
    GiveawayWinners,
    Integer,
    Invoice,
    LivePhoto,
    Location,
    MaybeInaccessibleMessage,
    PaidMediaInfo,
    PassportData,
    PhotoSize,
    Poll,
    RefundedPayment,
    RichMessage,
    SharedUser,
    Sticker,
    Story,
    SuccessfulPayment,
    SuggestedPostApprovalFailed,
    SuggestedPostApproved,
    SuggestedPostDeclined,
    SuggestedPostPaid,
    SuggestedPostRefunded,
    Text,
    TextEntities,
    True,
    UniqueGiftInfo,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    WebAppData,
};

/// Represents a message data.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "RawMessageData", into = "RawMessageData")]
pub enum MessageData {
    /// Information about the animation.
    Animation(Animation),
    /// Auto-delete timer settings changed.
    AutoDeleteTimerChanged(MessageDataAutoDeleteTimer),
    /// Service message: user boosted the chat.
    ///
    /// Contains a number of boosts added by the user.
    BoostAdded(Integer),
    /// The channel has been created.
    ///
    /// This field can't be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created.
    /// It can only be found in the `reply_to` field of the [`crate::types::Message`] struct
    /// if someone replies to a very first message in a channel.
    ChannelChatCreated,
    /// Chat background set.
    ChatBackgroundSet(ChatBackground),
    /// Describes a service message about an ownership change in the chat.
    ChatOwnerChanged(User),
    /// Describes a service message about the chat owner leaving the chat.
    ///
    /// Optional user is the user which will be the new owner of the chat
    /// if the previous owner does not return to the chat.
    ChatOwnerLeft(Option<User>),
    /// A chat was shared with the bot.
    ChatShared(MessageDataChatShared),
    /// Message is a checklist.
    Checklist(Checklist),
    /// Tasks were added to a checklist.
    ChecklistTasksAdded(ChecklistTasksAdded),
    /// Some tasks in a checklist were marked as done or not done.
    ChecklistTasksDone(ChecklistTasksDone),
    /// Service message: chat added to a Community.
    CommunityChatAdded(CommunityChatAdded),
    /// Service message: chat joined by a user from a community.
    CommunityChatJoined(CommunityChatJoined),
    /// Service message: chat removed from a Community
    CommunityChatRemoved(CommunityChatRemoved),
    /// The domain name of the website on which the user has logged in.
    ConnectedWebsite(String),
    /// Information about the shared contact.
    Contact(Contact),
    /// The chat photo was deleted.
    DeleteChatPhoto,
    /// A dice with a random value.
    Dice(Dice),
    /// The price for paid messages in the corresponding direct messages chat of a channel has changed.
    DirectMessagePriceChanged(MessageDataDirectMessagePriceChanged),
    /// Forum topic closed.
    ForumTopicClosed,
    /// Forum topic created.
    ForumTopicCreated(MessageDataForumTopicCreated),
    /// Forum topic edited.
    ForumTopicEdited(MessageDataForumTopicEdited),
    /// Forum topic reopened.
    ForumTopicReopened,
    /// Information about the game.
    Game(Game),
    /// A service message about a sent or received regular gift.
    Gift(GiftInfo),
    /// A service message about upgrade of a gift was purchased after the gift was sent.
    GiftUpgradeSent(GiftInfo),
    /// The 'General' forum topic hidden.
    GeneralForumTopicHidden,
    /// The 'General' forum topic unhidden.
    GeneralForumTopicUnhidden,
    /// A scheduled giveaway.
    Giveaway(Giveaway),
    /// Service message: a scheduled giveaway was created.
    GiveawayCreated(GiveawayCreated),
    /// Service message: a giveaway without public winners was completed.
    GiveawayCompleted(GiveawayCompleted),
    /// A giveaway with public winners was completed.
    GiveawayWinners(GiveawayWinners),
    /// The group has been created.
    GroupChatCreated,
    /// Information about the invoice for a payment.
    Invoice(Invoice),
    /// Information about the live photo.
    LivePhoto(LivePhoto),
    /// A member was removed from the group.
    ///
    /// This member may be the bot itself.
    LeftChatMember(User),
    /// Information about the shared location.
    Location(Location),
    /// User created a bot that will be managed by the current bot.
    ManagedBotCreated(MessageDataManagedBotCreated),
    /// The supergroup has been migrated from a group with the specified identifier.
    MigrateFromChatId(Integer),
    /// The group has been migrated to a supergroup with the specified identifier.
    MigrateToChatId(Integer),
    /// New members that were added to the group or supergroup.
    ///
    /// The bot itself may be one of these members.
    NewChatMembers(Vec<User>),
    /// A chat photo was changed to this value.
    NewChatPhoto(Vec<PhotoSize>),
    /// A chat title was changed to this value.
    NewChatTitle(String),
    /// Message contains paid media; information about the paid media.
    PaidMedia(PaidMediaInfo),
    /// A service message about the changed price for paid messages.
    PaidMessagePriceChanged(MessageDataPaidMessagePriceChanged),
    /// Telegram Passport data.
    PassportData(PassportData),
    /// Specified message was pinned.
    ///
    /// Note that the Message object in variant will not contain
    /// further `reply_to` field even if it is itself a reply.
    PinnedMessage(MaybeInaccessibleMessage),
    /// Information about the native poll.
    Poll(Poll),
    /// Answer option was added to a poll.
    PollOptionAdded(MessageDataPollOptionAdded),
    /// Answer option was deleted from a poll.
    PollOptionDeleted(MessageDataPollOptionDeleted),
    /// A user in the chat triggered another user's proximity alert while sharing Live Location.
    ProximityAlertTriggered(MessageDataProximityAlert),
    /// A service message about a refunded payment, information about the payment.
    RefundedPayment(RefundedPayment),
    /// Message is a rich formatted message.
    RichMessage(RichMessage),
    /// Information about the sticker.
    Sticker(Sticker),
    /// A forwarded story.
    Story(Story),
    /// Service message: a suggested post was approved.
    SuggestedPostApproved(SuggestedPostApproved),
    /// Service message: approval of a suggested post has failed.
    SuggestedPostApprovalFailed(SuggestedPostApprovalFailed),
    /// Service message: a suggested post was declined.
    SuggestedPostDeclined(SuggestedPostDeclined),
    /// Service message: payment for a suggested post was received.
    SuggestedPostPaid(SuggestedPostPaid),
    /// Service message: payment for a suggested post was refunded.
    SuggestedPostRefunded(SuggestedPostRefunded),
    /// Information about the successful payment.
    SuccessfulPayment(SuccessfulPayment),
    /// The supergroup has been created.
    ///
    /// This field can't be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created
    /// It can only be found in the `reply_to` field of the [`crate::types::Message`] struct
    /// if someone replies to a very first message
    /// in a directly created supergroup.
    SupergroupChatCreated,
    /// A service message about a sent or received unique gift.
    UniqueGift(Box<UniqueGiftInfo>),
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
    Audio(MessageDataAudio),
    /// Describes the document.
    Document(MessageDataDocument),
    /// Available sizes of the photo.
    Photo(MessageDataPhoto),
    /// The actual UTF-8 text of the message; 0-4096 characters.
    Text(Text),
    /// Describes the video.
    Video(MessageDataVideo),
    /// Describes the voice.
    Voice(MessageDataVoice),
    /// Contains arbitrary data for future variants.
    Unknown(JsonValue),
}

/// Represents a service message about a change in auto-delete timer settings.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataAutoDeleteTimer {
    /// New auto-delete time for messages in the chat; in seconds.
    #[serde(rename = "message_auto_delete_time")]
    pub time: Integer,
}

/// Represents an audio message data.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageDataAudio {
    /// Audio data.
    pub data: Audio,
    /// Audio caption.
    pub caption: Option<Text>,
}

/// Represents information about the chat
/// whose identifier was shared with the bot
/// using a [`crate::types::KeyboardButtonRequestChat`] button.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataChatShared {
    /// Identifier of the shared chat.
    ///
    /// The bot may not have access to the chat and could be unable to use this identifier,
    /// unless the chat is already known to the bot by some other means.
    pub chat_id: Integer,
    /// Identifier of the request.
    pub request_id: Integer,
    /// Available sizes of the chat photo, if the photo was requested by the bot.
    pub photo: Option<Vec<PhotoSize>>,
    /// Title of the chat, if the title was requested by the bot.
    pub title: Option<String>,
    /// Username of the chat, if the username was requested by the bot and available.
    pub username: Option<String>,
}

/// Represents an document message data.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageDataDocument {
    /// Document data.
    pub data: Document,
    /// Document caption.
    pub caption: Option<Text>,
}

/// Represents a service message about a new forum topic created in the chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataForumTopicCreated {
    /// Color of the icon of the topic.
    pub icon_color: ForumTopicIconColor,
    /// Name of the topic.
    pub name: String,
    /// Unique identifier of the custom emoji shown as the topic icon.
    pub icon_custom_emoji_id: Option<String>,
    /// Whether the name of the topic wasn't specified explicitly by its creator
    /// and likely needs to be changed by the bot.
    pub is_name_implicit: Option<bool>,
}

/// Represents a service message about an edited forum topic.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataForumTopicEdited {
    /// New name, if it was edited.
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon,
    /// if it was edited; an empty string if the icon was removed.
    pub icon_custom_emoji_id: Option<String>,
}

/// Contains information about the bot that was created to be managed by the current bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataManagedBotCreated {
    /// Information about the bot.
    pub user: User,
}

/// Describes a service message about a change in the price of paid messages within a chat.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataPaidMessagePriceChanged {
    /// The new number of Telegram Stars
    /// that must be paid by non-administrator users of the supergroup chat for each sent message.
    pub paid_message_star_count: Integer,
}

/// Describes a service message about a change in the price of direct messages sent to a channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataDirectMessagePriceChanged {
    /// Whether the direct messages are enabled for the channel chat.
    pub are_direct_messages_enabled: bool,
    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel.
    ///
    /// Does not apply to users who have been exempted by administrators.
    /// Defaults to 0.
    pub direct_message_star_count: Option<Integer>,
}

/// Represents a list of available sizes of the photo.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageDataPhoto {
    /// Photo sizes.
    pub data: Vec<PhotoSize>,
    /// Photo caption.
    pub caption: Option<Text>,
}

/// Describes a service message about an option added to a poll.
#[derive(Clone, Debug)]
pub struct MessageDataPollOptionAdded {
    /// Unique identifier of the added option.
    pub option_persistent_id: String,
    /// Option text.
    pub option: Text,
    /// Message containing the poll to which the option was added, if known.
    pub poll_message: Option<MaybeInaccessibleMessage>,
}

/// Desribes a service message about an option deleted from a poll.
#[derive(Clone, Debug)]
pub struct MessageDataPollOptionDeleted {
    /// Unique identifier of the deleted option.
    pub option_persistent_id: String,
    /// Option text.
    pub option: Text,
    /// Message containing the poll from which the option was deleted, if known.
    pub poll_message: Option<MaybeInaccessibleMessage>,
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

/// Contains information about the users
/// whose identifiers were shared with the bot
/// using a [`crate::types::KeyboardButton::with_request_users`] button.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataUsersShared {
    /// Identifier of the request.
    pub request_id: Integer,
    /// Information about users shared with the bot.
    pub users: Vec<SharedUser>,
}

/// Represents a video message data.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageDataVideo {
    /// Video data.
    pub data: Video,
    /// Video caption.
    pub caption: Option<Text>,
}

/// Represents a service message about a video chat ended in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatEnded {
    /// Video chat duration; in seconds.
    pub duration: Integer,
}

/// A service message about new members invited to a video chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatParticipantsInvited {
    /// New members that were invited to the voice chat.
    pub users: Option<Vec<User>>,
}

/// A service message about a video chat scheduled in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageDataVideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat
    /// is supposed to be started by a chat administrator.
    pub start_date: Integer,
}

/// Message is a voice message, information about the file.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageDataVoice {
    /// Voice data.
    pub data: Voice,
    /// Voice caption.
    pub caption: Option<Text>,
}

/// Represents a service message about a user allowing a bot to write messages
/// after adding it to the attachment menu,
/// launching a Web App from a link,
/// or accepting an explicit request from a Web App
/// sent by the method `requestWriteAccess`.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageDataWriteAccess {
    /// Indicates whether access was granted when the bot was added to the attachment or side menu.
    pub from_attachment_menu: Option<bool>,
    /// Indicates whether access was granted after the user accepted an explicit request
    /// from a Web App sent by the method `requestWriteAccess`.
    pub from_request: Option<bool>,
    /// Name of the Web App,
    /// if the access was granted when the Web App was launched from a link.
    pub web_app_name: Option<String>,
}

/// Represents a message data.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum RawMessageData {
    Animation(Animation),
    #[serde(rename = "message_auto_delete_timer_changed")]
    AutoDeleteTimerChanged(MessageDataAutoDeleteTimer),
    BoostAdded {
        boost_count: Integer,
    },
    ChannelChatCreated(True),
    ChatBackgroundSet(ChatBackground),
    ChatOwnerChanged {
        new_owner: User,
    },
    ChatOwnerLeft {
        new_owner: Option<User>,
    },
    ChatShared(MessageDataChatShared),
    Checklist(Checklist),
    ChecklistTasksAdded(ChecklistTasksAdded),
    ChecklistTasksDone(ChecklistTasksDone),
    CommunityChatAdded(CommunityChatAdded),
    CommunityChatJoined(CommunityChatJoined),
    CommunityChatRemoved(CommunityChatRemoved),
    ConnectedWebsite(String),
    Contact(Contact),
    DeleteChatPhoto(True),
    Dice(Dice),
    DirectMessagePriceChanged(MessageDataDirectMessagePriceChanged),
    ForumTopicClosed {},
    ForumTopicCreated(MessageDataForumTopicCreated),
    ForumTopicEdited(MessageDataForumTopicEdited),
    ForumTopicReopened {},
    Game(Game),
    Gift(GiftInfo),
    GiftUpgradeSent(GiftInfo),
    GeneralForumTopicHidden {},
    GeneralForumTopicUnhidden {},
    Giveaway(Giveaway),
    GiveawayCreated(GiveawayCreated),
    GiveawayCompleted(GiveawayCompleted),
    GiveawayWinners(GiveawayWinners),
    GroupChatCreated(True),
    Invoice(Invoice),
    LeftChatMember(User),
    Location(Location),
    ManagedBotCreated(MessageDataManagedBotCreated),
    MigrateFromChatId(Integer),
    MigrateToChatId(Integer),
    NewChatMembers(Vec<User>),
    NewChatPhoto(Vec<PhotoSize>),
    NewChatTitle(String),
    PaidMedia(PaidMediaInfo),
    PaidMessagePriceChanged(MessageDataPaidMessagePriceChanged),
    PassportData(PassportData),
    PinnedMessage(MaybeInaccessibleMessage),
    Poll(Poll),
    PollOptionAdded {
        option_persistent_id: String,
        option_text: String,
        option_text_entities: Option<TextEntities>,
        poll_message: Option<MaybeInaccessibleMessage>,
    },
    PollOptionDeleted {
        option_persistent_id: String,
        option_text: String,
        option_text_entities: Option<TextEntities>,
        poll_message: Option<MaybeInaccessibleMessage>,
    },
    ProximityAlertTriggered(MessageDataProximityAlert),
    RefundedPayment(RefundedPayment),
    RichMessage(RichMessage),
    Sticker(Sticker),
    Story(Story),
    SuggestedPostApproved(SuggestedPostApproved),
    SuggestedPostApprovalFailed(SuggestedPostApprovalFailed),
    SuggestedPostDeclined(SuggestedPostDeclined),
    SuggestedPostPaid(SuggestedPostPaid),
    SuggestedPostRefunded(SuggestedPostRefunded),
    SuccessfulPayment(SuccessfulPayment),
    SupergroupChatCreated(True),
    UniqueGift(Box<UniqueGiftInfo>),
    UsersShared(MessageDataUsersShared),
    Venue(Venue),
    VideoNote(VideoNote),
    VideoChatEnded(MessageDataVideoChatEnded),
    VideoChatParticipantsInvited(MessageDataVideoChatParticipantsInvited),
    VideoChatScheduled(MessageDataVideoChatScheduled),
    VideoChatStarted {},
    WebAppData(WebAppData),
    WriteAccessAllowed(MessageDataWriteAccess),
    #[serde(untagged)]
    Audio {
        audio: Audio,
        caption: Option<String>,
        caption_entities: Option<TextEntities>,
    },
    #[serde(untagged)]
    Document {
        document: Document,
        caption: Option<String>,
        caption_entities: Option<TextEntities>,
    },
    /// When both photo and a live_photo are provided, the live_photo is ignored.
    /// Using untagged to parse the live_photo first
    #[serde(untagged)]
    LivePhoto {
        live_photo: LivePhoto,
    },
    #[serde(untagged)]
    Photo {
        photo: Vec<PhotoSize>,
        caption: Option<String>,
        caption_entities: Option<TextEntities>,
    },
    #[serde(untagged)]
    Text {
        text: String,
        entities: Option<TextEntities>,
    },
    #[serde(untagged)]
    Video {
        video: Video,
        caption: Option<String>,
        caption_entities: Option<TextEntities>,
    },
    #[serde(untagged)]
    Voice {
        voice: Voice,
        caption: Option<String>,
        caption_entities: Option<TextEntities>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

impl From<RawMessageData> for MessageData {
    fn from(value: RawMessageData) -> Self {
        match value {
            RawMessageData::Animation(value) => Self::Animation(value),
            RawMessageData::AutoDeleteTimerChanged(value) => Self::AutoDeleteTimerChanged(value),
            RawMessageData::BoostAdded { boost_count } => Self::BoostAdded(boost_count),
            RawMessageData::ChannelChatCreated(True) => Self::ChannelChatCreated,
            RawMessageData::ChatBackgroundSet(value) => Self::ChatBackgroundSet(value),
            RawMessageData::ChatOwnerChanged { new_owner } => Self::ChatOwnerChanged(new_owner),
            RawMessageData::ChatOwnerLeft { new_owner } => Self::ChatOwnerLeft(new_owner),
            RawMessageData::ChatShared(value) => Self::ChatShared(value),
            RawMessageData::Checklist(value) => Self::Checklist(value),
            RawMessageData::ChecklistTasksAdded(value) => Self::ChecklistTasksAdded(value),
            RawMessageData::ChecklistTasksDone(value) => Self::ChecklistTasksDone(value),
            RawMessageData::CommunityChatAdded(value) => Self::CommunityChatAdded(value),
            RawMessageData::CommunityChatJoined(value) => Self::CommunityChatJoined(value),
            RawMessageData::CommunityChatRemoved(value) => Self::CommunityChatRemoved(value),
            RawMessageData::ConnectedWebsite(value) => Self::ConnectedWebsite(value),
            RawMessageData::Contact(value) => Self::Contact(value),
            RawMessageData::DeleteChatPhoto(True) => Self::DeleteChatPhoto,
            RawMessageData::Dice(value) => Self::Dice(value),
            RawMessageData::DirectMessagePriceChanged(value) => Self::DirectMessagePriceChanged(value),
            RawMessageData::ForumTopicClosed {} => Self::ForumTopicClosed,
            RawMessageData::ForumTopicCreated(value) => Self::ForumTopicCreated(value),
            RawMessageData::ForumTopicEdited(value) => Self::ForumTopicEdited(value),
            RawMessageData::ForumTopicReopened {} => Self::ForumTopicReopened,
            RawMessageData::Game(value) => Self::Game(value),
            RawMessageData::Gift(value) => Self::Gift(value),
            RawMessageData::GiftUpgradeSent(value) => Self::GiftUpgradeSent(value),
            RawMessageData::GeneralForumTopicHidden {} => Self::GeneralForumTopicHidden,
            RawMessageData::GeneralForumTopicUnhidden {} => Self::GeneralForumTopicUnhidden,
            RawMessageData::Giveaway(value) => Self::Giveaway(value),
            RawMessageData::GiveawayCreated(value) => Self::GiveawayCreated(value),
            RawMessageData::GiveawayCompleted(value) => Self::GiveawayCompleted(value),
            RawMessageData::GiveawayWinners(value) => Self::GiveawayWinners(value),
            RawMessageData::GroupChatCreated(True) => Self::GroupChatCreated,
            RawMessageData::Invoice(value) => Self::Invoice(value),
            RawMessageData::LeftChatMember(value) => Self::LeftChatMember(value),
            RawMessageData::Location(value) => Self::Location(value),
            RawMessageData::ManagedBotCreated(value) => Self::ManagedBotCreated(value),
            RawMessageData::MigrateFromChatId(value) => Self::MigrateFromChatId(value),
            RawMessageData::MigrateToChatId(value) => Self::MigrateToChatId(value),
            RawMessageData::NewChatMembers(value) => Self::NewChatMembers(value),
            RawMessageData::NewChatPhoto(value) => Self::NewChatPhoto(value),
            RawMessageData::NewChatTitle(value) => Self::NewChatTitle(value),
            RawMessageData::PaidMedia(value) => Self::PaidMedia(value),
            RawMessageData::PaidMessagePriceChanged(value) => Self::PaidMessagePriceChanged(value),
            RawMessageData::PassportData(value) => Self::PassportData(value),
            RawMessageData::PinnedMessage(value) => Self::PinnedMessage(value),
            RawMessageData::Poll(value) => Self::Poll(value),
            RawMessageData::PollOptionAdded {
                option_persistent_id,
                option_text,
                option_text_entities,
                poll_message,
            } => Self::PollOptionAdded(MessageDataPollOptionAdded {
                option_persistent_id,
                option: Text {
                    data: option_text,
                    entities: option_text_entities,
                },
                poll_message,
            }),
            RawMessageData::PollOptionDeleted {
                option_persistent_id,
                option_text,
                option_text_entities,
                poll_message,
            } => Self::PollOptionDeleted(MessageDataPollOptionDeleted {
                option_persistent_id,
                option: Text {
                    data: option_text,
                    entities: option_text_entities,
                },
                poll_message,
            }),
            RawMessageData::ProximityAlertTriggered(value) => Self::ProximityAlertTriggered(value),
            RawMessageData::RefundedPayment(value) => Self::RefundedPayment(value),
            RawMessageData::RichMessage(value) => Self::RichMessage(value),
            RawMessageData::Sticker(value) => Self::Sticker(value),
            RawMessageData::Story(value) => Self::Story(value),
            RawMessageData::SuggestedPostApproved(value) => Self::SuggestedPostApproved(value),
            RawMessageData::SuggestedPostApprovalFailed(value) => Self::SuggestedPostApprovalFailed(value),
            RawMessageData::SuggestedPostDeclined(value) => Self::SuggestedPostDeclined(value),
            RawMessageData::SuggestedPostPaid(value) => Self::SuggestedPostPaid(value),
            RawMessageData::SuggestedPostRefunded(value) => Self::SuggestedPostRefunded(value),
            RawMessageData::SuccessfulPayment(value) => Self::SuccessfulPayment(value),
            RawMessageData::SupergroupChatCreated(True) => Self::SupergroupChatCreated,
            RawMessageData::UniqueGift(value) => Self::UniqueGift(value),
            RawMessageData::UsersShared(value) => Self::UsersShared(value),
            RawMessageData::Venue(value) => Self::Venue(value),
            RawMessageData::VideoNote(value) => Self::VideoNote(value),
            RawMessageData::VideoChatEnded(value) => Self::VideoChatEnded(value),
            RawMessageData::VideoChatParticipantsInvited(value) => Self::VideoChatParticipantsInvited(value),
            RawMessageData::VideoChatScheduled(value) => Self::VideoChatScheduled(value),
            RawMessageData::VideoChatStarted {} => Self::VideoChatStarted,
            RawMessageData::WebAppData(value) => Self::WebAppData(value),
            RawMessageData::WriteAccessAllowed(value) => Self::WriteAccessAllowed(value),
            RawMessageData::Audio {
                audio,
                caption,
                caption_entities,
            } => Self::Audio(MessageDataAudio {
                data: audio,
                caption: caption.map(|data| Text {
                    data,
                    entities: caption_entities,
                }),
            }),
            RawMessageData::Document {
                document,
                caption,
                caption_entities,
            } => Self::Document(MessageDataDocument {
                data: document,
                caption: caption.map(|data| Text {
                    data,
                    entities: caption_entities,
                }),
            }),
            RawMessageData::LivePhoto { live_photo } => Self::LivePhoto(live_photo),
            RawMessageData::Photo {
                photo,
                caption,
                caption_entities,
            } => Self::Photo(MessageDataPhoto {
                data: photo,

                caption: caption.map(|data| Text {
                    data,
                    entities: caption_entities,
                }),
            }),
            RawMessageData::Text { text: data, entities } => Self::Text(Text { data, entities }),
            RawMessageData::Video {
                video,
                caption,
                caption_entities,
            } => Self::Video(MessageDataVideo {
                data: video,

                caption: caption.map(|data| Text {
                    data,
                    entities: caption_entities,
                }),
            }),
            RawMessageData::Voice {
                voice,
                caption,
                caption_entities,
            } => Self::Voice(MessageDataVoice {
                data: voice,

                caption: caption.map(|data| Text {
                    data,
                    entities: caption_entities,
                }),
            }),
            RawMessageData::Unknown(value) => Self::Unknown(value),
        }
    }
}

impl From<MessageData> for RawMessageData {
    fn from(value: MessageData) -> Self {
        match value {
            MessageData::Animation(value) => Self::Animation(value),
            MessageData::AutoDeleteTimerChanged(value) => Self::AutoDeleteTimerChanged(value),
            MessageData::BoostAdded(boost_count) => Self::BoostAdded { boost_count },
            MessageData::ChannelChatCreated => Self::ChannelChatCreated(True),
            MessageData::ChatBackgroundSet(value) => Self::ChatBackgroundSet(value),
            MessageData::ChatOwnerChanged(new_owner) => Self::ChatOwnerChanged { new_owner },
            MessageData::ChatOwnerLeft(new_owner) => Self::ChatOwnerLeft { new_owner },
            MessageData::ChatShared(value) => Self::ChatShared(value),
            MessageData::Checklist(value) => Self::Checklist(value),
            MessageData::ChecklistTasksAdded(value) => Self::ChecklistTasksAdded(value),
            MessageData::ChecklistTasksDone(value) => Self::ChecklistTasksDone(value),
            MessageData::CommunityChatAdded(value) => Self::CommunityChatAdded(value),
            MessageData::CommunityChatJoined(value) => Self::CommunityChatJoined(value),
            MessageData::CommunityChatRemoved(value) => Self::CommunityChatRemoved(value),
            MessageData::ConnectedWebsite(value) => Self::ConnectedWebsite(value),
            MessageData::Contact(value) => Self::Contact(value),
            MessageData::DeleteChatPhoto => Self::DeleteChatPhoto(True),
            MessageData::Dice(value) => Self::Dice(value),
            MessageData::DirectMessagePriceChanged(value) => Self::DirectMessagePriceChanged(value),
            MessageData::ForumTopicClosed => Self::ForumTopicClosed {},
            MessageData::ForumTopicCreated(value) => Self::ForumTopicCreated(value),
            MessageData::ForumTopicEdited(value) => Self::ForumTopicEdited(value),
            MessageData::ForumTopicReopened => Self::ForumTopicReopened {},
            MessageData::Game(value) => Self::Game(value),
            MessageData::Gift(value) => Self::Gift(value),
            MessageData::GiftUpgradeSent(value) => Self::GiftUpgradeSent(value),
            MessageData::GeneralForumTopicHidden => Self::GeneralForumTopicHidden {},
            MessageData::GeneralForumTopicUnhidden => Self::GeneralForumTopicUnhidden {},
            MessageData::Giveaway(value) => Self::Giveaway(value),
            MessageData::GiveawayCreated(value) => Self::GiveawayCreated(value),
            MessageData::GiveawayCompleted(value) => Self::GiveawayCompleted(value),
            MessageData::GiveawayWinners(value) => Self::GiveawayWinners(value),
            MessageData::GroupChatCreated => Self::GroupChatCreated(True),
            MessageData::Invoice(value) => Self::Invoice(value),
            MessageData::LeftChatMember(value) => Self::LeftChatMember(value),
            MessageData::Location(value) => Self::Location(value),
            MessageData::ManagedBotCreated(value) => Self::ManagedBotCreated(value),
            MessageData::MigrateFromChatId(value) => Self::MigrateFromChatId(value),
            MessageData::MigrateToChatId(value) => Self::MigrateToChatId(value),
            MessageData::NewChatMembers(value) => Self::NewChatMembers(value),
            MessageData::NewChatPhoto(value) => Self::NewChatPhoto(value),
            MessageData::NewChatTitle(value) => Self::NewChatTitle(value),
            MessageData::PaidMedia(value) => Self::PaidMedia(value),
            MessageData::PaidMessagePriceChanged(value) => Self::PaidMessagePriceChanged(value),
            MessageData::PassportData(value) => Self::PassportData(value),
            MessageData::PinnedMessage(value) => Self::PinnedMessage(value),
            MessageData::Poll(value) => Self::Poll(value),
            MessageData::PollOptionAdded(MessageDataPollOptionAdded {
                option_persistent_id,
                option,
                poll_message,
            }) => Self::PollOptionAdded {
                option_persistent_id,
                option_text: option.data,
                option_text_entities: option.entities,
                poll_message,
            },
            MessageData::PollOptionDeleted(MessageDataPollOptionDeleted {
                option_persistent_id,
                option,
                poll_message,
            }) => Self::PollOptionDeleted {
                option_persistent_id,
                option_text: option.data,
                option_text_entities: option.entities,
                poll_message,
            },
            MessageData::ProximityAlertTriggered(value) => Self::ProximityAlertTriggered(value),
            MessageData::RefundedPayment(value) => Self::RefundedPayment(value),
            MessageData::RichMessage(value) => Self::RichMessage(value),
            MessageData::Sticker(value) => Self::Sticker(value),
            MessageData::Story(value) => Self::Story(value),
            MessageData::SuggestedPostApproved(value) => Self::SuggestedPostApproved(value),
            MessageData::SuggestedPostApprovalFailed(value) => Self::SuggestedPostApprovalFailed(value),
            MessageData::SuggestedPostDeclined(value) => Self::SuggestedPostDeclined(value),
            MessageData::SuggestedPostPaid(value) => Self::SuggestedPostPaid(value),
            MessageData::SuggestedPostRefunded(value) => Self::SuggestedPostRefunded(value),
            MessageData::SuccessfulPayment(value) => Self::SuccessfulPayment(value),
            MessageData::SupergroupChatCreated => Self::SupergroupChatCreated(True),
            MessageData::UniqueGift(value) => Self::UniqueGift(value),
            MessageData::UsersShared(value) => Self::UsersShared(value),
            MessageData::Venue(value) => Self::Venue(value),
            MessageData::VideoNote(value) => Self::VideoNote(value),
            MessageData::VideoChatEnded(value) => Self::VideoChatEnded(value),
            MessageData::VideoChatParticipantsInvited(value) => Self::VideoChatParticipantsInvited(value),
            MessageData::VideoChatScheduled(value) => Self::VideoChatScheduled(value),
            MessageData::VideoChatStarted => Self::VideoChatStarted {},
            MessageData::WebAppData(value) => Self::WebAppData(value),
            MessageData::WriteAccessAllowed(value) => Self::WriteAccessAllowed(value),
            MessageData::Audio(value) => {
                let audio = value.data;
                let (caption, caption_entities) = value
                    .caption
                    .map(|x| (Some(x.data), x.entities))
                    .unwrap_or((None, None));
                Self::Audio {
                    audio,
                    caption,
                    caption_entities,
                }
            }
            MessageData::Document(value) => {
                let document = value.data;
                let (caption, caption_entities) = value
                    .caption
                    .map(|x| (Some(x.data), x.entities))
                    .unwrap_or((None, None));
                Self::Document {
                    document,
                    caption,
                    caption_entities,
                }
            }
            MessageData::LivePhoto(live_photo) => Self::LivePhoto { live_photo },
            MessageData::Photo(value) => {
                let photo = value.data;
                let (caption, caption_entities) = value
                    .caption
                    .map(|x| (Some(x.data), x.entities))
                    .unwrap_or((None, None));
                Self::Photo {
                    photo,
                    caption,
                    caption_entities,
                }
            }
            MessageData::Text(Text { data: text, entities }) => Self::Text { text, entities },
            MessageData::Video(value) => {
                let video = value.data;
                let (caption, caption_entities) = value
                    .caption
                    .map(|x| (Some(x.data), x.entities))
                    .unwrap_or((None, None));
                Self::Video {
                    video,
                    caption,
                    caption_entities,
                }
            }
            MessageData::Voice(value) => {
                let voice = value.data;
                let (caption, caption_entities) = value
                    .caption
                    .map(|x| (Some(x.data), x.entities))
                    .unwrap_or((None, None));
                Self::Voice {
                    voice,
                    caption,
                    caption_entities,
                }
            }
            MessageData::Unknown(value) => Self::Unknown(value),
        }
    }
}

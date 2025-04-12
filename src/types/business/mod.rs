use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{AcceptedGiftTypes, Chat, InputProfilePhoto, InputProfilePhotoError, Integer, Location, Sticker, User},
};

#[cfg(test)]
mod tests;

/// Represents the rights of a business bot
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessBotRights {
    /// Whether the bot can change the privacy settings pertaining to gifts for the business account.
    pub can_change_gift_settings: Option<bool>,
    /// Whether the bot can convert regular gifts owned by the business account to Telegram Stars.
    pub can_convert_gifts_to_stars: Option<bool>,
    /// Whether the bot can delete all private messages in managed chats.
    pub can_delete_all_messages: Option<bool>,
    /// Whether the bot can delete messages sent by the bot.
    pub can_delete_outgoing_messages: Option<bool>,
    /// Whether the bot can edit the bio of the business account.
    pub can_edit_bio: Option<bool>,
    /// Whether the bot can edit the first and last name of the business account.
    pub can_edit_name: Option<bool>,
    /// Whether the bot can edit the profile photo of the business account.
    pub can_edit_profile_photo: Option<bool>,
    /// Whether the bot can edit the username of the business account.
    pub can_edit_username: Option<bool>,
    /// Whether the bot can post, edit and delete stories on behalf of the business account.
    pub can_manage_stories: Option<bool>,
    /// Whether the bot can mark incoming private messages as read.
    pub can_read_messages: Option<bool>,
    /// Whether the bot can send and edit messages in the private chats
    /// that had incoming messages in the last 24 hours.
    pub can_reply: Option<bool>,
    /// Whether the bot can transfer and upgrade gifts owned by the business account.
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// Whether the bot can transfer Telegram Stars received by the business account to its own account,
    /// or use them to upgrade and transfer gifts.
    pub can_transfer_stars: Option<bool>,
    /// Whether the bot can view gifts and the amount of Telegram Stars owned by the business account.
    pub can_view_gifts_and_stars: Option<bool>,
}

impl BusinessBotRights {
    /// Sets a new value for the `can_change_gift_settings` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can change the privacy settings pertaining to gifts for the business account.
    pub fn with_can_change_gift_settings(mut self, value: bool) -> Self {
        self.can_change_gift_settings = Some(value);
        self
    }

    /// Sets a new value for the `can_convert_gifts_to_stars` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can convert regular gifts owned by the business account to Telegram Stars.
    pub fn with_can_convert_gifts_to_stars(mut self, value: bool) -> Self {
        self.can_convert_gifts_to_stars = Some(value);
        self
    }

    /// Sets a new value for the `can_delete_all_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can delete all private messages in managed chats.
    pub fn with_can_delete_all_messages(mut self, value: bool) -> Self {
        self.can_delete_all_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_delete_outgoing_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can delete messages sent by the bot.
    pub fn with_can_delete_outgoing_messages(mut self, value: bool) -> Self {
        self.can_delete_outgoing_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_bio` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can edit the bio of the business account.
    pub fn with_can_edit_bio(mut self, value: bool) -> Self {
        self.can_edit_bio = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_name` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can edit the first and last name of the business account.
    pub fn with_can_edit_name(mut self, value: bool) -> Self {
        self.can_edit_name = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_profile_photo` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can edit the profile photo of the business account.
    pub fn with_can_edit_profile_photo(mut self, value: bool) -> Self {
        self.can_edit_profile_photo = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_username` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can edit the username of the business account.
    pub fn with_can_edit_username(mut self, value: bool) -> Self {
        self.can_edit_username = Some(value);
        self
    }

    /// Sets a new value for the `can_manage_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can post, edit and delete stories on behalf of the business account.
    pub fn with_can_manage_stories(mut self, value: bool) -> Self {
        self.can_manage_stories = Some(value);
        self
    }

    /// Sets a new value for the `can_read_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can mark incoming private messages as read.
    pub fn with_can_read_messages(mut self, value: bool) -> Self {
        self.can_read_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_reply` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can send and edit messages in the private chats
    ///   that had incoming messages in the last 24 hours.
    pub fn with_can_reply(mut self, value: bool) -> Self {
        self.can_reply = Some(value);
        self
    }

    /// Sets a new value for the `can_transfer_and_upgrade_gifts` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can transfer and upgrade gifts owned by the business account.
    pub fn with_can_transfer_and_upgrade_gifts(mut self, value: bool) -> Self {
        self.can_transfer_and_upgrade_gifts = Some(value);
        self
    }

    /// Sets a new value for the `can_transfer_stars` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can transfer Telegram Stars received by the business account to its own account,
    ///   or use them to upgrade and transfer gifts.
    pub fn with_can_transfer_stars(mut self, value: bool) -> Self {
        self.can_transfer_stars = Some(value);
        self
    }

    /// Sets a new value for the `can_view_gifts_and_stars` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can view gifts and the amount of Telegram Stars owned by the business account.
    pub fn with_can_view_gifts_and_stars(mut self, value: bool) -> Self {
        self.can_view_gifts_and_stars = Some(value);
        self
    }
}

/// Describes the connection of the bot with a business account.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessConnection {
    /// Date the connection was established in Unix time.
    pub date: Integer,
    /// Unique identifier of the business connection.
    pub id: String,
    /// Whether the connection is active.
    pub is_enabled: bool,
    /// Business account user that created the business connection.
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection.
    pub user_chat_id: Integer,
    /// Rights of the business bot.
    pub rights: Option<BusinessBotRights>,
}

impl BusinessConnection {
    /// Creates a new `BusinessConnection`.
    ///
    /// # Arguments
    ///
    /// * `date` - Date the connection was established in Unix time.
    /// * `id` - Unique identifier of the business connection.
    /// * `user` - Business account user that created the business connection.
    /// * `user_chat_id` - Identifier of a private chat with the user who created the business connection.
    pub fn new<T>(date: Integer, id: T, user: User, user_chat_id: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            date,
            id: id.into(),
            is_enabled: false,
            user,
            user_chat_id,
            rights: None,
        }
    }

    /// Sets a new value for the `is_enabled` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the connection is active.
    pub fn with_is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = value;
        self
    }

    /// Sets a new rights.
    ///
    /// # Arguments
    ///
    /// * `value` - Rights of the business bot.
    pub fn with_rights(mut self, value: BusinessBotRights) -> Self {
        self.rights = Some(value);
        self
    }
}

/// Represents the intro of the business.
#[serde_with::skip_serializing_none]
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessIntro {
    /// Message text of the business intro.
    pub message: Option<String>,
    /// Sticker of the business intro.
    pub sticker: Option<Sticker>,
    /// Title text of the business intro.
    pub title: Option<String>,
}

impl BusinessIntro {
    /// Sets a new message.
    ///
    /// # Arguments
    ///
    /// * `value` - .Message text of the business intro.
    pub fn with_message<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.message = Some(value.into());
        self
    }

    /// Sets a new sticker.
    ///
    /// # Arguments
    ///
    /// * `value` - .Sticker of the business intro.
    pub fn with_sticker(mut self, value: Sticker) -> Self {
        self.sticker = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - .Title text of the business intro.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

/// Provides information about address and location of the business.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,
    /// Location of the business.
    pub location: Option<Location>,
}

impl BusinessLocation {
    /// Creates a new `BusinessLocation`.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the business.
    pub fn new<T>(address: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            address: address.into(),
            location: None,
        }
    }

    /// Sets a new location.
    ///
    /// # Arguments
    ///
    /// * `value` - Location of the business.
    pub fn with_location(mut self, value: Location) -> Self {
        self.location = Some(value);
        self
    }
}

/// Provides information about messages deleted from a connected business account.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,
    /// Information about a chat in the business account.
    /// The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// A list of identifiers of deleted messages in the chat of the business account.
    pub message_ids: Vec<Integer>,
}

impl BusinessMessagesDeleted {
    /// Creates a new `BusinessMessagesDeleted`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `chat` - Information about a chat in the business account.
    /// * `message_ids` - A list of identifiers of deleted messages in the chat of the business account.
    pub fn new<A, B, C>(business_connection_id: A, chat: B, message_ids: C) -> Self
    where
        A: Into<String>,
        B: Into<Chat>,
        C: IntoIterator<Item = Integer>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            chat: chat.into(),
            message_ids: message_ids.into_iter().collect(),
        }
    }
}

/// Provides information about the time interval describing business opening hours.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday,
    /// marking the start of the time interval during which the business is open; 0 - 7 24 60.
    pub opening_minute: Integer,
    /// The minute's sequence number in a week, starting on Monday,
    /// marking the end of the time interval during which the business is open; 0 - 8 24 60.
    pub closing_minute: Integer,
}

impl BusinessOpeningHoursInterval {
    /// Creates a new `BusinessOpeningHoursInterval`.
    ///
    /// # Arguments
    ///
    /// * `opening_minute` - Start of the time interval during which the business is open.
    /// * `closing_minute` - End of the time interval during which the business is open.
    pub fn new(opening_minute: Integer, closing_minute: Integer) -> Self {
        Self {
            opening_minute,
            closing_minute,
        }
    }
}

impl From<(Integer, Integer)> for BusinessOpeningHoursInterval {
    fn from((opening_minute, closing_minute): (Integer, Integer)) -> Self {
        Self::new(opening_minute, closing_minute)
    }
}

/// Provides information about the opening hours of the business.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined.
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

impl BusinessOpeningHours {
    /// Creates a new `BusinessOpeningHours`.
    ///
    /// # Arguments
    ///
    /// * `time_zone_name` - Unique name of the time zone for which the opening hours are defined.
    /// * `opening_hours` - List of time intervals describing business opening hours.
    pub fn new<A, B, C>(time_zone_name: A, opening_hours: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = C>,
        C: Into<BusinessOpeningHoursInterval>,
    {
        Self {
            time_zone_name: time_zone_name.into(),
            opening_hours: opening_hours.into_iter().map(Into::into).collect(),
        }
    }
}

/// Deletes messages on behalf of a business account.
///
/// Requires the `can_delete_outgoing_messages` business bot right to delete messages sent by the bot itself,
/// or the `can_delete_all_messages` business bot right to delete any message.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteBusinessMessages {
    business_connection_id: String,
    message_ids: Vec<Integer>,
}

impl DeleteBusinessMessages {
    /// Creates a new `DeleteBusinessMessages`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection
    ///   on behalf of which to delete the messages.
    /// * `message_ids` - A list of 1-100 identifiers of messages to delete;
    ///   all messages must be from the same chat;
    ///   see deleteMessage for limitations on which messages can be deleted.
    pub fn new<A, B>(business_connection_id: A, message_ids: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = Integer>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            message_ids: message_ids.into_iter().collect(),
        }
    }
}

impl Method for DeleteBusinessMessages {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteBusinessMessages", self)
    }
}

/// Returns information about the connection of the bot with a business account.
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessConnection {
    business_connection_id: String,
}

impl GetBusinessConnection {
    /// Creates a new `GetBusinessConnection`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
        }
    }
}

impl Method for GetBusinessConnection {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("getBusinessConnection", self)
    }
}

/// Marks incoming message as read on behalf of a business account.
///
/// Requires the can_read_messages business bot right.
#[derive(Clone, Debug, Serialize)]
pub struct ReadBusinessMessage {
    business_connection_id: String,
    chat_id: Integer,
    message_id: Integer,
}

impl ReadBusinessMessage {
    /// Creates a new `ReadBusinessMessage`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection
    ///   on behalf of which to read the message.
    /// * `chat_id` - Unique identifier of the chat in which the message was received;
    ///   the chat must have been active in the last 24 hours.
    /// * `message_id` - Unique identifier of the message to mark as read.
    pub fn new<T>(business_connection_id: T, chat_id: Integer, message_id: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
        }
    }
}

impl Method for ReadBusinessMessage {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("readBusinessMessage", self)
    }
}

/// Removes the current profile photo of a managed business account.
///
/// Requires the can_edit_profile_photo business bot right.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct RemoveBusinessAccountProfilePhoto {
    business_connection_id: String,
    is_public: Option<bool>,
}

impl RemoveBusinessAccountProfilePhoto {
    /// Creates a new `RemoveBusinessAccountProfilePhoto`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            is_public: None,
        }
    }

    /// Sets a new value for the `is_public` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to remove the public photo,
    ///   which is visible even if the main photo is hidden by the business account's privacy settings;
    ///   after the main photo is removed, the previous profile photo (if present) becomes the main photo.
    pub fn with_is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }
}

impl Method for RemoveBusinessAccountProfilePhoto {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("removeBusinessAccountProfilePhoto", self)
    }
}

/// Changes the bio of a managed business account.
///
/// Requires the can_change_bio business bot right.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountBio {
    business_connection_id: String,
    bio: Option<String>,
}

impl SetBusinessAccountBio {
    /// Creates a new `SetBusinessAccountBio`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            bio: None,
        }
    }

    /// Sets a new bio
    ///
    /// # Arguments
    ///
    // * `value` - The new value of the bio for the business account; 0-140 characters.
    pub fn with_bio<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.bio = Some(value.into());
        self
    }
}

impl Method for SetBusinessAccountBio {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setBusinessAccountBio", self)
    }
}

/// Changes the privacy settings pertaining to incoming gifts in a managed business account.
///
/// Requires the can_change_gift_settings business bot right.
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountGiftSettings {
    accepted_gift_types: AcceptedGiftTypes,
    business_connection_id: String,
    show_gift_button: bool,
}

impl SetBusinessAccountGiftSettings {
    /// Creates a new `SetBusinessAccountGiftSettings`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `show_gift_button` - Whether a button for sending a gift to the user
    ///   or by the business account must always be shown in the input field
    /// * `accepted_gift_types` - Types of gifts accepted by the business account.
    pub fn new<T>(business_connection_id: T, show_gift_button: bool, accepted_gift_types: AcceptedGiftTypes) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            show_gift_button,
            accepted_gift_types,
        }
    }
}

impl Method for SetBusinessAccountGiftSettings {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setBusinessAccountGiftSettings", self)
    }
}

/// Changes the first and last name of a managed business account.
///
/// Requires the can_change_name business bot right.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountName {
    business_connection_id: String,
    first_name: String,
    last_name: Option<String>,
}

impl SetBusinessAccountName {
    /// Creates a new `SetBusinessAccountName`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `first_name` - The new value of the first name for the business account; 1-64 characters.
    pub fn new<A, B>(business_connection_id: A, first_name: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            first_name: first_name.into(),
            last_name: None,
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value of the last name for the business account; 0-64 characters.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }
}

impl Method for SetBusinessAccountName {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setBusinessAccountName", self)
    }
}

/// Changes the profile photo of a managed business account.
///
/// Requires the can_edit_profile_photo business bot right.
#[derive(Debug)]
pub struct SetBusinessAccountProfilePhoto {
    form: Form,
}

impl SetBusinessAccountProfilePhoto {
    /// Creates a new `SetBusinessAccountProfilePhoto`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `photo` - The new profile photo to set.
    pub fn new<A, B>(business_connection_id: A, photo: B) -> Result<Self, InputProfilePhotoError>
    where
        A: Into<String>,
        B: Into<InputProfilePhoto>,
    {
        let mut form = Form::try_from(photo.into())?;
        form.insert_field("business_connection_id", business_connection_id.into());
        Ok(Self { form })
    }

    /// Sets a new value for the `is_public` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to set the public photo,
    ///   which will be visible even if the main photo is hidden by the business account's privacy settings;
    ///   an account can have only one public photo.
    pub fn with_is_public(mut self, value: bool) -> Self {
        self.form.insert_field("is_public", value);
        self
    }
}

impl Method for SetBusinessAccountProfilePhoto {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("setBusinessAccountProfilePhoto", self.form)
    }
}

/// Changes the username of a managed business account.
///
/// Requires the can_change_username business bot right.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountUsername {
    business_connection_id: String,
    username: Option<String>,
}

impl SetBusinessAccountUsername {
    /// Creates a new `SetBusinessAccountUsername`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            username: None,
        }
    }

    /// Sets a new username
    ///
    /// # Arguments
    ///
    /// * `value` - The new value of the username for the business account; 0-32 characters.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.username = Some(value.into());
        self
    }
}

impl Method for SetBusinessAccountUsername {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setBusinessAccountUsername", self)
    }
}

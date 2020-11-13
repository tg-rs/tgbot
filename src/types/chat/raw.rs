use crate::types::{
    chat::{location::ChatLocation, permissions::ChatPermissions, photo::ChatPhoto},
    message::Message,
    primitive::Integer,
    user::User,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawChat {
    pub id: Integer,
    #[serde(rename = "type")]
    pub kind: RawChatKind,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<Integer>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<Integer>,
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Debug, Deserialize)]
pub struct RawChatMember {
    pub user: User,
    pub status: RawChatMemberStatus,
    pub custom_title: Option<String>,
    pub until_date: Option<Integer>,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub is_member: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawChatMemberStatus {
    Administrator,
    Creator,
    Kicked,
    Left,
    Member,
    Restricted,
}

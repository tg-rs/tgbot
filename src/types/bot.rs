use crate::types::primitive::Integer;
use serde::Deserialize;

/// A Bot info returned in getMe
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct Bot {
    /// Unique identifier of this bot
    pub id: Integer,
    /// Bot's username
    pub username: String,
    /// Bot's first name
    pub first_name: String,
    /// Bot's last name
    pub last_name: Option<String>,
    /// True, if the bot can be invited to groups
    pub can_join_groups: bool,
    /// True, if privacy mode is disabled for the bot
    pub can_read_all_group_messages: bool,
    /// True, if the bot supports inline queries
    pub supports_inline_queries: bool,
}

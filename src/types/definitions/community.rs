use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// Represents a community (a group of chats).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Community {
    /// Unique identifier for this community.
    pub id: Integer,
    /// Name of the community.
    pub name: String,
}

/// Describes a service message about a chat being added to a community.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommunityChatAdded {
    /// The new community to which the chat belongs
    pub community: Community,
}

/// Describes a service message about a chat being removed from a community.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommunityChatRemoved {}

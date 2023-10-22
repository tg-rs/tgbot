use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Represents a message about a forwarded story in the chat
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Story {}

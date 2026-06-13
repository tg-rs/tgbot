use serde::{Deserialize, Serialize};

use crate::types::InputRichMessage;

/// Represents the content of a rich message.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentRich {
    rich_message: InputRichMessage,
}

impl From<InputRichMessage> for InputMessageContentRich {
    fn from(value: InputRichMessage) -> Self {
        Self { rich_message: value }
    }
}

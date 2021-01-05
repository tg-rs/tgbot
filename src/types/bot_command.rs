use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

const MIN_NAME_LEN: usize = 1;
const MAX_NAME_LEN: usize = 32;
const MIN_DESCRIPTION_LEN: usize = 3;
const MAX_DESCRIPTION_LEN: usize = 256;

/// This object represents a bot command
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BotCommand {
    command: String,
    description: String,
}

impl BotCommand {
    /// Creates a new BotCommand
    ///
    /// # Arguments
    ///
    /// * name - Name of the command, 1-32 characters
    ///          Can contain only lowercase English letters, digits and underscores
    /// * description - Description of the command, 3-256 characters
    pub fn new<C, D>(name: C, description: D) -> Result<Self, BotCommandError>
    where
        C: Into<String>,
        D: Into<String>,
    {
        let name = name.into();
        let description = description.into();
        let name_len = name.len();
        let description_len = description.len();
        if !(MIN_NAME_LEN..=MAX_NAME_LEN).contains(&name_len) {
            Err(BotCommandError::BadNameLen(name_len))
        } else if !(MIN_DESCRIPTION_LEN..=MAX_DESCRIPTION_LEN).contains(&description_len) {
            Err(BotCommandError::BadDescriptionLen(description_len))
        } else {
            Ok(Self {
                command: name,
                description,
            })
        }
    }

    /// Returns the command name
    pub fn name(&self) -> &str {
        &self.command
    }

    /// Returns the command description
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// An error when creating a new BotCommand
#[derive(Debug)]
pub enum BotCommandError {
    /// Got a name with invalid length
    BadNameLen(usize),
    /// Got a description with invalid length
    BadDescriptionLen(usize),
}

impl Error for BotCommandError {}

impl fmt::Display for BotCommandError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::BotCommandError::*;
        match self {
            BadNameLen(len) => write!(
                out,
                "command name can have a length of {} up to {} characters, got {}",
                MIN_NAME_LEN, MAX_NAME_LEN, len
            ),
            BadDescriptionLen(len) => write!(
                out,
                "command description can have a length of {} up to {} characters, got {}",
                MIN_DESCRIPTION_LEN, MAX_DESCRIPTION_LEN, len
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bot_command() {
        let err = BotCommand::new("", "description").unwrap_err().to_string();
        assert_eq!(err, "command name can have a length of 1 up to 32 characters, got 0");
        let err = BotCommand::new("2".repeat(33), "description").unwrap_err().to_string();
        assert_eq!(err, "command name can have a length of 1 up to 32 characters, got 33");
        let err = BotCommand::new("name", "d").unwrap_err().to_string();
        assert_eq!(
            err,
            "command description can have a length of 3 up to 256 characters, got 1"
        );
        let err = BotCommand::new("name", "d".repeat(257)).unwrap_err().to_string();
        assert_eq!(
            err,
            "command description can have a length of 3 up to 256 characters, got 257"
        );
    }
}

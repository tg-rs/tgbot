use std::{convert::TryFrom, error::Error, fmt, string::FromUtf16Error};

use shellwords::MismatchedQuotes;

use crate::types::Message;

#[cfg(test)]
mod tests;

/// A simple command implementation
///
/// We just take first command from a message and ignore others.
/// Also we assume that entire text after command is arguments separated by whitespace.
/// In order to include space in argument you need to wrap it with `'`: `'arg1 v' arg2`.
///
/// # Example
/// ```
/// use tgbot::types::{Command, Message};
/// use std::convert::TryFrom;
///
/// fn handle_command(message: Message) {
///     let command = Command::try_from(message).unwrap();
///     println!("NAME: {}", command.get_name());
///     println!("ARGUMENTS: {:?}", command.get_args());
///     println!("MESSAGE: {:?}", command.get_message());
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
    message: Message,
}

impl Command {
    /// Returns command name with leading slash
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a list of arguments
    pub fn get_args(&self) -> &[String] {
        &self.args
    }

    /// Returns a message where command comes from
    pub fn get_message(&self) -> &Message {
        &self.message
    }
}

/// An error when parsing command
#[derive(Debug)]
pub enum CommandError {
    /// Command not found in a message
    NotFound,
    /// Failed to create an UTF-16 string when reading command from a message
    Utf16(FromUtf16Error),
    /// An error when splitting a string with mismatched quotes
    MismatchedQuotes(MismatchedQuotes),
}

impl From<FromUtf16Error> for CommandError {
    fn from(err: FromUtf16Error) -> Self {
        Self::Utf16(err)
    }
}

impl From<MismatchedQuotes> for CommandError {
    fn from(err: MismatchedQuotes) -> Self {
        Self::MismatchedQuotes(err)
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandError::NotFound => None,
            CommandError::Utf16(err) => Some(err),
            CommandError::MismatchedQuotes(_) => None,
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "failed to parse command: {}",
            match self {
                CommandError::NotFound => String::from("not found"),
                CommandError::Utf16(err) => err.to_string(),
                CommandError::MismatchedQuotes(_) => String::from("mismatched quotes"),
            }
        )
    }
}

impl TryFrom<Message> for Command {
    type Error = CommandError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_text().map(|text| (text.get_bot_commands(), text)) {
            Some((Some(commands), text)) => {
                // just take first command and ignore others
                let command = &commands[0];
                let name = command.command.clone();
                // assume that all text after command is arguments
                let offset = text.data.find(&name).unwrap_or(0);
                // bot suffix is 1 character longer due to '@' symbol
                let length = name.len() + command.bot_name.as_ref().map(|x| x.len() + 1).unwrap_or(0);
                let pos = offset + length;
                // pos is UTF-16 offset
                let raw_args: Vec<u16> = text.data.encode_utf16().skip(pos).collect();
                let raw_args = String::from_utf16(&raw_args)?;
                let args = shellwords::split(&raw_args)?;
                Ok(Command { name, args, message })
            }
            _ => Err(CommandError::NotFound),
        }
    }
}

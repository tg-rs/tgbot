use crate::{methods::Method, request::Request, types::BotCommand};
use serde::Serialize;

/// Use this method to get the current list of the bot's commands
#[derive(Clone, Copy, Debug)]
pub struct GetMyCommands;

impl Method for GetMyCommands {
    type Response = Vec<BotCommand>;

    fn into_request(self) -> Request {
        Request::empty("getMyCommands")
    }
}

/// Use this method to change the list of the bot's commands
#[derive(Clone, Debug, Serialize)]
pub struct SetMyCommands {
    commands: Vec<BotCommand>,
}

impl SetMyCommands {
    /// Creates a new SetMyCommands
    ///
    /// # Arguments
    ///
    /// * commands - Commands to set
    pub fn new(commands: impl IntoIterator<Item = BotCommand>) -> Self {
        Self {
            commands: commands.into_iter().collect(),
        }
    }
}

impl Method for SetMyCommands {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setMyCommands", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_bot_commands() {
        let request = GetMyCommands.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getMyCommands"
        );
        assert!(matches!(request.into_body(), RequestBody::Empty));
    }

    #[test]
    fn set_bot_commands() {
        let request = SetMyCommands::new(vec![BotCommand::new("name", "description").unwrap()]).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(
                data.unwrap(),
                r#"{"commands":[{"command":"name","description":"description"}]}"#
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

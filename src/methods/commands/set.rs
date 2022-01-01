use crate::{
    methods::Method,
    request::Request,
    types::{BotCommand, BotCommandScope},
};
use serde::Serialize;

/// Use this method to change the list of the bot's commands
#[derive(Clone, Debug, Serialize)]
pub struct SetMyCommands {
    commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
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
            scope: None,
            language_code: None,
        }
    }

    /// Sets a scope of users for which the commands are relevant
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code
    ///
    /// If empty, commands will be applied to all users from the given scope,
    /// for whose language there are no dedicated commands
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
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
    use serde_json::Value as JsonValue;

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

        let request = SetMyCommands::new(vec![BotCommand::new("name", "description").unwrap()])
            .scope(BotCommandScope::AllPrivateChats)
            .language_code("ru")
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(
                serde_json::from_str::<JsonValue>(&data.unwrap()).unwrap(),
                serde_json::json!({
                    "commands": [
                        {
                            "command": "name",
                            "description": "description"
                        }
                    ],
                    "scope": {
                        "type": "all_private_chats"
                    },
                    "language_code": "ru"
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

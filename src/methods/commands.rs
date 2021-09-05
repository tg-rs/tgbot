use crate::{
    methods::Method,
    request::Request,
    types::{BotCommand, BotCommandScope},
};
use serde::Serialize;

/// Use this method to get the current list of the bot's commands
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl GetMyCommands {
    /// An object, describing scope of users
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code or an empty string
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for GetMyCommands {
    type Response = Vec<BotCommand>;

    fn into_request(self) -> Request {
        Request::json("getMyCommands", self)
    }
}

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

/// Use this method to delete the list of the bot's commands for the given scope and user language
///
///  After deletion, higher level commands will be shown to affected users
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl DeleteMyCommands {
    /// An object, describing scope of users
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code or an empty string
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for DeleteMyCommands {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("deleteMyCommands", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value as JsonValue;

    #[test]
    fn get_bot_commands() {
        let request = GetMyCommands::default().into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{}"#)
        } else {
            panic!("Unexpected request body");
        }

        let request = GetMyCommands::default()
            .scope(BotCommandScope::Default)
            .language_code("ru")
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{"scope":{"type":"default"},"language_code":"ru"}"#)
        } else {
            panic!("Unexpected request body");
        }
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

    #[test]
    fn delete_bot_commands() {
        let request = DeleteMyCommands::default().into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{}"#)
        } else {
            panic!("Unexpected request body");
        }

        let request = DeleteMyCommands::default()
            .scope(BotCommandScope::Default)
            .language_code("ru")
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteMyCommands"
        );
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{"scope":{"type":"default"},"language_code":"ru"}"#)
        } else {
            panic!("Unexpected request body");
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

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
}

use crate::{methods::Method, request::Request, types::BotCommandScope};
use serde::Serialize;

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

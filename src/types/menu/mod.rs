use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, WebAppInfo},
};

#[cfg(test)]
mod tests;

/// Represents a menu button of the bot in a private chat.
///
/// If a menu button other than default is set for a private chat, then it is applied in the chat.
/// Otherwise the default menu button is applied.
/// By default, the menu button opens the list of bot commands.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "MenuButtonRaw", into = "MenuButtonRaw")]
pub enum MenuButton {
    /// Opens the list of bot commands.
    Commands,
    /// Default behaviour.
    Default,
    /// Launches a web app.
    WebApp(MenuButtonWebApp),
}

/// Represents a menu button, which launches a Web App.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MenuButtonWebApp {
    /// Text on the button.
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button.
    ///
    /// The Web App will be able to send an arbitrary message on behalf of the user
    /// using the method `answerWebAppQuery`.
    pub web_app: WebAppInfo,
}

impl MenuButtonWebApp {
    /// Creates a new `MenuButtonWebApp`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `web_app` - Web app to launch.
    pub fn new<T>(text: T, web_app: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            web_app,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum MenuButtonRaw {
    Commands {},
    Default {},
    WebApp(MenuButtonWebApp),
}

impl From<MenuButtonRaw> for MenuButton {
    fn from(value: MenuButtonRaw) -> Self {
        match value {
            MenuButtonRaw::Commands {} => Self::Commands,
            MenuButtonRaw::Default {} => Self::Default,
            MenuButtonRaw::WebApp(value) => Self::WebApp(value),
        }
    }
}

impl From<MenuButton> for MenuButtonRaw {
    fn from(value: MenuButton) -> Self {
        match value {
            MenuButton::Commands => Self::Commands {},
            MenuButton::Default => Self::Default {},
            MenuButton::WebApp(value) => Self::WebApp(value),
        }
    }
}

/// Returns the current value of the bot menu button in a private chat, or the default menu button.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
}

impl GetChatMenuButton {
    /// Sets a new chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target private chat.
    ///
    /// If not specified, default bot menu button will be returned.
    pub fn with_chat_id(mut self, value: Integer) -> Self {
        self.chat_id = Some(value);
        self
    }
}

impl Method for GetChatMenuButton {
    type Response = MenuButton;

    fn into_payload(self) -> Payload {
        Payload::json("getChatMenuButton", self)
    }
}

/// Changes a button of a menu of a bot in a private chat, or a default menu button.
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<MenuButton>,
}

impl SetChatMenuButton {
    /// Sets a new chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target private chat.
    ///
    /// If not specified, default bot menu button will be changed.
    pub fn chat_id(mut self, chat_id: Integer) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    /// Sets a new menu button.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for the new bot menu button; default - [`MenuButton::Default`].
    pub fn menu_button(mut self, value: MenuButton) -> Self {
        self.menu_button = Some(value);
        self
    }
}

impl Method for SetChatMenuButton {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatMenuButton", self)
    }
}

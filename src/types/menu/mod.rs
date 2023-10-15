use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, WebAppInfo},
};

#[cfg(test)]
mod tests;

/// Describes the bot menu button in a private chat
///
/// If a menu button other than [`MenuButtonDefault`] is set for a private chat,
/// then it is applied in the chat.
/// Otherwise the default menu button is applied.
/// By default, the menu button opens the list of bot commands.
///
/// [`MenuButtonDefault`]: MenuButtonDefault
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MenuButton {
    /// Represents a menu button, which opens the list of bot commands
    Commands(MenuButtonCommands),
    /// Describes that no specific value for the menu button was set
    Default(MenuButtonDefault),
    /// Represents a menu button, which launches a Web App
    WebApp(MenuButtonWebApp),
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::Default(MenuButtonDefault {})
    }
}

impl MenuButton {
    /// Creates a new commands menu button
    pub fn commands() -> Self {
        Self::Commands(MenuButtonCommands {})
    }
}

/// Represents a menu button, which opens the list of bot commands
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MenuButtonCommands {}

/// Describes that no specific value for the menu button was set
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MenuButtonDefault {}

/// Represents a menu button, which launches a Web App
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button
    ///
    /// The Web App will be able to send an arbitrary message on behalf of the user
    /// using the method answerWebAppQuery.
    pub web_app: WebAppInfo,
}

/// Get the current value of the bot menu button in a private chat, or the default menu button
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
}

impl GetChatMenuButton {
    /// Unique identifier for the target private chat
    ///
    /// If not specified, default bot menu button will be returned
    pub fn chat_id(mut self, chat_id: Integer) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
}

impl Method for GetChatMenuButton {
    type Response = MenuButton;

    fn into_payload(self) -> Payload {
        Payload::json("getChatMenuButton", self)
    }
}

/// Change the bot menu button in a private chat, or the default menu button.
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<MenuButton>,
}

impl SetChatMenuButton {
    /// Unique identifier for the target private chat
    ///
    /// If not specified, default bot menu button will be changed
    pub fn chat_id(mut self, chat_id: Integer) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    /// An object for the new bot menu button
    ///
    /// Defaults to [`MenuButtonDefault`]
    ///
    /// [`MenuButtonDefault`]: MenuButtonDefault
    pub fn menu_button(mut self, menu_button: MenuButton) -> Self {
        self.menu_button = Some(menu_button);
        self
    }
}

impl Method for SetChatMenuButton {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatMenuButton", self)
    }
}

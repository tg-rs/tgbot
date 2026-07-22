//! The example shows how to use inline keyboards
use std::env;

use dotenvy::dotenv;
use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{AnswerCallbackQuery, InlineKeyboardButton, Message, SendMessage, Update, UpdateType},
};

struct Handler {
    client: Client,
}

async fn handle_update(client: &Client, update: Update) -> Option<Message> {
    match update.update_type {
        UpdateType::Message(message) => {
            let chat_id = message.chat.get_id();
            if let Some(commands) = message.get_text().and_then(|text| text.get_bot_commands()) {
                let command = &commands[0];
                if command.command == "/start" {
                    let method = SendMessage::new(chat_id, "Press the button").with_reply_markup([[
                        // You also can use with_callback_data to pass a regular string
                        InlineKeyboardButton::for_callback_data("Greet", "Hello!"),
                    ]]);
                    return Some(client.execute(method).await.unwrap());
                }
            }
        }
        UpdateType::CallbackQuery(query) => {
            let data = query.data.unwrap_or_else(|| String::from("???"));
            let method = AnswerCallbackQuery::new(query.id).with_text(data);
            client.execute(method).await.unwrap();
        }
        _ => {}
    }
    None
}

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("Got an update: {update:?}");
        if let Some(msg) = handle_update(&self.client, update).await {
            log::info!("Message sent: {msg:?}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(client.clone(), Handler { client }).run().await;
}

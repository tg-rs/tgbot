//! The example shows how to use inline keyboards
use std::env;

use dotenvy::dotenv;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{AnswerCallbackQuery, InlineKeyboardButton, Message, SendMessage, Update, UpdateType},
};

struct Handler {
    client: Client,
}

#[derive(Deserialize, Serialize)]
struct CallbackData {
    value: String,
}

impl CallbackData {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: value.into() }
    }
}

async fn handle_update(client: &Client, update: Update) -> Option<Message> {
    match update.update_type {
        UpdateType::Message(message) => {
            let chat_id = message.chat.get_id();
            if let Some(commands) = message.get_text().and_then(|text| text.get_bot_commands()) {
                let command = &commands[0];
                if command.command == "/start" {
                    let callback_data = CallbackData::new("Hello!");
                    let method = SendMessage::new(chat_id, "Press the button").with_reply_markup([[
                        // You also can use with_callback_data to pass a regular string
                        InlineKeyboardButton::for_callback_data_struct("Greet", &callback_data).unwrap(),
                    ]]);
                    return Some(client.execute(method).await.unwrap());
                }
            }
        }
        UpdateType::CallbackQuery(query) => {
            let data = query.parse_data::<CallbackData>().unwrap().unwrap(); // or query.data if you have passed a plain string
            let method = AnswerCallbackQuery::new(query.id).with_text(data.value);
            client.execute(method).await.unwrap();
        }
        _ => {}
    }
    None
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            if let Some(msg) = handle_update(&client, update).await {
                log::info!("Message sent: {:?}", msg);
            }
        })
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

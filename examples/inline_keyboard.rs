use dotenv::dotenv;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{InlineKeyboardButton, Message, Update, UpdateKind},
    Api, UpdateHandler,
};

struct Handler {
    api: Api,
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

async fn handle_update(api: &Api, update: Update) -> Option<Message> {
    match update.kind {
        UpdateKind::Message(message) => {
            let chat_id = message.get_chat_id();
            if let Some(commands) = message.get_text().and_then(|text| text.get_bot_commands()) {
                let command = &commands[0];
                if command.command == "/start" {
                    let callback_data = CallbackData::new("hello!");
                    let method = SendMessage::new(chat_id, "keyboard example").reply_markup(vec![vec![
                        // You also can use with_callback_data in order to pass a plain string
                        InlineKeyboardButton::with_callback_data_struct("button", &callback_data).unwrap(),
                    ]]);
                    return Some(api.execute(method).await.unwrap());
                }
            }
        }
        UpdateKind::CallbackQuery(query) => {
            if let Some(ref message) = query.message {
                let chat_id = message.get_chat_id();
                // or query.data if you have passed a plain string
                let data = query.parse_data::<CallbackData>().unwrap().unwrap();
                let method = SendMessage::new(chat_id, data.value);
                return Some(api.execute(method).await.unwrap());
            }
        }
        _ => {}
    }
    None
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let api = self.api.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            if let Some(msg) = handle_update(&api, update).await {
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
    let api = Api::new(token).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}

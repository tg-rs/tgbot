//! The example shows how to use long polling
use std::env;

use dotenvy::dotenv;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{SendMessage, Update},
};

struct Handler {
    client: Client,
}

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("Got an update: {:?}", update);
        let chat_id = update.get_chat_id();
        let message = update.get_message();
        let text = message.and_then(|x| x.get_text());
        if let (Some(chat_id), Some(text)) = (chat_id, text) {
            let method = SendMessage::new(chat_id, text.data.clone());
            self.client.execute(method).await.unwrap();
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

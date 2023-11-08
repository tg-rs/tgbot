use std::env;

use dotenv::dotenv;
use futures_util::future::BoxFuture;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{SendMessage, Update, UpdateType},
};

struct Handler {
    client: Client,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let UpdateType::Message(message) = update.update_type {
                if let Some(text) = message.get_text() {
                    let chat_id = message.chat.get_id();
                    let method = SendMessage::new(chat_id, text.data.clone());
                    client.execute(method).await.unwrap();
                }
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

use std::env;

use dotenv::dotenv;
use futures_util::future::BoxFuture;

use tgbot::{
    longpoll::LongPoll,
    types::{SendMessage, Update, UpdateKind},
    Api,
    UpdateHandler,
};

struct Handler {
    api: Api,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let api = self.api.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let UpdateKind::Message(message) = update.kind {
                if let Some(text) = message.get_text() {
                    let chat_id = message.chat.get_id();
                    let method = SendMessage::new(chat_id, text.data.clone());
                    api.execute(method).await.unwrap();
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
    let api = Api::new(token).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}

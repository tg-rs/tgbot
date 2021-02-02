use async_trait::async_trait;
use dotenv::dotenv;
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{Update, UpdateKind},
    Api, Config, UpdateHandler,
};

struct Handler {
    api: Api,
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.get_text() {
                let chat_id = message.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                self.api.execute(method).await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}

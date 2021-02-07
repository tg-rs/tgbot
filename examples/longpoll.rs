use dotenv::dotenv;
use futures_util::future::BoxFuture;
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

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let api = self.api.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let UpdateKind::Message(message) = update.kind {
                if let Some(text) = message.get_text() {
                    let chat_id = message.get_chat_id();
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
    let proxy = env::var("TGBOT_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}

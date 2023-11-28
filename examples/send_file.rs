//! The example shows how to send a file
use std::env;

use dotenvy::dotenv;
use futures_util::future::BoxFuture;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{InputFile, SendDocument, Update},
};

#[derive(Clone)]
struct Handler {
    client: Client,
    file_url: String,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            let chat_id = update.get_chat_id().unwrap();
            this.client
                .execute(SendDocument::new(chat_id, InputFile::url(this.file_url)))
                .await
                .unwrap();
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let file_url = env::var("TGBOT_FILE_URL").expect("TGBOT_FILE_URL is not set");
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(client.clone(), Handler { client, file_url }).run().await;
}

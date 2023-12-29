//! The example shows how to send a file
use std::env;

use dotenvy::dotenv;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{InputFile, SendDocument, Update},
};

struct Handler {
    client: Client,
    file_url: String,
}

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("Got an update: {:?}", update);
        let chat_id = update.get_chat_id().unwrap();
        self.client
            .execute(SendDocument::new(chat_id, InputFile::url(&self.file_url)))
            .await
            .unwrap();
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

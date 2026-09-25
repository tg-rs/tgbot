//! The example shows how to send a file
use std::env;

use dotenvy::dotenv;
use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{InputFile, SendDocument, Update},
};

enum FileSource {
    Path(String),
    Url(String),
}

struct Handler {
    client: Client,
    file_source: FileSource,
}

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("Got an update: {update:?}");
        let chat_id = update.get_chat_id().unwrap();
        let f = match &self.file_source {
            FileSource::Path(path) => InputFile::path(path).await.unwrap(),
            FileSource::Url(url) => InputFile::url(url),
        };
        self.client.execute(SendDocument::new(chat_id, f)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let file_source = match env::var("TGBOT_FILE_URL") {
        Ok(value) => FileSource::Url(value),
        Err(_) => {
            let value = env::var("TGBOT_FILE_PATH").expect("TGBOT_FILE_URL or TGBOT_FILE_PATH must be set");
            FileSource::Path(value)
        }
    };
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(client.clone(), Handler { client, file_source })
        .run()
        .await;
}

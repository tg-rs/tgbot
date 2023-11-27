//! The example shows how to download a file
use std::{
    env,
    path::{Path, PathBuf},
};

use dotenv::dotenv;
use futures_util::{future::BoxFuture, stream::StreamExt};
use tempfile::tempdir;
use tokio::{fs::File, io::AsyncWriteExt};

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{Document, GetFile, MessageData, Update, UpdateType},
};

#[derive(Clone)]
struct Handler {
    client: Client,
    tmpdir: PathBuf,
}

async fn handle_document(client: &Client, tmpdir: &Path, document: Document) {
    log::info!(
        "Got a document (Name: {:?}, MIME-Type: {:?}, Size: {:?})",
        document.file_name,
        document.mime_type,
        document.file_size
    );

    // Get a path of the file
    let target_path = tmpdir.join(document.file_name.clone().unwrap_or_else(|| String::from("unknown")));
    let file = client.execute(GetFile::new(document.file_id.as_str())).await.unwrap();
    let source_path = file.file_path.unwrap();

    log::info!("Downloading a document from {:?}", source_path);

    let mut stream = client.download_file(source_path).await.unwrap();
    let mut file = File::create(target_path.clone()).await.unwrap();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        file.write_all(&chunk).await.unwrap();
    }
    log::info!("The document saved to {:?}", target_path);
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            if let UpdateType::Message(message) = update.update_type {
                if let MessageData::Document(document) = message.data {
                    handle_document(&this.client, &this.tmpdir, document.data).await;
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
    let tmpdir = tempdir().expect("Failed to create temporary directory");
    LongPoll::new(
        client.clone(),
        Handler {
            client,
            tmpdir: tmpdir.path().to_path_buf(),
        },
    )
    .run()
    .await;
}

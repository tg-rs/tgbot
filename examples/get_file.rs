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
    types::{Document, GetFile, MessageData, Update, UpdateKind},
};

#[derive(Clone)]
struct Handler {
    client: Client,
    tmpdir: PathBuf,
}

async fn handle_document(client: &Client, tmpdir: &Path, document: Document) {
    let path = tmpdir.join(document.file_name.clone().unwrap_or_else(|| String::from("unknown")));
    let file = client.execute(GetFile::new(document.file_id.as_str())).await.unwrap();
    let file_path = file.file_path.unwrap();
    let mut stream = client.download_file(file_path).await.unwrap();
    println!("Name: {:?}", document.file_name);
    println!("Mime-Type: {:?}", document.mime_type);
    println!("Document size: {:?}", document.file_size);
    let mut file = File::create(path).await.unwrap();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        file.write_all(&chunk).await.unwrap();
    }
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let UpdateKind::Message(message) = update.kind {
                if let MessageData::Document { data, .. } = message.data {
                    handle_document(&this.client, &this.tmpdir, data).await;
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
    log::info!("Temp dir: {}", tmpdir.path().display());
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

//! The example shows how to handle updates using a webhook
use dotenvy::dotenv;
use tgbot::{
    handler::{UpdateHandler, WebhookServer},
    types::Update,
};

struct Handler;

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("Got an update: {update:?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    WebhookServer::new("/", Handler)
        .run(([127, 0, 0, 1], 8080))
        .await
        .unwrap();
}

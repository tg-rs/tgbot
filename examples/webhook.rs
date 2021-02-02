use async_trait::async_trait;
use dotenv::dotenv;
use tgbot::{types::Update, webhook, UpdateHandler};

struct Handler;

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        log::info!("got an update: {:?}\n", update);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}

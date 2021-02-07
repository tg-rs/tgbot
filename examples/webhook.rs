use dotenv::dotenv;
use futures_util::future::BoxFuture;
use tgbot::{types::Update, webhook, UpdateHandler};

struct Handler;

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}

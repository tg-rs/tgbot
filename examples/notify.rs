//! The example shows how to schedule a message
use std::{env, time::Duration};

use dotenvy::dotenv;
use tokio::{spawn, sync::mpsc, time::sleep};

use tgbot::{
    api::Client,
    types::{ChatId, Integer, SendMessage, Update},
};

#[allow(clippy::large_enum_variant)]
enum Notification {
    Hello,
    #[allow(dead_code)]
    Update(Update),
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let chat_id = env::var("TGBOT_NOTIFICATION_CHAT_ID").expect("TGBOT_NOTIFICATION_CHAT_ID is not set");
    let chat_id: ChatId = match chat_id.parse::<Integer>() {
        Ok(chat_id) => chat_id.into(),
        Err(_) => chat_id.into(),
    };
    let client = Client::new(token).expect("Failed to create API");

    let (tx, mut rx) = mpsc::channel(100);

    // Spawn a scheduler task
    spawn(async move {
        let timeout = Duration::from_secs(1);
        for _ in 0..10usize {
            if tx.send(Notification::Hello).await.is_err() {
                log::error!("Receiver dropped");
                return;
            }
            sleep(timeout).await;
        }
    });

    // Handle notifications here
    while let Some(notification) = rx.recv().await {
        match notification {
            Notification::Update(_update) => {
                // Handle the update from Telegram here
                unimplemented!()
            }
            Notification::Hello => {
                client
                    .execute(SendMessage::new(chat_id.clone(), "Hello!"))
                    .await
                    .unwrap();
            }
        }
    }
}

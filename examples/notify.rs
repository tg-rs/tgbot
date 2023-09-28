use std::{env, time::Duration};

use dotenv::dotenv;
use tokio::{spawn, sync::mpsc, time::sleep};

use tgbot::{
    types::{ChatId, Integer, SendMessage, Update},
    Api,
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
    let chat_id = match chat_id.parse::<Integer>() {
        Ok(chat_id) => ChatId::Id(chat_id),
        Err(_) => ChatId::Username(chat_id),
    };
    let api = Api::new(token).expect("Failed to create API");

    let (tx, mut rx) = mpsc::channel(100);

    spawn(async move {
        let timeout = Duration::from_secs(1);
        for _ in 0..10usize {
            if tx.send(Notification::Hello).await.is_err() {
                println!("Receiver dropped");
                return;
            }
            sleep(timeout).await;
        }
    });

    while let Some(notification) = rx.recv().await {
        match notification {
            Notification::Update(_update) => {
                // you can handle update from telegram here
                unimplemented!()
            }
            Notification::Hello => {
                api.execute(SendMessage::new(chat_id.clone(), "Hello!")).await.unwrap();
            }
        }
    }
}

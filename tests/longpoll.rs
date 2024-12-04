#![allow(missing_docs)]
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use dotenvy::dotenv;
use mockito::{Matcher, Server};
use serde_json::json;
use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::Update,
};
use tokio::{spawn, sync::Mutex, time::sleep};

struct Handler {
    updates: Arc<Mutex<Vec<Update>>>,
}

impl UpdateHandler for Handler {
    async fn handle(&self, update: Update) {
        let mut updates = self.updates.lock().await;
        updates.push(update);
    }
}

#[tokio::test]
async fn longpoll() {
    dotenv().ok();
    env_logger::init();
    let mut server = Server::new_async().await;
    server
        .mock("POST", "/bot-token/getUpdates")
        .match_body(Matcher::PartialJson(json!({
            "limit": 100,
            "timeout": 10,
            "allowed_updates": []
        })))
        .with_body(
            serde_json::to_vec(&json!({
                "ok": true,
                "result": [
                    {
                        "update_id": 1,
                        "message": {
                            "message_id": 1,
                            "date": 0,
                            "from": {
                                "id": 1,
                                "is_bot": false,
                                "first_name": "test"
                            },
                            "chat": {
                                "id": 1,
                                "type": "private",
                                "first_name": "test"
                            },
                            "text": "test"
                        }
                    },
                    {
                        "update_id": 1,
                        "unknown": {
                            "description": "unknown updates should be parsed",
                        }
                    }
                ]
            }))
            .unwrap(),
        )
        .create();
    let client = Client::new("-token").unwrap().with_host(server.url());
    let updates = Arc::new(Mutex::new(Vec::new()));
    let handler = Handler {
        updates: updates.clone(),
    };
    let poll = LongPoll::new(client, handler);
    let handle = poll.get_handle();
    let wait_updates = updates.clone();
    spawn(async move {
        let now = Instant::now();
        while wait_updates.lock().await.is_empty() {
            if now.elapsed().as_secs() >= 2 {
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
        handle.shutdown().await
    });
    poll.run().await;
    assert!(!updates.lock().await.is_empty())
}

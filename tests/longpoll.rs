use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use dotenv::dotenv;
use futures_util::future::BoxFuture;
use mockito::{mock, server_url, Matcher};
use serde_json::json;
use tokio::{spawn, sync::Mutex, time::sleep};

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::Update,
};

struct Handler {
    updates: Arc<Mutex<Vec<Update>>>,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let updates = self.updates.clone();
        Box::pin(async move {
            let mut updates = updates.lock().await;
            updates.push(update);
        })
    }
}

#[tokio::test]
async fn longpoll() {
    dotenv().ok();
    env_logger::init();
    let _m = mock("POST", "/bot-token/getUpdates")
        .match_body(Matcher::PartialJson(json!({
            "limit": 100,
            "timeout": 10,
            "allowed_updates": []
        })))
        .with_body(
            &serde_json::to_vec(&json!({
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
    let client = Client::new("-token").unwrap().with_host(server_url());
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

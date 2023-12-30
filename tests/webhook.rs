use std::sync::Arc;

use reqwest::{Client, StatusCode};
use tgbot::{
    handler::{UpdateHandler, WebhookServer},
    types::Update,
};
use tokio::sync::Mutex;

#[derive(Clone)]
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
async fn webhook() {
    let updates = Arc::new(Mutex::new(Vec::new()));
    let webhook_server = WebhookServer::new(
        "/",
        Handler {
            updates: updates.clone(),
        },
    );
    tokio::spawn(async move {
        webhook_server.run(([127, 0, 0, 1], 8080)).await.unwrap();
    });
    let client = Client::new();
    let response = client
        .post("http://localhost:8080/")
        .json(&serde_json::json!({
            "update_id":10000,
            "message":{
                "date":1441645532,
                "chat":{
                    "last_name":"Test Lastname",
                    "id":1111111,
                    "first_name":"Test",
                    "username":"Test",
                    "type": "private"
                },
                "message_id":1365,
                "from":{
                    "last_name":"Test Lastname",
                    "id":1111111,
                    "first_name":"Test",
                    "username":"Test",
                    "is_bot": false
                },
                "text":"/start"
            }
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!updates.lock().await.is_empty())
}

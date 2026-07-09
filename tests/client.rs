#![allow(missing_docs)]
use std::{sync::Mutex, time::Instant};

use futures_util::stream::StreamExt;
use mockito::{Server, ServerGuard};
use tgbot::{
    api::{Client, ExecuteError},
    types::Close,
};

struct Cx {
    server: ServerGuard,
    client: Client,
}

impl Cx {
    async fn new_async() -> Self {
        let server = Server::new_async().await;
        let host = server.url();
        Self {
            server,
            client: Client::new("-token").unwrap().with_host(host).with_max_retry_after(0),
        }
    }

    async fn execute_close(&self) -> Result<bool, ExecuteError> {
        self.client.execute(Close).await
    }

    async fn execute_close_without_retries(&self) -> Result<bool, ExecuteError> {
        self.client.clone().with_max_retries(0).execute(Close).await
    }

    fn set_close_response(&mut self, body: &str) {
        self.server.mock("GET", "/bot-token/close").with_body(body).create();
    }

    fn set_close_response_retry(&mut self, retry_after: usize) {
        let counter = Mutex::new(0u8);
        self.server
            .mock("GET", "/bot-token/close")
            .with_body_from_request(move |_| {
                let mut val = counter.lock().unwrap();
                if *val < 2 {
                    *val += 1;
                    serde_json::to_string(&serde_json::json!({
                        "ok": false,
                        "description": format!("test {val}"),
                        "parameters": {"retry_after": retry_after}
                    }))
                    .unwrap()
                    .into()
                } else {
                    *val = 0;
                    r#"{"ok": true, "result": true}"#.into()
                }
            })
            .create();
    }
}

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new_async().await;
    cx.set_close_response(r#"{"ok": true, "result": true}"#);
    cx.execute_close().await.unwrap();

    cx.set_close_response(r#"{"ok": false, "description": "test"}"#);
    let err = cx.execute_close().await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to execute method: a telegram error has occurred: description=test"
    );

    cx.set_close_response(r#"invalid-data"#);
    let err = cx.execute_close().await.unwrap_err();
    assert!(
        err.to_string()
            .starts_with("failed to execute method: error decoding response body"),
    );

    cx.set_close_response_retry(0);
    cx.execute_close().await.unwrap();
    let err = cx.execute_close_without_retries().await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to execute method: a telegram error has occurred: description=test 1; retry_after=0"
    );
    cx.execute_close().await.unwrap();

    cx.set_close_response_retry(10000000000000000);
    let start = Instant::now();
    cx.execute_close().await.unwrap();
    let end = Instant::now();
    let duration = end - start;
    assert!(duration.as_secs() <= 1)
}

#[tokio::test]
async fn download_file() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/file/bot-token/file-ok")
        .with_body(b"file-data")
        .create();
    server
        .mock("GET", "/file/bot-token/file-err")
        .with_body("test-error")
        .with_status(400)
        .create();
    let client = Client::new("-token").unwrap().with_host(server.url());
    let mut stream = client.download_file("file-ok").await.unwrap();
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        buf.extend(chunk);
    }
    assert_eq!(&buf[..], b"file-data");

    match client.download_file("file-err").await {
        Ok(_) => panic!("Got an unexpected stream"),
        Err(err) => assert_eq!(err.to_string(), "failed to download file: status=400 text=test-error"),
    };
}

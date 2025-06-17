#![allow(missing_docs)]
use std::sync::Mutex;

use futures_util::stream::StreamExt;
use mockito::Server;
use tgbot::{api::Client, types::Close};

#[tokio::test]
async fn execute() {
    let mut server = Server::new_async().await;
    let client = Client::new("-token").unwrap().with_host(server.url());

    server
        .mock("GET", "/bot-token/close")
        .with_body(r#"{"ok": true, "result": true}"#)
        .create();
    client.execute(Close).await.unwrap();

    server
        .mock("GET", "/bot-token/close")
        .with_body(r#"{"ok": false, "description": "test"}"#)
        .create();
    let err = client.execute(Close).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to execute method: a telegram error has occurred: description=test"
    );

    server
        .mock("GET", "/bot-token/close")
        .with_body(r#"invalid-data"#)
        .create();
    let err = client.execute(Close).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to execute method: error decoding response body"
    );

    let counter = Mutex::new(0u8);
    server
        .mock("GET", "/bot-token/close")
        .with_body_from_request(move |_| {
            let mut val = counter.lock().unwrap();
            if *val < 2 {
                *val += 1;
                serde_json::to_string(&serde_json::json!({
                    "ok": false,
                    "description": format!("test {val}"),
                    "parameters": {"retry_after": 0}
                }))
                .unwrap()
                .into()
            } else {
                *val = 0;
                r#"{"ok": true, "result": true}"#.into()
            }
        })
        .create();
    client.execute(Close).await.unwrap();
    let err = client.clone().with_max_retries(0).execute(Close).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to execute method: a telegram error has occurred: description=test 1; retry_after=0"
    );
    client.execute(Close).await.unwrap();
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

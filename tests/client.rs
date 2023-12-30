use futures_util::stream::StreamExt;
use mockito::Server;
use tgbot::{api::Client, types::Close};

#[tokio::test]
async fn execute() {
    let mut server = Server::new();
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
        "failed to execute method: error decoding response body: expected value at line 1 column 1"
    );

    server
        .mock("GET", "/bot-token/close")
        .with_body(r#"{"ok": false, "description": "test", "parameters": {"retry_after": 0}}"#)
        .create();
    let err = client.execute(Close).await.unwrap_err();
    assert_eq!(err.to_string(), "failed to execute method: too many requests");
}

#[tokio::test]
async fn download_file() {
    let mut server = Server::new();
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

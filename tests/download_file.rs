use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mockito::{mock, server_url};

use tgbot::api::Client;

#[tokio::test]
async fn download_file() {
    dotenv().ok();
    env_logger::init();
    let _m = mock("GET", "/file/bot-token/file-path")
        .with_body(b"file-data")
        .create();
    let client = Client::new("-token").unwrap().with_host(server_url());
    let mut stream = client.download_file("file-path").await.unwrap();
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        buf.extend(chunk);
    }
    assert_eq!(&buf[..], b"file-data");
}

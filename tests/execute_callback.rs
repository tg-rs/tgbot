#![allow(missing_docs)]
use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "answer-callback-query-base",
            AnswerCallbackQuery::new("query-id"),
            |x| assert!(x),
        ),
        (
            "answer-callback-query-full",
            AnswerCallbackQuery::new("query-id")
                .with_text("text")
                .with_show_alert(true)
                .with_url("https://example.com")
                .with_cache_time(86400),
            |x| assert!(x),
        ),
    ])
    .await;
}

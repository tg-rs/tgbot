#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("approve-suggested-post-base", ApproveSuggestedPost::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "approve-suggested-post-all",
            ApproveSuggestedPost::new(1, 2).with_send_date(1),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("decline-suggested-post-base", DeclineSuggestedPost::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "decline-suggested-post-all",
            DeclineSuggestedPost::new(1, 2).with_comment("test"),
            |x| assert!(x),
        ),
    ])
    .await;
}

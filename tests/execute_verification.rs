#![allow(missing_docs)]
use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute("remove-chat-verification", RemoveChatVerification::new(1), |x| {
        assert!(x)
    })
    .await;
    cx.execute("remove-user-verification", RemoveUserVerification::new(1), |x| {
        assert!(x)
    })
    .await;
    cx.execute_batch([
        ("verify-chat-base", VerifyChat::new(1), |x| assert!(x)),
        (
            "verify-chat-full",
            VerifyChat::new(1).with_custom_description("test"),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("verify-user-base", VerifyUser::new(1), |x| assert!(x)),
        (
            "verify-user-full",
            VerifyUser::new(1).with_custom_description("test"),
            |x| assert!(x),
        ),
    ])
    .await;
}

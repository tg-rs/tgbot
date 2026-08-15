#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([(
        "save-prepared-keyboard-button",
        SavePreparedKeyboardButton::new(1, KeyboardButton::new("test")),
        |x| assert_eq!(x.id, "button-id"),
    )])
    .await;
}

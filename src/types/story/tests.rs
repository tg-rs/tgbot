use crate::types::{tests::assert_json_eq, PrivateChat, Story};

#[test]
fn story() {
    let chat = PrivateChat::new(1, "test");
    assert_json_eq(
        Story::new(chat, 1),
        serde_json::json!({
            "chat": {
                "first_name": "test",
                "id": 1,
                "type": "private"
            },
            "id": 1
        }),
    );
}

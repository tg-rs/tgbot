use crate::types::{tests::assert_json_eq, PreparedInlineMessage};

#[test]
fn prepared_inline_message() {
    assert_json_eq(
        PreparedInlineMessage::new("id", 1),
        serde_json::json!({
            "id": "id",
            "expiration_date": 1,
        }),
    );
}

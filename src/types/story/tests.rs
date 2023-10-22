use crate::types::{tests::assert_json_eq, Story};

#[test]
fn story() {
    assert_json_eq(Story {}, serde_json::json!({}));
}

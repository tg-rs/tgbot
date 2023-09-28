use crate::types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultGame};

#[test]
fn inline_query_result_game_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultGame::new("id", "name")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        ))
        .unwrap(),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
        })
    );
}

#[test]
fn inline_query_result_game_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultGame::new("id", "name"))).unwrap(),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name"
        })
    );
}

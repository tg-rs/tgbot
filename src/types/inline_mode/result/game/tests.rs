use crate::types::{tests::assert_json_eq, InlineKeyboardButton, InlineQueryResult, InlineQueryResultGame};

#[test]
fn inline_query_result_game() {
    assert_json_eq(
        InlineQueryResult::from(
            InlineQueryResultGame::new("id", "name")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]]),
        ),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(InlineQueryResultGame::new("id", "name")),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name"
        }),
    );
}

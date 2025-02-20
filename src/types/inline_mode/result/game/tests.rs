use crate::types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultGame, tests::assert_json_eq};

#[test]
fn inline_query_result_game() {
    assert_json_eq(
        InlineQueryResult::from(
            InlineQueryResultGame::new("name", "id")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
        ),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(InlineQueryResultGame::new("name", "id")),
        serde_json::json!({
            "type": "game",
            "id": "id",
            "game_short_name": "name"
        }),
    );
}

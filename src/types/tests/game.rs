use crate::types::*;

#[test]
fn game() {
    insta::assert_json_snapshot!(
        Game::new(
            "Description",
            [PhotoSize::new("photo-file-id", "photo-file-unique-id", 200, 200)],
            "Game",
        )
        .with_animation(Animation::new(
            24,
            "animation-file-id",
            "animation-file-unique-id",
            200,
            200,
        ))
        .with_text(Text::from("text").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..2)])))
    );
    insta::assert_json_snapshot!(Game::new("Description", [], "Game"));
}

#[test]
fn game_high_score() {
    insta::assert_json_snapshot!(GameHighScore::new(1, 3, User::new(2, "John", false)));
}

#[test]
fn get_game_high_scores() {
    let method = GetGameHighScores::for_chat_message(1, 2, 3);
    assert_payload_eq!(POST JSON "getGameHighScores" => method);
    let method = GetGameHighScores::for_inline_message(1, "inline-message-id");
    assert_payload_eq!(POST JSON "getGameHighScores" => method);
}

#[test]
fn send_game() {
    let method = SendGame::new(1, "Game");
    assert_payload_eq!(POST JSON "sendGame" => method.clone());
    let method = method
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_disable_notification(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "example.com")]])
        .with_reply_parameters(ReplyParameters::new(1));
    assert_payload_eq!(POST JSON "sendGame" => method);
}

#[test]
fn set_game_score() {
    let method = SetGameScore::for_chat_message(1, 2, 3, 100);
    assert_payload_eq!(POST JSON "setGameScore" => method.clone());
    let method = method.with_disable_edit_message(true).with_force(true);
    assert_payload_eq!(POST JSON "setGameScore" => method);

    let method = SetGameScore::for_inline_message("inline-message-id", 3, 100);
    assert_payload_eq!(POST JSON "setGameScore" => method.clone());
    let method = method.with_disable_edit_message(true).with_force(true);
    assert_payload_eq!(POST JSON "setGameScore" => method);
}

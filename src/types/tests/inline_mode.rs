use crate::types::*;

#[test]
fn prepared_inline_message() {
    insta::assert_json_snapshot!(PreparedInlineMessage::new("id", 1));
}

#[test]
fn save_prepared_inline_message() {
    let method = SavePreparedInlineMessage::new(1, InlineQueryResultContact::new("test", "result-id", "+1000"));
    assert_payload_eq!(POST JSON "savePreparedInlineMessage" => method.clone());
    let method = method
        .with_allow_bot_chats(true)
        .with_allow_channel_chats(true)
        .with_allow_group_chats(true)
        .with_allow_user_chats(true);
    assert_payload_eq!(POST JSON "savePreparedInlineMessage" => method);
}

#[test]
fn inline_query() {
    let expected_struct = InlineQuery::new(User::new(1, "test", false), "query id", "query offset", "query string");
    insta::assert_json_snapshot!(
        expected_struct
            .clone()
            .with_chat_type(InlineQueryChatType::Private)
            .with_location(Location::new(1.0, 2.0))
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn inline_query_chat_type() {
    use InlineQueryChatType::*;
    for value in [Sender, Private, Group, Supergroup, Channel] {
        insta::assert_json_snapshot!(value);
    }
}

#[test]
fn answer_inline_query() {
    let text = InputMessageContent::Text(InputMessageContentText::new("text"));
    let article = InlineQueryResult::Article(InlineQueryResultArticle::new("id", text, "title"));
    let method = AnswerInlineQuery::new("id", [article]);
    assert_payload_eq!(POST JSON "answerInlineQuery" => method.clone());
    let method = method
        .with_button(InlineQueryResultsButton::for_start_parameter("text", "param"))
        .with_cache_time(300)
        .with_is_personal(true)
        .with_next_offset("offset");
    assert_payload_eq!(POST JSON "answerInlineQuery" => method);
}

#[test]
fn inline_query_result_article() {
    let result = InlineQueryResultArticle::new("id", InputMessageContentText::new("text"), "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_description("desc")
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_thumbnail_url("thumb-url")
            .with_thumbnail_width(200)
            .with_thumbnail_height(200)
            .with_url("URL"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_audio() {
    let result = InlineQueryResultAudio::new("url", "id", "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_audio_duration(100)
            .with_caption("caption")
            .with_caption_parse_mode(ParseMode::Html)
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_performer("performer")
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_audio() {
    let result = InlineQueryResultCachedAudio::new("file-id", "id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("test")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_contact() {
    let result = InlineQueryResultContact::new("name", "id", "phone");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_last_name("last name")
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_thumbnail_url("url")
            .with_thumbnail_width(200)
            .with_thumbnail_height(200)
            .with_vcard("vcard"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_document() {
    let result = InlineQueryResultDocument::new("url", "id", "mime", "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_thumbnail_height(200)
            .with_thumbnail_url("thumb-url")
            .with_thumbnail_width(200),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_document() {
    let result = InlineQueryResultCachedDocument::new("file-id", "id", "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_game() {
    insta::assert_json_snapshot!(InlineQueryResult::from(
        InlineQueryResultGame::new("name", "id").with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(InlineQueryResultGame::new("name", "id")));
}

#[test]
fn inline_query_result_gif() {
    let result = InlineQueryResultGif::new("url", "id", "thumb-url");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_gif_width(200)
            .with_gif_height(300)
            .with_gif_duration(400)
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true)
            .with_thumbnail_mime_type("video/mp4")
            .with_title("title"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_gif() {
    let result = InlineQueryResultCachedGif::new("file-id", "id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_title("title")
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_location() {
    let result = InlineQueryResultLocation::new("id", 1.0, 2.0, "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_live_period(100)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_thumbnail_url("thumb-url")
            .with_thumbnail_width(200)
            .with_thumbnail_height(300),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_mpeg4_gif() {
    let result = InlineQueryResultMpeg4Gif::new("id", "url", "thumb-url");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_mpeg4_width(200)
            .with_mpeg4_height(300)
            .with_mpeg4_duration(400)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_thumbnail_mime_type("video/mp4")
            .with_title("title")
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_mpeg4_gif() {
    let result = InlineQueryResultCachedMpeg4Gif::new("id", "file-id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true)
            .with_title("title"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_photo() {
    let result = InlineQueryResultPhoto::new("id", "url", "thumb-url");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_photo_height(300)
            .with_photo_width(200)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true)
            .with_title("title"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_photo() {
    let result = InlineQueryResultCachedPhoto::new("id", "file-id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true)
            .with_title("title"),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_sticker() {
    let result = InlineQueryResultCachedSticker::new("id", "file-id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_venue() {
    let result = InlineQueryResultVenue::new("addr", "id", 1.0, 2.0, "title");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_thumbnail_height(300)
            .with_thumbnail_url("thumb-url")
            .with_thumbnail_width(200),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_video() {
    let result = InlineQueryResultVideo::new("id", "mime", "thumb-url", "title", "url");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true)
            .with_video_duration(400)
            .with_video_width(200)
            .with_video_height(300),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_video() {
    let result = InlineQueryResultCachedVideo::new("id", "title", "file-id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_description("desc")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_show_caption_above_media(true),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_voice() {
    let result = InlineQueryResultVoice::new("voice-id", "voice-title", "voice-url");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("voice-caption")
            .with_input_message_content(InputMessageContentText::new("voice-content-text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("voice-kb-text", "voice-kb-url")]])
            .with_voice_duration(100),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn inline_query_result_cached_voice() {
    let result = InlineQueryResultCachedVoice::new("id", "title", "file-id");
    insta::assert_json_snapshot!(InlineQueryResult::from(
        result
            .clone()
            .with_caption("caption")
            .with_input_message_content(InputMessageContentText::new("text"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    ));
    insta::assert_json_snapshot!(InlineQueryResult::from(result));
}

#[test]
fn chosen_inline_result() {
    insta::assert_json_snapshot!(
        ChosenInlineResult::new(User::new(1, "test", false), "q", "result-id")
            .with_location(Location::new(2.0, 1.0))
            .with_inline_message_id("message-id")
    );
    insta::assert_json_snapshot!(ChosenInlineResult::new(User::new(1, "test", false), "q", "result-id"));
}

#[test]
fn inline_query_results_button() {
    insta::assert_json_snapshot!(InlineQueryResultsButton::for_start_parameter("text", "param"));
    insta::assert_json_snapshot!(InlineQueryResultsButton::for_web_app(
        "text",
        WebAppInfo::from("https://example.com")
    ));
}

#[test]
fn sent_web_app_message() {
    insta::assert_json_snapshot!(SentWebAppMessage::default().with_inline_message_id("id"));
    insta::assert_json_snapshot!(SentWebAppMessage::default())
}

#[test]
fn answer_web_app_query() {
    let method = AnswerWebAppQuery::new(
        InlineQueryResultArticle::new("article-id", "article-text", "article-title"),
        "query-id",
    );
    assert_payload_eq!(POST JSON "answerWebAppQuery" => method);
}

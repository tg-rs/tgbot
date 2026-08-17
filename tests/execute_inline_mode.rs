#![allow(missing_docs)]

use std::io::Cursor;

use serde::Serialize;
use tgbot::types::*;

mod cx;

use cx::Cx;

#[derive(Serialize)]
struct InvoiceProviderData {
    key: String,
}

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([
        (
            "save-prepared-inline-message-base",
            SavePreparedInlineMessage::new(1, InlineQueryResultContact::new("test", "result-id", "+1000")),
            |x| {
                assert_eq!(x.id, "test");
                assert_eq!(x.expiration_date, 0);
            },
        ),
        (
            "save-prepared-inline-message-all",
            {
                SavePreparedInlineMessage::new(1, InlineQueryResultContact::new("test", "result-id", "+1000"))
                    .with_allow_bot_chats(true)
                    .with_allow_channel_chats(true)
                    .with_allow_group_chats(true)
                    .with_allow_user_chats(true)
            },
            |x| {
                assert_eq!(x.id, "test");
                assert_eq!(x.expiration_date, 0);
            },
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "answer-inline-query-base",
            AnswerInlineQuery::new(
                "id",
                [InlineQueryResult::from(InlineQueryResultArticle::new(
                    "id", "text", "title",
                ))],
            ),
            |x| assert!(x),
        ),
        (
            "answer-inline-query-all",
            AnswerInlineQuery::new(
                "id",
                [
                    InlineQueryResult::from(
                        InlineQueryResultArticle::new("id", "text", "title")
                            .with_description("desc")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_thumbnail_url("thumb-url")
                            .with_thumbnail_width(200)
                            .with_thumbnail_height(200)
                            .with_url("URL"),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultAudio::new("url", "id", "title")
                            .with_audio_duration(100)
                            .with_caption("caption")
                            .with_input_message_content(InputMessageContent::from(
                                InputMessageContentContact::new("V", "+79001231212")
                                    .with_last_name("P")
                                    .with_vcard("vcard"),
                            ))
                            .with_performer("performer")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedAudio::new("file-id", "id")
                            .with_caption("test")
                            .with_input_message_content(InputMessageContent::from(
                                InputMessageContentInvoice::new(
                                    "RUB",
                                    "description",
                                    "payload",
                                    [LabeledPrice::new(100, "item")],
                                    "title",
                                )
                                .with_is_flexible(true)
                                .with_need_email(false)
                                .with_need_name(true)
                                .with_need_phone_number(true)
                                .with_need_shipping_address(false)
                                .with_provider_data(&InvoiceProviderData {
                                    key: String::from("value"),
                                })
                                .unwrap()
                                .with_provider_token("provider-token")
                                .with_photo_height(24)
                                .with_photo_size(100)
                                .with_photo_width(24)
                                .with_photo_url("https://google.com/favicon.ico")
                                .with_max_tip_amount(1)
                                .with_send_email_to_provider(false)
                                .with_send_phone_number_to_provider(true)
                                .with_suggested_tip_amounts([2]),
                            ))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultContact::new("name", "id", "phone")
                            .with_input_message_content(InputMessageContent::from(
                                Location::new(1.0, 2.0)
                                    .with_heading(90)
                                    .with_horizontal_accuracy(1.5)
                                    .with_live_period(100)
                                    .with_proximity_alert_radius(100),
                            ))
                            .with_last_name("last name")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_thumbnail_url("url")
                            .with_thumbnail_width(200)
                            .with_thumbnail_height(200)
                            .with_vcard("vcard"),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultDocument::new("url", "id", "mime", "title")
                            .with_caption("caption")
                            .with_description("desc")
                            .with_input_message_content(InputMessageContent::from(InputRichMessage::html("test")))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_thumbnail_height(200)
                            .with_thumbnail_url("thumb-url")
                            .with_thumbnail_width(200),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedDocument::new("file-id", "id", "title")
                            .with_caption("caption")
                            .with_description("desc")
                            .with_input_message_content(
                                InputMessageContentText::new(
                                    InputText::from("text")
                                        .with_format(vec![TextEntity::bold(0..10)])
                                        .with_format(ParseMode::Html),
                                )
                                .with_link_preview_options(
                                    LinkPreviewOptions::default()
                                        .with_media_size(LinkPreviewMediaSize::Large)
                                        .with_show_above_text(true)
                                        .with_url("test-url"),
                                ),
                            )
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultGame::new("name", "id")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(InlineQueryResultGame::new("name", "id")),
                    InlineQueryResult::from(
                        InlineQueryResultGif::new("url", "id", "thumb-url")
                            .with_caption("caption")
                            .with_gif_width(200)
                            .with_gif_height(300)
                            .with_gif_duration(400)
                            .with_input_message_content(InputMessageContentText::from(
                                InputText::from("text")
                                    .with_format(ParseMode::Markdown)
                                    .with_format(vec![TextEntity::bold(0..10)]),
                            ))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true)
                            .with_thumbnail_mime_type("video/mp4")
                            .with_title("title"),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedGif::new("file-id", "id")
                            .with_caption("caption")
                            .with_input_message_content(InputMessageContent::from(
                                InputMessageContentVenue::new("addr", 1.0, 2.0, "title")
                                    .with_foursquare_id("f-id")
                                    .with_foursquare_type("f-type")
                                    .with_google_place_id("g-id")
                                    .with_google_place_type("g-type"),
                            ))
                            .with_title("title")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultLocation::new("id", 1.0, 2.0, "title")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_live_period(100)
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_thumbnail_url("thumb-url")
                            .with_thumbnail_width(200)
                            .with_thumbnail_height(300),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultMpeg4Gif::new("id", "url", "thumb-url")
                            .with_caption("caption")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_mpeg4_width(200)
                            .with_mpeg4_height(300)
                            .with_mpeg4_duration(400)
                            .with_thumbnail_mime_type("video/mp4")
                            .with_title("title")
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedMpeg4Gif::new("id", "file-id")
                            .with_caption("caption")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true)
                            .with_title("title"),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultPhoto::new("id", "url", "thumb-url")
                            .with_caption("caption")
                            .with_description("desc")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_photo_height(300)
                            .with_photo_width(200)
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true)
                            .with_title("title"),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedSticker::new("id", "file-id")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultVenue::new("addr", "id", 1.0, 2.0, "title")
                            .with_foursquare_id("f-id")
                            .with_foursquare_type("f-type")
                            .with_google_place_id("g-id")
                            .with_google_place_type("g-type")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_thumbnail_height(300)
                            .with_thumbnail_url("thumb-url")
                            .with_thumbnail_width(200),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultVideo::new("id", "mime", "thumb-url", "title", "url")
                            .with_caption("caption")
                            .with_description("desc")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true)
                            .with_video_duration(400)
                            .with_video_width(200)
                            .with_video_height(300),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedVideo::new("id", "title", "file-id")
                            .with_caption("caption")
                            .with_description("desc")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                            .with_show_caption_above_media(true),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultVoice::new("voice-id", "voice-title", "voice-url")
                            .with_caption("voice-caption")
                            .with_input_message_content(InputMessageContentText::new("voice-content-text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("voice-kb-text", "voice-kb-url")]])
                            .with_voice_duration(100),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultCachedVoice::new("id", "title", "file-id")
                            .with_caption("caption")
                            .with_input_message_content(InputMessageContentText::new("text"))
                            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultLocation::new("id", 1.0, 2.0, "title")
                            .with_heading(1)
                            .with_horizontal_accuracy(1.0)
                            .with_proximity_alert_radius(2)
                            .with_input_message_content(InputMessageContent::from(
                                Venue::new("test-title", "test-address", Location::new(1.0, 2.0))
                                    .with_foursquare_id("f-id")
                                    .with_foursquare_type("f-type")
                                    .with_google_place_id("g-id")
                                    .with_google_place_type("g-type"),
                            )),
                    ),
                    InlineQueryResult::from(
                        InlineQueryResultLocation::new("id", 1.0, 2.0, "title").with_input_message_content(
                            InputMessageContent::from(
                                Contact::new("John", "+11002003040")
                                    .with_last_name("Doe")
                                    .with_user_id(1)
                                    .with_vcard("test-vcard"),
                            ),
                        ),
                    ),
                ],
            )
            .with_button(InlineQueryResultsButton::for_start_parameter("text", "param"))
            .with_cache_time(300)
            .with_is_personal(true)
            .with_next_offset("offset"),
            |x| assert!(x),
        ),
        (
            "answer-inline-query-button-web-app",
            AnswerInlineQuery::new(
                "id",
                [InlineQueryResult::from(
                    InlineQueryResultDocument::new("url", "id", "mime", "title")
                        .with_input_message_content(InputText::from(("test-text", ParseMode::Markdown))),
                )],
            )
            .with_button(InlineQueryResultsButton::for_web_app(
                "test-text",
                WebAppInfo::from("test-url"),
            )),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "answer-web-app-query",
        AnswerWebAppQuery::new(
            InlineQueryResultArticle::new(
                "article-id",
                InputRichMessage::blocks([InputRichBlock::animation(Cursor::new(b"animation-file-data"))]),
                "article-title",
            ),
            "query-id",
        ),
        |x| assert_eq!(x.inline_message_id.unwrap(), "test"),
    )])
    .await;

    cx.execute_batch([(
        "answer-guest-query",
        AnswerGuestQuery::new(
            "test",
            InlineQueryResult::from(
                InlineQueryResultCachedPhoto::new("id", "file-id")
                    .with_caption(("caption", ParseMode::Markdown))
                    .with_description("desc")
                    .with_input_message_content(InputMessageContentText::new("text"))
                    .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                    .with_show_caption_above_media(true)
                    .with_title("title"),
            ),
        ),
        |x| assert_eq!(x.inline_message_id, "test"),
    )])
    .await;
}

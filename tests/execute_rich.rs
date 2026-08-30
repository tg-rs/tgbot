#![allow(missing_docs)]
use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-rich-message-markdown",
            SendRichMessage::new(1, InputRichMessage::markdown("test").with_skip_entity_detection(true)),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::RichMessage(data) = x.data {
                    assert!(data.is_rtl.is_none());
                    let blocks = data.blocks;
                    assert_eq!(blocks.len(), 1);
                    let block = &blocks[0];
                    let RichBlock::Paragraph(p) = block else {
                        panic!("Got an unexpected rich block");
                    };
                    let RichText::PlainText(text) = p else {
                        panic!("Got an unexpected text");
                    };
                    assert_eq!(text, "test");
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-rich-message-html",
            SendRichMessage::new(
                1,
                InputRichMessage::html("test")
                    .with_is_rtl(true)
                    .with_media([("id", InputMediaAnimation::from(InputFile::file_id("test")))]),
            )
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("test")
            .with_direct_messages_topic_id(1)
            .with_disable_notification(true)
            .with_message_effect_id("test")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1))
            .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::RichMessage(data) = x.data {
                    assert!(data.is_rtl.is_none());
                    let blocks = data.blocks;
                    assert_eq!(blocks.len(), 2);
                    let block = &blocks[1];
                    let RichBlock::Animation(b) = block else {
                        panic!("Got an unexpected rich block");
                    };
                    assert_eq!(b.animation.file_id, "file-id");
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-rich-message-blocks",
            SendRichMessage::new(
                1,
                InputRichMessage::blocks([
                    InputRichBlock::anchor("anchor"),
                    InputRichBlock::animation(Cursor::new(b"test")),
                    InputRichBlock::audio(Cursor::new(b"test")),
                    InputRichBlock::blockquote([InputRichBlock::paragraph("bq")], None::<RichText>),
                    InputRichBlock::blockquote([InputRichBlock::paragraph("bqwc")], Some("bqc")),
                    InputRichBlock::collage([], None::<RichText>),
                    InputRichBlock::collage(
                        [InputRichBlock::animation(Cursor::new("collage-animation-file"))],
                        Some("test"),
                    ),
                    InputRichBlock::details("details-1", [], false),
                    InputRichBlock::details(
                        "details-2",
                        [InputRichBlock::audio(Cursor::new("details-audio-file"))],
                        true,
                    ),
                    InputRichBlock::divider(),
                    InputRichBlock::footer("footer"),
                    InputRichBlock::heading("heading", 1),
                    InputRichBlock::list([
                        InputRichBlockListItem::from_iter([InputRichBlock::paragraph("list-item-1")])
                            .with_has_checkbox(true),
                        InputRichBlockListItem::from_iter([InputRichBlock::audio(Cursor::new("list-item-audio-file"))])
                            .with_is_checked(true)
                            .with_type(RichBlockListItemType::UppercaseRoman)
                            .with_value(2),
                        InputRichBlockListItem::from_iter([InputRichBlock::blockquote(
                            [InputRichBlock::audio(Cursor::new("list-item-audio-file"))],
                            None::<RichText>,
                        )])
                        .with_is_checked(true)
                        .with_type(RichBlockListItemType::UppercaseRoman),
                    ]),
                    InputRichBlock::map(Location::new(0.0, 1.0), 0, 1, 2, None::<RichBlockCaption>),
                    InputRichBlock::map(Location::new(0.0, 1.0), 0, 1, 2, Some((String::from("text"), "credit"))),
                    InputRichBlock::mathematical_expression("math-expr"),
                    InputRichBlock::paragraph(RichText::italic("paragraph")),
                    InputRichBlock::photo(
                        InputMediaPhoto::from(Cursor::new("photo-file")).with_show_caption_above_media(true),
                    ),
                    InputRichBlock::pre("print()", Some("python")),
                    InputRichBlock::pullquote("pullquote", None::<RichText>),
                    InputRichBlock::slideshow([InputRichBlock::paragraph("slideshow")], Some("test")),
                    InputRichBlock::table([["test"]]),
                    InputRichBlock::table(
                        InputRichBlockTable::from([[
                            RichBlockTableCell::from("test")
                                .with_align(RichBlockTableCellAlign::Left)
                                .with_colspan(2)
                                .with_is_header(true)
                                .with_rowspan(2)
                                .with_valign(RichBlockTableCellValign::Top),
                            RichBlockTableCell::from("test")
                                .with_align(RichBlockTableCellAlign::Center)
                                .with_valign(RichBlockTableCellValign::Middle),
                            RichBlockTableCell::from("test")
                                .with_align(RichBlockTableCellAlign::Right)
                                .with_valign(RichBlockTableCellValign::Bottom)
                                .with_text("test-override"),
                        ]])
                        .with_caption("test")
                        .with_is_bordered(true)
                        .with_is_compact(true)
                        .with_is_striped(true),
                    ),
                    InputRichBlock::thinking("thinking"),
                    InputRichBlock::thinking(RichText::array([
                        RichText::anchor("test-anchor"),
                        RichText::anchor_link("test-anchor-link", "anchor"),
                        RichText::bank_card_number("test-bank-card-number", "0000"),
                        RichText::bold("test-bold"),
                        RichText::bot_command("test-bot-command", "command"),
                        RichText::cashtag("test-cashtag", "cashtag"),
                        RichText::code("test-code"),
                        RichText::custom_emoji("id", "alt"),
                        RichText::date_time("test-datetime", 0, "YYYY"),
                        RichText::email_address("test-email", "u@h"),
                        RichText::hashtag(RichText::url("test", "https://example.com"), "example"),
                        RichText::italic("test-italic"),
                        RichText::marked(RichText::italic("test-marked-italic")),
                        RichText::mathematical_expression("test-mathematical-expression"),
                        RichText::mention("test-mention", "username"),
                        RichText::phone_number("test-phone-number", "1234"),
                        RichText::plain_text("test-plain-text"),
                        RichText::reference("test-reference"),
                        RichText::reference_link("test-reference-link", "test-reference"),
                        RichText::spoiler("test-spoiler"),
                        RichText::strikethrough("test-strikethrough"),
                        RichText::subscript("test-subscript"),
                        RichText::superscript("test-superscript"),
                        RichText::text_mention(
                            "test-text-mention",
                            User::new(1, "John", false)
                                .with_added_to_attachment_menu(false)
                                .with_is_premium(false)
                                .with_language_code("XX")
                                .with_last_name("Doe")
                                .with_username("johndoe"),
                        ),
                        RichText::underline("test-underline"),
                        RichText::url("test-url", "https://example.com"),
                    ])),
                    InputRichBlock::video(Cursor::new("video-file")),
                    InputRichBlock::voice_note(Cursor::new("voice-note-file")),
                    InputRichBlock::buttons(
                        [
                            RichMessageButton::from((
                                "callback-data",
                                RichMessageButtonType::CallbackData(String::from("test")),
                            ))
                            .with_style(RichMessageButtonStyle::Danger),
                            RichMessageButton::from((
                                "copy-text",
                                RichMessageButtonType::CopyText {
                                    text: String::from("test"),
                                },
                            ))
                            .with_style(RichMessageButtonStyle::Success),
                            RichMessageButton::from(("disabled", RichMessageButtonType::Disabled {}))
                                .with_style(RichMessageButtonStyle::Primary),
                            RichMessageButton::from((
                                "login-url",
                                RichMessageButtonType::LoginUrl(LoginUrl::new("test")),
                            ))
                            .with_style(RichMessageButtonStyle::Link),
                            RichMessageButton::from((
                                "switch-inline-query",
                                RichMessageButtonType::SwitchInlineQuery(String::from("test")),
                            )),
                            RichMessageButton::from((
                                "switch-inline-query-current-chat",
                                RichMessageButtonType::SwitchInlineQueryCurrentChat(String::from("test")),
                            )),
                            RichMessageButton::from((
                                "switch-inline-query-chosen-chat",
                                RichMessageButtonType::SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat::new(
                                    "test",
                                )),
                            )),
                            RichMessageButton::from(("url", RichMessageButtonType::Url(String::from("test")))),
                            RichMessageButton::from((
                                "web-app",
                                RichMessageButtonType::WebApp(WebAppInfo::from("test")),
                            )),
                        ],
                        None,
                    ),
                    InputRichBlock::buttons(
                        [RichMessageButton::from((
                            "callback-data",
                            RichMessageButtonType::CallbackData(String::from("test")),
                        ))],
                        Some(RichBlockButtonsAlignment::Left),
                    ),
                    InputRichBlock::expandable_block_quotation("test", None::<&str>),
                    InputRichBlock::expandable_block_quotation("test", Some("credit")),
                    InputRichBlock::document(InputFile::file_id("test"), None::<&str>),
                ]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::RichMessage(data) = x.data {
                    let RichBlock::Anchor(anchor) = &data.blocks[0] else {
                        panic!("not an anchor")
                    };
                    assert_eq!(anchor, "anchor");
                    let RichBlock::Animation(animation) = &data.blocks[1] else {
                        panic!("not an animation")
                    };
                    assert_eq!(animation.animation.file_id, "file-id");
                    let RichBlock::Audio(audio) = &data.blocks[2] else {
                        panic!("not an audio")
                    };
                    assert_eq!(audio.audio.file_id, "file-id");
                    let RichBlock::BlockQuotation(bq) = &data.blocks[3] else {
                        panic!("not a blockquote")
                    };
                    assert_eq!(bq.blocks.len(), 1);
                    assert!(bq.credit.is_none());
                    let RichBlock::BlockQuotation(bq) = &data.blocks[4] else {
                        panic!("not a blockquote")
                    };
                    assert_eq!(bq.blocks.len(), 1);
                    let RichText::PlainText(bqc) = bq.credit.as_ref().unwrap() else {
                        panic!("invalid credit")
                    };
                    assert_eq!(bqc, "bqc");
                    let RichBlock::Collage(collage) = &data.blocks[5] else {
                        panic!("not a collage")
                    };
                    assert!(collage.blocks.is_empty());
                    assert!(collage.caption.is_none());
                    let RichBlock::Collage(collage) = &data.blocks[6] else {
                        panic!("not a collage")
                    };
                    assert_eq!(collage.blocks.len(), 1);
                    assert!(collage.caption.is_some());
                    let RichBlock::Details(details) = &data.blocks[7] else {
                        panic!("not details")
                    };
                    assert!(details.blocks.is_empty());
                    let RichText::PlainText(summary) = &details.summary else {
                        panic!("invalid summary")
                    };
                    assert_eq!(summary, "details-1");
                    let RichBlock::Details(details) = &data.blocks[8] else {
                        panic!("not details")
                    };
                    assert_eq!(details.blocks.len(), 1);
                    let RichText::PlainText(summary) = &details.summary else {
                        panic!("invalid summary")
                    };
                    assert_eq!(summary, "details-2");
                    assert!(matches!(&data.blocks[9], RichBlock::Divider));
                    let RichBlock::Footer(RichText::PlainText(footer)) = &data.blocks[10] else {
                        panic!("not a footer")
                    };
                    assert_eq!(footer, "footer");
                    let RichBlock::SectionHeading(RichText::PlainText(heading), size) = &data.blocks[11] else {
                        panic!("not a heading")
                    };
                    assert_eq!(heading, "heading");
                    assert_eq!(*size, 1);
                    let RichBlock::List(list) = &data.blocks[12] else {
                        panic!("not a list")
                    };
                    assert_eq!(list.len(), 3);
                    let RichBlock::Map(map) = &data.blocks[13] else {
                        panic!("not a map")
                    };
                    assert_eq!(map.location.latitude, 0.0);
                    assert_eq!(map.location.longitude, 1.0);
                    assert!(map.caption.is_none());
                    let RichBlock::Map(map) = &data.blocks[14] else {
                        panic!("not a map")
                    };
                    assert_eq!(map.location.latitude, 0.0);
                    assert_eq!(map.location.longitude, 1.0);
                    assert!(map.caption.is_some());
                    let RichBlock::MathematicalExpression(expr) = &data.blocks[15] else {
                        panic!("not a math expr")
                    };
                    assert_eq!(expr, "math-expr");
                    let RichBlock::Paragraph(RichText::Italic(p)) = &data.blocks[16] else {
                        panic!("not a paragraph")
                    };
                    assert!(matches!(**p, RichText::PlainText(_)));
                    let RichBlock::Photo(photo) = &data.blocks[17] else {
                        panic!("not a photo")
                    };
                    assert_eq!(photo.photo.len(), 1);
                    let RichBlock::Preformatted(pre) = &data.blocks[18] else {
                        panic!("not a pre")
                    };
                    let RichText::PlainText(pre_text) = &pre.text else {
                        panic!("invalid pre text")
                    };
                    assert_eq!(pre_text, "print()");
                    let RichBlock::PullQuotation(pq) = &data.blocks[19] else {
                        panic!("not a pull quote")
                    };
                    assert!(matches!(pq.text, RichText::PlainText(_)));
                    let RichBlock::Slideshow(slideshow) = &data.blocks[20] else {
                        panic!("not a slideshow")
                    };
                    assert_eq!(slideshow.blocks.len(), 1);
                    let RichBlock::Table(table) = &data.blocks[21] else {
                        panic!("not a table")
                    };
                    assert_eq!(table.cells.len(), 1);
                    assert_eq!(table.cells[0].len(), 1);
                    let RichBlock::Table(table) = &data.blocks[22] else {
                        panic!("not a table")
                    };
                    assert_eq!(table.cells.len(), 1);
                    assert_eq!(table.cells[0].len(), 3);
                    let RichBlock::Thinking(RichText::PlainText(thinking)) = &data.blocks[23] else {
                        panic!("not a thinking")
                    };
                    assert_eq!(thinking, "thinking");
                    let RichBlock::Thinking(RichText::Array(thinking)) = &data.blocks[24] else {
                        panic!("not a thinking")
                    };
                    assert!(!thinking.is_empty());
                    let RichBlock::Video(video) = &data.blocks[25] else {
                        panic!("not a video")
                    };
                    assert_eq!(video.video.file_id, "file-id");
                    let RichBlock::VoiceNote(voice_note) = &data.blocks[26] else {
                        panic!("not a voice note")
                    };
                    assert_eq!(voice_note.voice_note.file_id, "file-id");
                    let RichBlock::Buttons(buttons) = &data.blocks[27] else {
                        panic!("not a buttons list")
                    };
                    assert_eq!(buttons.buttons.len(), 9);
                    assert!(buttons.align.is_none());
                    let RichBlock::ExpandableBlockQuotation(ebq) = &data.blocks[28] else {
                        panic!("not an expandable block quotation")
                    };
                    assert!(matches!(ebq.text, RichText::PlainText(_)));
                    assert!(ebq.credit.is_none());
                    let RichBlock::ExpandableBlockQuotation(ebq) = &data.blocks[29] else {
                        panic!("not an expandable block quotation")
                    };
                    assert!(matches!(ebq.text, RichText::PlainText(_)));
                    assert!(ebq.credit.is_some());
                    let RichBlock::Document(document) = &data.blocks[30] else {
                        panic!("not a document")
                    };
                    assert_eq!(document.document.file_id, "file-id");
                    assert!(document.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "send-rich-message-draft-base",
            SendRichMessageDraft::new(1, 2, InputRichMessage::markdown("test")),
            |x| assert!(x),
        ),
        (
            "send-rich-message-draft-all",
            SendRichMessageDraft::new(1, 2, InputRichMessage::markdown("test")).with_message_thread_id(1),
            |x| assert!(x),
        ),
    ])
    .await;
}

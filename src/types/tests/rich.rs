use crate::types::*;

#[test]
fn input_rich_message() {
    insta::assert_json_snapshot!(InputRichMessage::markdown("test"));
    insta::assert_json_snapshot!(InputRichMessage::html("test"));
    insta::assert_json_snapshot!(
        InputRichMessage::html("test")
            .with_is_rtl(true)
            .with_skip_entity_detection(true)
    );
}

#[test]
fn rich_block() {
    insta::assert_json_snapshot!(vec![
        RichBlock::anchor("test-anchor"),
        RichBlock::animation(Animation::new(10, "file-id", "file-unique-id", 200, 200)),
        RichBlock::animation((Animation::new(10, "file-id", "file-unique-id", 200, 200), "caption",)),
        RichBlock::animation((
            Animation::new(10, "file-id", "file-unique-id", 200, 200),
            "caption",
            true
        )),
        RichBlock::audio(Audio::new(10, "file-id", "file-unique-id")),
        RichBlock::audio((
            Audio::new(10, "file-id", "file-unique-id"),
            ("test-caption", "test-caption-credit")
        )),
        RichBlock::block_quotation(
            ["bq1", "bq2"]
                .into_iter()
                .collect::<RichBlockBlockQuotation>()
                .with_credit("test")
        ),
        RichBlock::block_quotation(RichBlock::paragraph("test-block-quotation-1")),
        RichBlock::block_quotation((RichBlock::paragraph("test-block-quotation-2"), "credits")),
        RichBlock::collage(["test-collage-1", "test-collage-2"]),
        RichBlock::collage(
            ["test-collage-1", "test-collage-2"]
                .into_iter()
                .collect::<RichBlockCollage>()
                .with_caption("test")
        ),
        RichBlock::details((["test-details"], "summary")),
        RichBlock::details((["test-details"], "summary", true)),
        RichBlock::footer("test-footer"),
        RichBlock::list([("test-1", ["test-1"]), ("test-2", ["test-2"])]),
        RichBlock::list([
            RichBlockListItem::new("test", ["test-item"]),
            RichBlockListItem::new("test", ["test-item"])
                .with_has_checkbox(true)
                .with_is_checked(true)
                .with_item_type(RichBlockListItemType::Decimal)
                .with_value(1)
        ]),
        RichBlock::map(Location::new(1.0, 2.0)),
        RichBlock::map(
            RichBlockMap::from(Location::new(1.0, 2.0))
                .with_caption("test")
                .with_height(300)
                .with_width(300)
                .with_zoom(20)
        ),
        RichBlock::mathematical_expression("test-mathematical-expression"),
        RichBlock::paragraph("test-paragraph"),
        RichBlock::photo([PhotoSize::new("file-id", "file-unique-id", 200, 200)]),
        RichBlock::photo(
            RichBlockPhoto::from([PhotoSize::new("file-id", "file-unique-id", 200, 200)])
                .with_caption("test")
                .with_has_spoiler(true)
        ),
        RichBlock::preformatted("test-preformatted-1"),
        RichBlock::preformatted(("lang", "test-preformatted-2")),
        RichBlock::pull_quotation("test-pull-quotation-1"),
        RichBlock::pull_quotation(("test-pull-quotation-2", "credits")),
        RichBlock::section_heading("heading", 1),
        RichBlock::slideshow(["test-slideshow-1", "test-slideshow-2"]),
        RichBlock::slideshow(RichBlockSlideshow::from(["test-slideshow-1", "test-slideshow-2"]).with_caption("test")),
        RichBlock::table([["test-cell-1", "test-cell-2"]]),
        RichBlock::table(
            RichBlockTable::from([[
                RichBlockTableCell::default()
                    .with_align(RichBlockTableCellAlign::Left)
                    .with_colspan(2)
                    .with_is_header(true)
                    .with_rowspan(3)
                    .with_text("test")
                    .with_valign(RichBlockTableCellValign::Top),
                RichBlockTableCell::default()
            ]])
            .with_caption("test")
            .with_is_bordered(true)
            .with_is_striped(true)
        ),
        RichBlock::thinking("test-thinking"),
        RichBlock::video(Video::new(10, "file-id", "file-unique-id", 200, 200)),
        RichBlock::video((Video::new(10, "file-id", "file-unique-id", 200, 200), "caption")),
        RichBlock::video((Video::new(10, "file-id", "file-unique-id", 200, 200), "caption", true)),
        RichBlock::voice_note(Voice::new(20, "file-id", "file-unique-id")),
        RichBlock::voice_note((Voice::new(20, "file-id", "file-unique-id"), "caption"))
    ]);
}

#[test]
fn rich_message() {
    insta::assert_json_snapshot!(RichMessage::from(["test"]));
    insta::assert_json_snapshot!(RichMessage::from(["test"]).with_is_rtl(true));
}

#[test]
fn rich_text() {
    insta::assert_json_snapshot!(RichText::array([
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
        RichText::text_mention("test-text-mention", User::new(1, "John", false)),
        RichText::underline("test-underline"),
        RichText::url("test-url", "https://example.com"),
    ]));

    serde_json::from_value::<RichText>(serde_json::json!(
        [
            {
                "type": "anchor",
                "name": "test-anchor"
            },
            {
                "type": "anchor_link",
                "text": "test-anchor-link",
                "anchor_name": "anchor"
            },
            {
                "type": "bank_card_number",
                "text": "test-bank-card-number",
                "bank_card_number": "0000"
            },
            {
                "type": "bold",
                "text": "test-bold"
            },
            {
                "type": "bot_command",
                "text": "test-bot-command",
                "bot_command": "command"
            },
            {
                "type": "cashtag",
                "text": "test-cashtag",
                "cashtag": "cashtag"
            },
            {
                "type": "code",
                "text": "test-code"
            },
            {
                "type": "custom_emoji",
                "custom_emoji_id": "id",
                "alternative_text": "alt"
            },
            {
                "type": "date_time",
                "text": "test-datetime",
                "unix_time": 0,
                "date_time_format": "YYYY"
            },
            {
                "type": "email_address",
                "text": "test-email",
                "email_address": "u@h"
            },
            {
                "type": "hashtag",
                "text": {
                    "type": "url",
                    "text": "test",
                    "url": "https://example.com"
                },
                "hashtag": "example"
            },
            {
                "type": "italic",
                "text": "test-italic"
            },
            {
                "type": "marked",
                "text": {
                    "type": "italic",
                    "text": "test-marked-italic"
                }
            },
            {
                "type": "mathematical_expression",
                "expression": "test-mathematical-expression"
            },
            {
                "type": "mention",
                "text": "test-mention",
                "username": "username"
            },
            {
                "type": "phone_number",
                "text": "test-phone-number",
                "phone_number": "1234"
            },
            "test-plain-text",
            {
                "type": "reference",
                "name": "test-reference"
            },
            {
                "type": "reference_link",
                "text": "test-reference-link",
                "reference_name": "test-reference"
            },
            {
                "type": "spoiler",
                "text": "test-spoiler"
            },
            {
                "type": "strikethrough",
                "text": "test-strikethrough"
            },
            {
                "type": "subscript",
                "text": "test-subscript"
            },
            {
                "type": "superscript",
                "text": "test-superscript"
            },
            {
                "type": "text_mention",
                "text": "test-text-mention",
                "user": {
                    "first_name": "John",
                    "id": 1,
                    "is_bot": false
                }
            },
            {
                "type": "underline",
                "text": "test-underline"
            },
            {
                "type": "url",
                "text": "test-url",
                "url": "https://example.com"
            }
        ]
    ))
    .unwrap();
}

#[test]
fn send_rich_message() {
    let method = SendRichMessage::new(1, InputRichMessage::markdown("test"));
    assert_payload_eq!(POST JSON "sendRichMessage" => method.clone());
    let method = method
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("test")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_message_effect_id("test")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default());
    assert_payload_eq!(POST JSON "sendRichMessage" => method.clone());
}

#[test]
fn send_rich_message_draft() {
    let method = SendRichMessageDraft::new(1, 2, InputRichMessage::markdown("test"));
    assert_payload_eq!(POST JSON "sendRichMessageDraft" => method.clone());
    let method = method.with_message_thread_id(1);
    assert_payload_eq!(POST JSON "sendRichMessageDraft" => method.clone());
}

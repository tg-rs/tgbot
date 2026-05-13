use crate::types::*;

#[test]
fn live_photo() {
    let expected_struct = LivePhoto::new(1, "file-id", "file-unique-id", 200, 300);
    insta::assert_json_snapshot!(expected_struct.clone());

    let expected_struct = expected_struct
        .with_mime_type("video/gif")
        .with_file_size(1000)
        .with_photo([PhotoSize::new("p-id", "p-uid", 200, 300)]);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn send_live_photo() {
    let method = SendLivePhoto::new(1, InputFile::file_id("file-id"), InputFile::file_id("photo-id"));
    assert_payload_eq!(POST FORM "sendLivePhoto" => method);
    let method = SendLivePhoto::new(1, InputFile::file_id("file-id"), InputFile::file_id("photo-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("bc-id")
        .with_caption("test")
        .with_caption_entities([TextEntity::bold(0..2)])
        .unwrap()
        .with_direct_messages_topic_id(1)
        .with_disable_notification(false)
        .with_has_spoiler(false)
        .with_message_effect_id("eid")
        .with_message_thread_id(10)
        .with_parse_mode(ParseMode::MarkdownV2)
        .with_protect_content(true)
        .with_reply_markup(ReplyKeyboardRemove::default())
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_show_caption_above_media(true)
        .with_suggested_post_parameters(&SuggestedPostParameters::default())
        .unwrap();
    assert_payload_eq!(POST FORM "sendLivePhoto" => method);
}

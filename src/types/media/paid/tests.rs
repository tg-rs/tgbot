use crate::{
    api::{Form, Payload, assert_payload_eq},
    types::{
        ForceReply,
        InputFile,
        InputPaidMediaGroup,
        InputPaidMediaGroupItem,
        PaidMedia,
        PaidMediaInfo,
        PaidMediaPreview,
        PaidMediaPurchased,
        PhotoSize,
        ReplyMarkup,
        ReplyParameters,
        SendPaidMedia,
        TextEntities,
        TextEntity,
        User,
        Video,
        tests::assert_json_eq,
    },
};

#[test]
fn paid_media_purchased() {
    assert_json_eq(
        PaidMediaPurchased::new(User::new(1, "John", false), "payload"),
        serde_json::json!({
            "from": {
                "id": 1,
                "first_name": "John",
                "is_bot": false,
            },
            "paid_media_payload": "payload"
        }),
    );
}

#[test]
fn paid_media_info() {
    assert_json_eq(
        PaidMediaInfo::new(
            1,
            [
                PaidMedia::Photo(vec![PhotoSize::new("file-id", "file-unique-id", 200, 300)]),
                PaidMedia::Preview(
                    PaidMediaPreview::default()
                        .with_duration(100)
                        .with_height(200)
                        .with_width(300),
                ),
            ],
        ),
        serde_json::json!({
            "star_count": 1,
            "paid_media": [
                {
                    "type": "photo",
                    "photo": [
                        {
                            "file_id": "file-id",
                            "file_unique_id": "file-unique-id",
                            "height": 200,
                            "width": 300,
                        }
                    ]
                },
                {
                    "type": "preview",
                    "duration": 100,
                    "height": 200,
                    "width": 300
                }
            ]
        }),
    );
}

#[test]
fn paid_media_photo() {
    assert_json_eq(
        PaidMedia::Photo(vec![PhotoSize::new("file-id", "file-unique-id", 200, 300)]),
        serde_json::json!({
            "type": "photo",
            "photo": [
                {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "height": 200,
                    "width": 300,
                }
            ]
        }),
    );
}

#[test]
fn paid_media_preview() {
    assert_json_eq(
        PaidMedia::Preview(PaidMediaPreview::default()),
        serde_json::json!({"type": "preview"}),
    );
    assert_json_eq(
        PaidMedia::Preview(
            PaidMediaPreview::default()
                .with_duration(100)
                .with_height(200)
                .with_width(300),
        ),
        serde_json::json!({
            "type": "preview",
            "duration": 100,
            "height": 200,
            "width": 300
        }),
    );
}

#[test]
fn paid_media_video() {
    assert_json_eq(
        PaidMedia::Video(Video::new(100, "file-id", "file-unique-id", 200, 300)),
        serde_json::json!({
            "type": "video",
            "video": {
                "duration": 100,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 200,
                "width": 300,
            }
        }),
    );
}

#[test]
fn send_paid_media() {
    let media = InputPaidMediaGroup::new([InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id"))]).unwrap();
    let method = SendPaidMedia::new(1, media, 100);
    let form = Form::from([
        ("chat_id", 1.into()),
        ("media", "[{\"type\":\"photo\",\"media\":\"file-id\"}]".into()),
        ("star_count", 100.into()),
    ]);
    assert_payload_eq(Payload::form("sendPaidMedia", form), method);

    let media = InputPaidMediaGroup::new([InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id"))]).unwrap();
    let caption_entities = vec![TextEntity::bold(0..1)];
    let reply_parameters = ReplyParameters::new(1);
    let reply_markup = ForceReply::new(true);
    let method = SendPaidMedia::new(1, media, 100)
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("c-id")
        .with_caption("caption")
        .with_caption_entities(caption_entities.clone())
        .unwrap()
        .with_disable_notification(true)
        .with_payload("payload")
        .with_protect_content(true)
        .with_reply_parameters(reply_parameters.clone())
        .unwrap()
        .with_reply_markup(reply_markup.clone())
        .unwrap()
        .with_show_caption_above_media(true);
    let form = Form::from([
        ("chat_id", 1.into()),
        ("media", "[{\"type\":\"photo\",\"media\":\"file-id\"}]".into()),
        ("star_count", 100.into()),
        ("allow_paid_broadcast", true.into()),
        ("business_connection_id", "c-id".into()),
        ("caption", "caption".into()),
        (
            "caption_entities",
            serde_json::to_string(&TextEntities::from_iter(caption_entities))
                .unwrap()
                .into(),
        ),
        ("disable_notification", true.into()),
        ("payload", "payload".into()),
        ("protect_content", true.into()),
        (
            "reply_parameters",
            serde_json::to_string(&reply_parameters).unwrap().into(),
        ),
        (
            "reply_markup",
            serde_json::to_string(&ReplyMarkup::from(reply_markup)).unwrap().into(),
        ),
        ("show_caption_above_media", true.into()),
    ]);
    assert_payload_eq(Payload::form("sendPaidMedia", form), method);
}

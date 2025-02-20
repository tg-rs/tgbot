use crate::types::{
    Animation,
    Audio,
    ChannelChat,
    Contact,
    Dice,
    DiceType,
    Document,
    ExternalReplyData,
    ExternalReplyInfo,
    Game,
    Giveaway,
    GiveawayWinners,
    Invoice,
    LinkPreviewOptions,
    Location,
    MessageOriginHiddenUser,
    PaidMedia,
    PaidMediaInfo,
    PaidMediaPreview,
    PhotoSize,
    Poll,
    PollOption,
    PrivateChat,
    RegularPoll,
    Sticker,
    StickerType,
    Story,
    User,
    Venue,
    Video,
    VideoNote,
    Voice,
    tests::assert_json_eq,
};

fn create_origin() -> MessageOriginHiddenUser {
    MessageOriginHiddenUser::new(1, "test")
}

#[test]
fn external_reply_info_animation() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Animation::new(10, "file-id", "file-unique-id", 20, 30), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "animation": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 20,
                "width": 30
            }
        }),
    );
}

#[test]
fn external_reply_info_audio() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Audio::new(10, "file-id", "file-unique-id"), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "audio": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id"
            }
        }),
    );
}

#[test]
fn external_reply_info_contact() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Contact::new("first-name", "+79001230099"), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "contact": {
                "first_name": "first-name",
                "phone_number": "+79001230099"
            }
        }),
    );
}

#[test]
fn external_reply_info_dice() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Dice::new(DiceType::Basketball, 1), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "dice": {
                "emoji": "🏀",
                "value": 1
            }
        }),
    );
}

#[test]
fn external_reply_info_document() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Document::new("file-id", "file-unique-id"), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id"
            }
        }),
    );
}

#[test]
fn external_reply_info_game() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            Game::new(
                "description",
                [PhotoSize::new("file-id", "file-unique-id", 10, 20)],
                "title",
            ),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "game": {
                "description": "description",
                "photo": [
                    {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "height": 10,
                        "width": 20
                    }
                ],
                "title": "title"
            }
        }),
    );
}

#[test]
fn external_reply_info_giveaway() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Giveaway::new([ChannelChat::new(1, "test")], 1, 1), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "giveaway": {
                "chats": [
                    {
                        "type": "channel",
                        "id": 1,
                        "title": "test"
                    }
                ],
                "winners_selection_date": 1,
                "winner_count": 1
            }
        }),
    );
}

#[test]
fn external_reply_info_giveaway_winners() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            GiveawayWinners::new(ChannelChat::new(1, "test"), 1, 1, [User::new(1, "test", false)], 1),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "giveaway_winners": {
                "chat": {
                    "type": "channel",
                    "id": 1,
                    "title": "test"
                },
                "giveaway_message_id": 1,
                "winner_count": 1,
                "winners": [
                    {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                ],
                "winners_selection_date": 1
            }
        }),
    );
}

#[test]
fn external_reply_info_invoice() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            Invoice::new("RUB", "description", "start-parameter", "title", 1),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "invoice": {
                "currency": "RUB",
                "description": "description",
                "start_parameter": "start-parameter",
                "title": "title",
                "total_amount": 1
            }
        }),
    );
}

#[test]
fn external_reply_info_location() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Location::new(1.0, 2.0), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "location": {
                "latitude": 1.0,
                "longitude": 2.0
            }
        }),
    );
}

#[test]
fn external_reply_info_paid_media() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            PaidMediaInfo::new(100, [PaidMedia::Preview(PaidMediaPreview::default())]),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "paid_media": {
                "star_count": 100,
                "paid_media": [
                    {
                        "type": "preview"
                    }
                ]
            }
        }),
    );
}

#[test]
fn external_reply_info_photo() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(vec![PhotoSize::new("file-id", "file-unique-id", 10, 20)], origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "photo": [
                {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "height": 10,
                    "width": 20
                }
            ]
        }),
    );
}

#[test]
fn external_reply_info_poll() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            Poll::from(
                RegularPoll::new("poll-id", "Rust?")
                    .with_is_anonymous(true)
                    .with_is_closed(true)
                    .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                    .with_total_voter_count(1000),
            ),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "poll": {
                "id": "poll-id",
                "question": "Rust?",
                "options": [
                    {"text": "Yes", "voter_count": 1000},
                    {"text": "No", "voter_count": 0}
                ],
                "is_closed": true,
                "total_voter_count": 1000,
                "is_anonymous": true,
                "type": "regular",
                "allows_multiple_answers": false
            }
        }),
    );
}

#[test]
fn external_reply_info_sticker() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            Sticker::new("file-id", "file-unique-id", StickerType::Regular, 10, 20),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "sticker": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "type": "regular",
                "height": 10,
                "width": 20,
                "is_animated": false,
                "is_video": false
            }
        }),
    );
}

#[test]
fn external_reply_info_story() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            ExternalReplyData::Story(Story::new(PrivateChat::new(1, "test"), 1)),
            origin,
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "story": {
                "chat": {
                    "first_name": "test",
                    "id": 1,
                    "type": "private"
                },
                "id": 1
            }
        }),
    );
}

#[test]
fn external_reply_info_venue() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Venue::new("title", "address", Location::new(1.0, 2.0)), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "venue": {
                "title": "title",
                "address": "address",
                "location": {
                    "latitude": 1.0,
                    "longitude": 2.0
                }
            }
        }),
    );
}

#[test]
fn external_reply_info_video() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Video::new(10, "file-id", "file-unique-id", 20, 30), origin),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "video": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 20,
                "width": 30
            }
        }),
    );
}

#[test]
fn external_reply_info_video_note() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(VideoNote::new(10, "file-id", "file-unique-id", 20), origin.clone()),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "video_note": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "length": 20
            }
        }),
    );
}

#[test]
fn external_reply_info_voice() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(Voice::new(10, "file-id", "file-unique-id"), origin.clone()),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "voice": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id"
            }
        }),
    );
}

#[test]
fn external_reply_info_unknown() {
    let origin = create_origin();
    assert_json_eq(
        ExternalReplyInfo::new(
            ExternalReplyData::Unknown(serde_json::json!({"key": "value"})),
            origin.clone(),
        ),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "key": "value"
        }),
    );
    assert_json_eq(
        ExternalReplyInfo::new(ExternalReplyData::Unknown(serde_json::json!({"key": "value"})), origin)
            .with_chat(ChannelChat::new(1, "test"))
            .with_has_media_spoiler(true)
            .with_link_preview_options(LinkPreviewOptions::default())
            .with_message_id(1),
        serde_json::json!({
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            },
            "has_media_spoiler": true,
            "link_preview_options": {},
            "message_id": 1,
            "key": "value"
        }),
    );
}

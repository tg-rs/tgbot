use serde::Deserialize;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AnswerCallbackQuery,
        CallbackQuery,
        Chat,
        Message,
        MessageData,
        MessageSender,
        SupergroupChat,
        Text,
        User,
    },
};

#[derive(Clone, Debug, Deserialize)]
struct QueryData {
    k: String,
}

#[test]
fn callback_query() {
    let user = User {
        id: 1,
        is_bot: false,
        first_name: String::from("User"),
        last_name: None,
        username: None,
        language_code: None,
        is_premium: None,
        added_to_attachment_menu: None,
    };

    let expected_struct = CallbackQuery {
        id: String::from("id"),
        from: user.clone(),
        message: Some(Message {
            id: 2,
            date: 0,
            edit_date: None,
            sender: MessageSender::User(user),
            chat: Chat::Supergroup(SupergroupChat {
                id: 3,
                title: String::from("Supergroup Chat"),
                username: None,
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                permissions: None,
                slow_mode_delay: None,
                message_auto_delete_time: None,
                linked_chat_id: None,
                location: None,
                has_protected_content: None,
                join_to_send_messages: None,
                join_by_request: None,
                is_forum: None,
            }),
            author_signature: None,
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("text"),
                entities: None,
            }),
        }),
        inline_message_id: Some(String::from("id")),
        chat_instance: Some(String::from("instance")),
        data: Some(String::from(r#"{"k": "v"}"#)),
        game_short_name: Some(String::from("Game")),
    };

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "id",
            "from": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            },
            "message": {
                "message_id": 2,
                "date": 0,
                "from": {"id": 1, "first_name": "User", "is_bot": false},
                "chat": {"id": 3, "type": "supergroup", "title": "Supergroup Chat"},
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "inline_message_id": "id",
            "chat_instance": "instance",
            "data": "{\"k\": \"v\"}",
            "game_short_name": "Game"
        }),
    );

    let parsed_query_data: QueryData = expected_struct.parse_data().unwrap().unwrap();
    assert_eq!(parsed_query_data.k, "v");

    let expected_struct = CallbackQuery {
        id: String::from("test"),
        from: User {
            id: 1,
            is_bot: false,
            first_name: String::from("test"),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
        },
        message: None,
        inline_message_id: None,
        chat_instance: None,
        data: None,
        game_short_name: None,
    };
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "test",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );

    assert!(expected_struct.parse_data::<QueryData>().unwrap().is_none());
}

#[test]
fn answer_callback_query() {
    let method = AnswerCallbackQuery::new("id");
    assert_payload_eq(
        Payload::json(
            "answerCallbackQuery",
            serde_json::json!({
                "callback_query_id": "id"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "answerCallbackQuery",
            serde_json::json!({
                "callback_query_id": "id",
                "text": "text",
                "show_alert": true,
                "url": "url",
                "cache_time": 10
            }),
        ),
        method.text("text").show_alert(true).url("url").cache_time(10),
    );
}

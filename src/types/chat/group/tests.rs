use crate::types::{
    tests::assert_json_eq,
    Chat,
    ChatPermissions,
    ChatPhoto,
    GroupChat,
    Message,
    MessageData,
    ReactionType,
    Text,
    User,
};

#[test]
fn group_chat() {
    let expected_struct = Chat::Group(
        GroupChat::new(1, "Group")
            .with_photo(ChatPhoto::new(
                "big-file-id",
                "big-file-unique-id",
                "small-file-id",
                "small-file-unique-id",
            ))
            .with_invite_link("example.com/join/group")
            .with_pinned_message(Message::new(
                1,
                0,
                GroupChat::new(1, "Group"),
                MessageData::Text(Text::from("text")),
                User::new(1, "User", false),
            ))
            .with_permissions(ChatPermissions::allowed())
            .with_has_protected_content(true)
            .with_message_auto_delete_time(86400)
            .with_has_hidden_members(true)
            .with_available_reactions([ReactionType::emoji("👍")])
            .with_emoji_status_custom_emoji_id("emoji-id")
            .with_emoji_status_expiration_date(0),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "invite_link": "example.com/join/group",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "permissions": {
                "can_send_messages": true,
                "can_send_audios": true,
                "can_send_documents": true,
                "can_send_photos": true,
                "can_send_videos": true,
                "can_send_video_notes": true,
                "can_send_voice_notes": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
                "can_manage_topics": true,
            },
            "has_protected_content": true,
            "message_auto_delete_time": 86400,
            "has_hidden_members": true,
            "available_reactions": [
                {
                    "type": "emoji",
                    "emoji": "👍"
                }
            ],
            "emoji_status_custom_emoji_id": "emoji-id",
            "emoji_status_expiration_date": 0
        }),
    );
    assert_json_eq(
        Chat::Group(GroupChat::new(1, "Group")),
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
        }),
    );
}

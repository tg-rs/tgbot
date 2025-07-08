use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn reaction_count() {
    insta::assert_json_snapshot!(ReactionCount::new(ReactionType::emoji("🤡"), 1));
}

#[test]
fn reaction_type() {
    insta::assert_json_snapshot!(ReactionType::custom_emoji("5420319826440644296"));
    insta::assert_json_snapshot!(ReactionType::emoji("🤡"));
    insta::assert_json_snapshot!(ReactionType::Paid);
}

#[test]
fn message_reaction_count_updated() {
    insta::assert_json_snapshot!(MessageReactionCountUpdated::new(
        PrivateChat::new(1, "test"),
        0,
        1,
        [ReactionCount::new(ReactionType::emoji("🤡"), 1)],
    ));
}

#[test]
fn message_reaction_updated() {
    let expected_struct = MessageReactionUpdated::new(
        PrivateChat::new(1, "test"),
        0,
        1,
        [ReactionType::emoji("🤡")],
        [ReactionType::emoji("🤮")],
    );
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(expected_struct.clone().with_actor_chat(ChannelChat::new(1, "test")),);
    insta::assert_json_snapshot!(expected_struct.clone().with_user(User::new(1, "test", false)));
}

#[test]
fn set_message_reaction() {
    let method = SetMessageReaction::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "setMessageReaction",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "is_big": false
            }),
        ),
        method.clone(),
    );

    let method = method.with_is_big(true).with_reaction([ReactionType::emoji("🤡")]);
    assert_payload_eq(
        Payload::json(
            "setMessageReaction",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "is_big": true,
                "reaction": [
                    {
                        "type": "emoji",
                        "emoji": "🤡"
                    }
                ]
            }),
        ),
        method.clone(),
    );
}

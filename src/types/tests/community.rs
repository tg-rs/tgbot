use crate::types::*;

#[test]
fn community() {
    let data = Community {
        id: 1,
        name: String::from("test"),
    };
    insta::assert_json_snapshot!(data);
}

#[test]
fn community_chat_added() {
    let data = CommunityChatAdded {
        community: Community {
            id: 1,
            name: String::from("test"),
        },
    };
    insta::assert_json_snapshot!(data);
}

#[test]
fn community_chat_removed() {
    let data = CommunityChatRemoved {};
    insta::assert_json_snapshot!(data);
}

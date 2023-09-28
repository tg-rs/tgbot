use std::collections::HashMap;

use crate::types::ChatId;

#[test]
fn chat_id() {
    let chat_id = ChatId::from(1);
    if let ChatId::Id(chat_id) = chat_id {
        assert_eq!(chat_id, 1);
    } else {
        panic!("Unexpected chat id: {:?}", chat_id);
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#"1"#);
    assert_eq!(chat_id.to_string(), "1");

    let chat_id = ChatId::from("username");
    if let ChatId::Username(ref username) = chat_id {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected chat id: {:?}", chat_id);
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
    assert_eq!(chat_id.to_string(), "username");

    let chat_id = ChatId::from(String::from("username"));
    if let ChatId::Username(ref username) = chat_id {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected chat id: {:?}", chat_id);
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
    assert_eq!(chat_id.to_string(), "username");

    let mut map = HashMap::new();
    let chat_id_1 = ChatId::from(1);
    let chat_id_2 = ChatId::from("username");
    map.insert(chat_id_1.clone(), "1".to_string());
    map.insert(chat_id_2.clone(), "2".to_string());
    assert_eq!(map.get(&chat_id_1).unwrap(), "1");
    assert_eq!(map.get(&chat_id_2).unwrap(), "2");
}

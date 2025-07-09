use crate::types::*;

#[test]
fn remove_chat_verification() {
    assert_payload_eq!(POST JSON "removeChatVerification" => RemoveChatVerification::new(1));
}

#[test]
fn remove_user_verification() {
    assert_payload_eq!(POST JSON "removeUserVerification" => RemoveUserVerification::new(1));
}

#[test]
fn verify_chat() {
    let method = VerifyChat::new(1);
    assert_payload_eq!(POST JSON "verifyChat" => method.clone());
    let method = method.with_custom_description("test").clone();
    assert_payload_eq!(POST JSON "verifyChat" => method);
}

#[test]
fn verify_user() {
    let method = VerifyUser::new(1);
    assert_payload_eq!(POST JSON "verifyUser" => method.clone());
    let method = method.with_custom_description("test").clone();
    assert_payload_eq!(POST JSON "verifyUser" => method);
}

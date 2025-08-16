use crate::types::*;

#[test]
fn concat() {
    insta::assert_json_snapshot!(
        Contact::new("John", "+79001231212")
            .with_last_name("Doe")
            .with_user_id(1)
            .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
    );
    insta::assert_json_snapshot!(Contact::new("John", "+79001231212"));
}

#[test]
fn send_contact() {
    let method = SendContact::new(1, "John", "+79001231212");
    assert_payload_eq!(POST JSON "sendContact" => method.clone());
    let method = method
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_last_name("Doe")
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD");
    assert_payload_eq!(POST JSON "sendContact" => method);
}

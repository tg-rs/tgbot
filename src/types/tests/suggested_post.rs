use crate::types::*;

fn create_message() -> Message {
    Message::new(
        1,
        2,
        PrivateChat::new(1, "John"),
        MessageData::Text("test".into()),
        MessageSender::User(User::new(1, "John", false)),
    )
}

fn create_price() -> SuggestedPostPrice {
    SuggestedPostPrice::new(1, "test")
}

#[test]
fn approve_suggested_post() {
    let method = ApproveSuggestedPost::new(1, 2);
    assert_payload_eq!(POST JSON "approveSuggestedPost" => method.clone());
    let method = method.with_send_date(3);
    assert_payload_eq!(POST JSON "approveSuggestedPost" => method);
}

#[test]
fn decline_suggested_post() {
    let method = DeclineSuggestedPost::new(1, 2);
    assert_payload_eq!(POST JSON "declineSuggestedPost" => method.clone());
    let method = method.with_comment("test");
    assert_payload_eq!(POST JSON "declineSuggestedPost" => method);
}

#[test]
fn suggested_post_approval_failed() {
    let price = create_price();
    let message = create_message();
    let expected_struct = SuggestedPostApprovalFailed::new(price);
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct.with_suggested_post_message(message);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_approved() {
    let price = create_price();
    let message = create_message();
    let expected_struct = SuggestedPostApproved::new(1);
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct.with_price(price).with_suggested_post_message(message);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_declined() {
    let message = create_message();
    let expected_struct = SuggestedPostDeclined::default();
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct
        .with_comment("test")
        .with_suggested_post_message(message);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_info() {
    let price = create_price();
    let expected_struct = SuggestedPostInfo::new(SuggestedPostState::Approved);
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct.with_price(price).with_send_date(1);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_paid() {
    let message = create_message();
    let expected_struct = SuggestedPostPaid::new("test");
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct
        .with_amount(1)
        .with_star_amount(2)
        .with_suggested_post_message(message);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_parameters() {
    let price = create_price();
    let expected_struct = SuggestedPostParameters::default();
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct.with_price(price).with_send_date(1);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_price() {
    insta::assert_json_snapshot!(create_price());
}

#[test]
fn suggested_post_refunded() {
    let expected_struct = SuggestedPostRefunded::new(SuggestedPostRefundReason::PaymentRefunded);
    insta::assert_json_snapshot!(expected_struct);
    let message = create_message();
    let expected_struct =
        SuggestedPostRefunded::new(SuggestedPostRefundReason::PostDeleted).with_suggested_post_message(message);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn suggested_post_refund_reason() {
    insta::assert_json_snapshot!(SuggestedPostRefundReason::PaymentRefunded);
    insta::assert_json_snapshot!(SuggestedPostRefundReason::PostDeleted);
}

#[test]
fn suggested_post_state() {
    insta::assert_json_snapshot!(SuggestedPostState::Approved);
    insta::assert_json_snapshot!(SuggestedPostState::Declined);
    insta::assert_json_snapshot!(SuggestedPostState::Pending);
}

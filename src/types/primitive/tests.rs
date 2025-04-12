use crate::types::StarAmount;

#[test]
fn star_amount() {
    let value = StarAmount::from(1);
    assert_eq!(value.amount, 1);
    assert!(value.nanostar_amount.is_none());
    let value = value.with_nanostar_amount(1);
    assert_eq!(value.nanostar_amount, Some(1));
}

use crate::types::{tests::assert_json_eq, MaskPosition, MaskPositionPoint};

#[test]
fn mask_position() {
    assert_json_eq(
        MaskPosition::new(MaskPositionPoint::Forehead, 1.0, 0.0, 1.0),
        serde_json::json!({
            "point": "forehead",
            "x_shift": 0.0,
            "y_shift": 1.0,
            "scale": 1.0
        }),
    );
}

#[test]
fn mask_position_point() {
    use crate::types::MaskPositionPoint::*;
    for (expected_struct, expected_value) in [
        (Forehead, serde_json::json!("forehead")),
        (Eyes, serde_json::json!("eyes")),
        (Mouth, serde_json::json!("mouth")),
        (Chin, serde_json::json!("chin")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

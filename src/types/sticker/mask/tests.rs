use crate::types::{MaskPosition, MaskPositionPoint};

#[test]
fn mask_position_deserialize() {
    let data: MaskPosition = serde_json::from_value(serde_json::json!({
        "point": "chin",
        "x_shift": 0.0,
        "y_shift": 1.0,
        "scale": 1.0
    }))
    .unwrap();

    assert_eq!(data.point, MaskPositionPoint::Chin);
    assert_eq!(data.x_shift, 0.0);
    assert_eq!(data.y_shift, 1.0);
    assert_eq!(data.scale, 1.0);
}

#[test]
fn mask_position_serialize() {
    let value = serde_json::to_value(MaskPosition {
        point: MaskPositionPoint::Forehead,
        x_shift: 0.0,
        y_shift: 1.0,
        scale: 1.0,
    })
    .unwrap();

    assert_eq!(
        value,
        serde_json::json!({
            "point": "forehead",
            "x_shift": 0.0,
            "y_shift": 1.0,
            "scale": 1.0
        })
    );
}

#[test]
fn mask_position_point_serialize() {
    use crate::types::MaskPositionPoint::*;
    for (variant, expected) in [
        (Forehead, r#""forehead""#),
        (Eyes, r#""eyes""#),
        (Mouth, r#""mouth""#),
        (Chin, r#""chin""#),
    ] {
        assert_eq!(serde_json::to_string(&variant).unwrap(), expected);
    }
}

#[test]
fn mask_position_pont_deserialize() {
    use crate::types::MaskPositionPoint::*;
    for (raw_value, expected) in [
        (r#""forehead""#, Forehead),
        (r#""eyes""#, Eyes),
        (r#""mouth""#, Mouth),
        (r#""chin""#, Chin),
    ] {
        assert_eq!(serde_json::from_str::<MaskPositionPoint>(raw_value).unwrap(), expected);
    }
}

use super::*;

macro_rules! test_bool_ser {
    ($fn:ident($name:ident = $value:ident)) => {
        #[test]
        fn $fn() {
            let value = serde_json::to_string(&$name).unwrap();
            assert_eq!(value, stringify!($value));
        }
    };
}

macro_rules! test_bool_de {
    ($( #[should_panic = $msg:literal] )? $fn:ident($name:ident = $value:ident)) => {
        #[test]
        $( #[should_panic = $msg] )?
        fn $fn() {
            serde_json::from_str::<$name>(stringify!($value)).unwrap();
        }
    };
}

test_bool_ser!(serialize_true(True = true));
test_bool_ser!(serialize_false(False = false));

test_bool_de!(deserialize_true(True = true));
test_bool_de! {
    #[should_panic = "invalid value: boolean `false`, expected true"]
    deserialize_true_unexpected(True = false)
}
test_bool_de!(deserialize_false(False = false));
test_bool_de! {
    #[should_panic = "invalid value: boolean `true`, expected false"]
    deserialize_false_unexpected(False = true)
}

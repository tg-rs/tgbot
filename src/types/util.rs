use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use std::fmt;

macro_rules! impl_bool_ty {
    ($name:ident = $value:ident) => {
        #[derive(Clone, Copy)]
        pub struct $name;

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let b = bool::deserialize(deserializer)?;
                if b == $value {
                    Ok(Self)
                } else {
                    Err(D::Error::invalid_value(Unexpected::Bool(b), &stringify!($value)))
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&$value, formatter)
            }
        }
    };
}

impl_bool_ty!(BoolTrue = true);
impl_bool_ty!(BoolFalse = false);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_bool {
        ($( #[should_panic = $msg:literal] )? $fn:ident($name:ident = $value:ident)) => {
            #[test]
            $( #[should_panic = $msg] )?
            fn $fn() {
                serde_json::from_str::<$name>(stringify!($value)).unwrap();
            }
        };
    }

    test_bool!(bool_true(BoolTrue = true));

    test_bool! {
        #[should_panic = "invalid value: boolean `false`, expected true"]
        bool_true_unexpected(BoolTrue = false)
    }

    test_bool!(bool_false(BoolFalse = false));

    test_bool! {
        #[should_panic = "invalid value: boolean `true`, expected false"]
        bool_false_unexpected(BoolFalse = true)
    }
}

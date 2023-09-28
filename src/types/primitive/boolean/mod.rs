use std::fmt;

use serde::{
    de::{Error, Unexpected},
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

#[cfg(test)]
mod tests;

macro_rules! impl_bool_type {
    ($name:ident = $value:ident) => {
        #[derive(Clone, Copy)]
        pub(crate) struct $name;

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_bool($value)
            }
        }

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

impl_bool_type!(True = true);
impl_bool_type!(False = false);

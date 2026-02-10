use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{Integer, Text, TextEntities, True, User};

pub mod serde_box {
    use super::*;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Box<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Box::new)
    }
}

#[derive(Deserialize, Serialize)]
pub(super) struct RawDataBoostAdded {
    boost_count: Integer,
}

impl RawDataBoostAdded {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<Integer, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataBoostAdded::deserialize(deserializer).map(|RawDataBoostAdded { boost_count }| boost_count)
    }

    pub(super) fn serialize_value<S>(value: &Integer, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = RawDataBoostAdded { boost_count: *value };
        value.serialize(serializer)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub(super) struct RawCaption {
    caption: String,
    caption_entities: Option<TextEntities>,
}

impl RawCaption {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<RawCaption>::deserialize(deserializer).map(|wrapper| {
            wrapper.map(
                |RawCaption {
                     caption: data,
                     caption_entities: entities,
                 }| Text { data, entities },
            )
        })
    }

    pub(super) fn serialize_value<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.clone().map(|value| RawCaption {
            caption: value.data,
            caption_entities: value.entities,
        });
        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RawDataChatOwnerChanged {
    new_owner: User,
}

impl RawDataChatOwnerChanged {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<User, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataChatOwnerChanged::deserialize(deserializer).map(|x| x.new_owner)
    }

    pub(super) fn serialize_value<S>(value: &User, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataChatOwnerChanged {
            new_owner: value.clone(),
        }
        .serialize(serializer)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RawDataChatOwnerLeft {
    new_owner: Option<User>,
}

impl RawDataChatOwnerLeft {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<User>, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataChatOwnerLeft::deserialize(deserializer).map(|x| x.new_owner)
    }

    pub(super) fn serialize_value<S>(value: &Option<User>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataChatOwnerLeft {
            new_owner: value.clone(),
        }
        .serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RawDataEmpty {}

impl RawDataEmpty {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataEmpty::deserialize(deserializer).map(|_| ())
    }

    pub(super) fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataEmpty {}.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
pub(super) struct RawDataFlag;

impl RawDataFlag {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        True::deserialize(deserializer).map(|_| ())
    }

    pub(super) fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        True.serialize(serializer)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub(super) struct RawDataText {
    text: String,
    entities: Option<TextEntities>,
}

impl RawDataText {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<Text, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawDataText::deserialize(deserializer).map(|x| Text {
            data: x.text,
            entities: x.entities,
        })
    }

    pub(super) fn serialize_value<S>(value: &Text, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawDataText {
            text: value.data.clone(),
            entities: value.entities.clone(),
        }
        .serialize(serializer)
    }
}

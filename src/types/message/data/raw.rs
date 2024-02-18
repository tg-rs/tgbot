use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{Integer, Text, TextEntities, True};

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

#[derive(Deserialize, Serialize)]
pub(super) struct RawCaption {
    caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Deserialize, Serialize)]
pub(super) struct RawDataText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

use std::fmt::Display;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    if let Some(ref d) = *value {
        return serializer.collect_str(d);
    }
    serializer.serialize_none()
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        return Ok(Some(s.parse().map_err(de::Error::custom)?));
    }

    Ok(None)
}

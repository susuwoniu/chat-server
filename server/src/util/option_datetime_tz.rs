use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(value: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if let Some(d) = *value {
    let dt = DateTime::<Utc>::from_utc(d, Utc);
    return serializer.serialize_str(&dt.to_rfc3339());
  }
  serializer.serialize_none()
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  if let Some(s) = s {
    return Ok(Some(
      DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?
        .naive_utc(),
    ));
  }
  Ok(None)
}

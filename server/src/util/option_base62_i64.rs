use serde::{self, Deserialize, Deserializer, Serializer};
// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if let Some(d) = *value {
    let base64_value = bs62::encode_num(&d);
    return serializer.serialize_str(&base64_value);
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
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  if let Some(s) = s {
    let num = bs62::decode_num(&s).map_err(serde::de::Error::custom)?;
    return Ok(Some(num.to_u64_digits()[0] as i64));
  }
  println!("deserialize: s is None");
  Ok(None)
}

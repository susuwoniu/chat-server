use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let base64_value = bs62::encode_num(value);
  serializer.collect_str(&base64_value)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  let value = String::deserialize(deserializer)?;
  let num = bs62::decode_num(&value).map_err(de::Error::custom)?;
  return Ok(num.to_u64_digits()[0] as i64);
}

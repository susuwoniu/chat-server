use super::constant::{ACCESS_TOKEN_KEY, PHONE_CODE_TEMP_KEY};
pub fn get_phone_code_temp_key(phone_country_code: i32, phone_number: String) -> String {
  let temp_key = format!(
    "{}/{}{}",
    PHONE_CODE_TEMP_KEY, phone_country_code, phone_number
  );
  return temp_key;
}
pub fn get_access_token_key(token: String) -> String {
  let temp_key = format!("{}/{}", ACCESS_TOKEN_KEY, token);
  return temp_key;
}

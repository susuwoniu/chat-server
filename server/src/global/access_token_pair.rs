use crate::util::key_pair::Pair;
use once_cell::sync::OnceCell;
pub static ACCESS_TOKEN_PAIR: OnceCell<AccessTokenPair> = OnceCell::new();
#[derive(Debug)]
pub struct AccessTokenPair(pub Pair);

impl AccessTokenPair {
  pub fn global() -> &'static Self {
    ACCESS_TOKEN_PAIR.get().expect("read access token failed")
  }
  pub fn init(secret_key: &str, public_key: &str) {
    ACCESS_TOKEN_PAIR
      .set(Self(Pair::from_str(secret_key, public_key)))
      .unwrap();
  }
}

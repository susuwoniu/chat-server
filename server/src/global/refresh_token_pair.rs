use crate::{global::Config, util::key_pair::Pair};
use once_cell::sync::OnceCell;
pub static REFRESH_TOKEN_PAIR: OnceCell<RefreshTokenPair> = OnceCell::new();
#[derive(Debug)]
pub struct RefreshTokenPair(pub Pair);

impl RefreshTokenPair {
  pub fn global() -> &'static Self {
    REFRESH_TOKEN_PAIR.get().expect("read refresh token failed")
  }
  pub fn init(refresh_token_secret_key: &str, refresh_token_public_key: &str) {
    REFRESH_TOKEN_PAIR
      .set(RefreshTokenPair(Pair::from_str(
        refresh_token_secret_key,
        refresh_token_public_key,
      )))
      .unwrap();
  }
}

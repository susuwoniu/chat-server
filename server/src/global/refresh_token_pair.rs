use crate::util::key_pair::Pair;
use once_cell::sync::OnceCell;
pub static REFRESH_TOKEN_PAIR: OnceCell<RefreshTokenPair> = OnceCell::new();
#[derive(Debug)]
pub struct RefreshTokenPair(pub Pair);

impl RefreshTokenPair {
  pub fn global() -> &'static Self {
    REFRESH_TOKEN_PAIR.get().expect("read refresh token failed")
  }
}

use once_cell::sync::OnceCell;
pub static SENSITIVE_WORDS: OnceCell<SensitiveWords> = OnceCell::new();
#[derive(Debug)]
pub struct SensitiveWords(pub Vec<String>);

impl SensitiveWords {
  pub fn global() -> &'static Self {
    SENSITIVE_WORDS.get().expect("read sensitive words failed")
  }
}

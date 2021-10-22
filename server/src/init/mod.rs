use crate::i18n::I18N;
pub fn init() {
  let hello = I18N.read().unwrap().with_lang("hello", "en-US");
}

use once_cell::sync::OnceCell;
pub static SENSITIVE_WORDS: OnceCell<SensitiveWords> = OnceCell::new();
#[derive(Debug)]
pub struct SensitiveWords(pub Vec<String>);

impl SensitiveWords {
    pub fn global() -> &'static Self {
        SENSITIVE_WORDS.get().expect("read sensitive words failed")
    }
    pub fn init() {
        let mut sensitive_words: Vec<String> = Vec::new();
        include_str!("../../../resources/sensitive-words/politics.txt")
            .split("\n")
            .for_each(|word| {
                sensitive_words.push(word.to_string());
            });
        include_str!("../../../resources/sensitive-words/gun.txt")
            .split("\n")
            .for_each(|word| {
                sensitive_words.push(word.to_string());
            });
        include_str!("../../../resources/sensitive-words/porn.txt")
            .split("\n")
            .for_each(|word| {
                sensitive_words.push(word.to_string());
            });
        include_str!("../../../resources/sensitive-words/other.txt")
            .split("\n")
            .for_each(|word| {
                sensitive_words.push(word.to_string());
            });
        SENSITIVE_WORDS
            .set(SensitiveWords(sensitive_words))
            .unwrap();
    }
}

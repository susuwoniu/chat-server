use rand::{distributions::Alphanumeric, Rng}; // 0.8

pub fn get_random_letter(len: usize) -> String {
  let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();
  s
}

pub fn to_first_letter_uppertcase(s: &str) -> String {
  format!(
    "{}{}",
    s.chars().next().unwrap().to_uppercase(),
    s.chars().skip(1).collect::<String>()
  )
}
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

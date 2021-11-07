use rand::Rng;

pub fn get_randome_code() -> String {
  let mut rng = rand::thread_rng();
  let mut code: String = "".to_string();
  for _ in 0..6 {
    code.push_str(&rng.gen_range(0..10).to_string())
  }
  return code;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_get_random_code() {
    let code = get_randome_code();

    assert_eq!(code.chars().count(), 6);
  }
}

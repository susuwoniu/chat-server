use crate::{types::ServiceResult, util::base62_to_i64};
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

pub fn parse_skip_range(arr: &Vec<&str>) -> ServiceResult<Vec<[i64; 2]>> {
  Ok(
    arr
      .iter()
      .filter_map(|s| {
        let mut parts = s.split('-');
        let start = base62_to_i64(parts.next()?).ok()?;
        let end = base62_to_i64(parts.next()?).ok()?;
        dbg!(&start);
        Some([start, end])
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

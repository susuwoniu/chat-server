use std::fs;
#[allow(dead_code)]
pub fn get_content_sync(file: &str) -> String {
  let contents =
    fs::read_to_string(file).expect(&format!("Something went wrong reading the file {}", file));
  return contents;
}

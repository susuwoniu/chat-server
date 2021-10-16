use sonyflake::Sonyflake;

pub fn next_id() -> i64 {
  let mut sf = Sonyflake::new().unwrap();
  let next_id = sf.next_id().unwrap();
  next_id as i64
}

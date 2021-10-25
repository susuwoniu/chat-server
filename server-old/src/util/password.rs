use rand::Rng;

pub fn generate() -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789)(*&^%$#@!~";
  const PASSWORD_LEN: usize = 32;
  let mut rng = rand::thread_rng();

  let password: String = (0..PASSWORD_LEN)
    .map(|_| {
      let idx = rng.gen_range(0..CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();

  return password;
}

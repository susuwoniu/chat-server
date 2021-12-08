use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn hash(data: &str) -> String {
  let mut hasher = Sha3::sha3_256();
  hasher.input_str(data);
  return hasher.result_str();
}

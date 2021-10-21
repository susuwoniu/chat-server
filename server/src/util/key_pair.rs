use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use subtle_encoding::hex;

#[derive(Debug)]
pub struct Pair {
  secret: SecretKey,
  public: PublicKey,
}
impl Pair {
  pub fn new() -> Pair {
    let mut csprng = rand07::rngs::OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    Pair {
      secret: keypair.secret,
      public: keypair.public,
    }
  }
  pub fn get_secret_bytes(&self) -> [u8; 32] {
    return self.secret.to_bytes();
  }
  pub fn get_public_bytes(&self) -> [u8; 32] {
    return self.public.to_bytes();
  }
  pub fn from_string(secret_encoded: String, public_encoded: String) -> Pair {
    let secret_bytes = hex::decode(secret_encoded).expect("decode key paire secret string failed");
    let secret = SecretKey::from_bytes(&secret_bytes).expect("parse secret failed");

    let public_bytes = hex::decode(public_encoded).expect("decode key paire public string failed");
    let public = PublicKey::from_bytes(&public_bytes).expect("parse public pair failed");

    Pair { secret, public }
  }
  pub fn get_secret_string(&self) -> String {
    let secret = self.secret.to_bytes();
    let secret_encoded = hex::encode(secret);
    return String::from_utf8(secret_encoded).unwrap();
  }
  pub fn get_public_string(&self) -> String {
    let public = self.public.to_bytes();

    let public_encoded = hex::encode(public);
    return String::from_utf8(public_encoded).unwrap();
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_new_key_pair() {
    let pair = Pair::new();
    let secret = pair.get_secret_string();
    let public = pair.get_public_string();
    println!("secret: {}", secret);
    println!("public: {}", public);
    assert_eq!(secret.chars().count(), 64);
    assert_eq!(public.chars().count(), 64);
  }
}

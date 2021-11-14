use crate::util::string::to_first_letter_uppertcase;
use config::{Config as ConfigBuilder, ConfigError, Environment, File, FileFormat};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fmt;
use url::Url;
pub static CONFIG: OnceCell<Config> = OnceCell::new();
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub server: Server,
  pub log: Log,
  pub env: ENV,
  pub i18n: I18n,
  pub db: Db,
  pub workers_count: Option<usize>,
  pub auth: Auth,
  pub kv: Kv,
  pub clients: Vec<Client>,
  pub invite_only: bool,
  pub default_timezone_offset_in_seconds: i32,
  pub account: Account,
  pub use_fixed_code: bool,
  pub page_size: i64,
  pub post: Post,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Post {
  pub default_listed_posts_duration_in_days: i64,
  pub min_post_content_count: i64,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Auth {
  pub secret_key: String,
  pub public_key: String,
  pub access_token_expires_in_minutes: i64,
  pub phone_code_verification_expires_in_minutes: i64,
  pub phone_code_verification_duration_in_seconds: i64,
  pub signature_client_date_expires_in_minutes: i64,
  pub refresh_token_secret_key: String,
  pub refresh_token_public_key: String,
  pub refresh_token_expires_in_days: i64,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Log {
  pub level: String,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Account {
  pub max_profile_images: i32,
  pub min_age: i32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub socket_address: SocketAddr,
  pub url: Url,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Db {
  pub max_connections: u32,
}
// kv storage
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Kv {
  pub url: String,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct I18n {
  pub fallback_language: String,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Template {
  pub directory: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ENV {
  Dev,
  Test,
  Prod,
}
impl Default for ENV {
  fn default() -> Self {
    ENV::Dev
  }
}

impl fmt::Display for ENV {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ENV::Dev => write!(f, "dev"),
      ENV::Test => write!(f, "test"),
      ENV::Prod => write!(f, "prod"),
    }
  }
}
#[derive(Debug, Deserialize, Clone)]
pub struct Client {
  pub client_id: i64,
  pub client_secret: String,
  pub name: String,
}

const CONFIG_DIRECTORY: &str = "config";
const CONFIG_ENV_PREFIX: &str = "CHAT";
impl Config {
  pub fn new() -> Result<Self, ConfigError> {
    let env = std::env::var("RUST_ENV").unwrap_or("dev".into());
    let lower_case_env = env.to_lowercase();
    // try first letter uppercase
    let first_letter_uppercase_env = to_first_letter_uppertcase(&env);
    let mut s = ConfigBuilder::default();
    let default_config = include_str!("./default.toml");
    s.merge(File::from_str(default_config, FileFormat::Toml))?;
    s.merge(File::with_name(&format!("{}/default", CONFIG_DIRECTORY)).required(false))?;
    s.set("env", first_letter_uppercase_env.clone())?;
    let env_config_file_name = File::with_name(&format!(
      "{}/{}",
      CONFIG_DIRECTORY,
      &format!("{}", lower_case_env)
    ))
    .required(false);
    s.merge(env_config_file_name)?;
    // This makes it so "EA_SERVER__PORT overrides server.port
    s.merge(Environment::with_prefix(CONFIG_ENV_PREFIX))?;
    s.try_into()
  }
  pub fn global() -> &'static Self {
    CONFIG.get().expect("read config failed")
  }
}

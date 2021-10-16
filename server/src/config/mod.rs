use crate::util::string::to_first_letter_uppertcase;
use config::{Config as ConfigBuilder, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::fmt;
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Log {
  pub level: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Server {
  pub host: String,
  pub port: u16,
  pub url: String,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Db {
  pub url: String,
  pub max_connections: u32,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct I18n {
  pub language: String,
  pub fallback_language: String,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Template {
  pub directory: String,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Config {
  pub server: Server,
  pub log: Log,
  pub env: ENV,
  pub i18n: I18n,
  pub db: Db,
  pub workers_count: Option<usize>,
}
const CONFIG_DIRECTORY: &str = "config";
const CONFIG_ENV_PREFIX: &str = "COMMUNICATION";
impl Config {
  pub fn new() -> Result<Self, ConfigError> {
    let env = std::env::var("RUST_ENV").unwrap_or("dev".into());
    let lower_case_env = env.to_lowercase();
    // try first letter uppercase
    let first_letter_uppercase_env = to_first_letter_uppertcase(&env);
    let mut s = ConfigBuilder::default();
    let default_config = include_str!("./default.toml");
    s.merge(File::from_str(default_config, FileFormat::Toml))?;
    s.merge(File::with_name(&format!("{}/server-default", CONFIG_DIRECTORY)).required(false))?;
    s.set("env", first_letter_uppercase_env.clone())?;
    let env_config_file_name = File::with_name(&format!(
      "{}/{}",
      CONFIG_DIRECTORY,
      &format!("server-{}", lower_case_env)
    ))
    .required(false);
    s.merge(env_config_file_name)?;
    // This makes it so "EA_SERVER__PORT overrides server.port
    s.merge(Environment::with_prefix(CONFIG_ENV_PREFIX))?;
    s.try_into()
  }
}

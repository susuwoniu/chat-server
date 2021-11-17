use crate::{
  account::{
    model::{IdentityType, SignupParam},
    service::signup::signup,
  },
  error::Error,
  global::Config,
  middleware::{ClientPlatform, Locale},
};
use dialoguer::Input;
use ipnetwork17::IpNetwork;
use sqlx::postgres::PgPoolOptions;

pub async fn create_admin() -> Result<(), Error> {
  println!("start create admin account");
  let phone_number: String = Input::new()
    .with_prompt("Please input the admin phone number?")
    .interact_text()?;
  let cfg = Config::global();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
  let phone_country_code = 86;

  let identifier = format!("{}{}", &phone_country_code, phone_number);
  // Database
  let pool = PgPoolOptions::new()
    .max_connections(cfg.db.max_connections)
    .connect(&database_url)
    .await?;
  let account_auth = signup(
    &Locale::default(),
    &pool,
    SignupParam {
      identity_type: IdentityType::Phone,
      identifier,
      phone_country_code: Some(phone_country_code),
      phone_number: Some(phone_number),
      timezone_in_seconds: 28800,
      ip: "127.0.0.1".parse::<IpNetwork>().unwrap(),
      platform: ClientPlatform::IOS,
      admin: true,
    },
  )
  .await?;
  println!("account_auth: {:?}", account_auth);
  Ok(())
}

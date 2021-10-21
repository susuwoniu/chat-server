pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type KvPool = deadpool::managed::Pool<deadpool_redis::Manager, deadpool_redis::Connection>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Null;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuccessNullResponse {
  pub data: Option<Null>,
}

impl SuccessNullResponse {
  pub fn null() -> Self {
    SuccessNullResponse { data: None }
  }
}

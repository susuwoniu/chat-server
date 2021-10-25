pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type KvPool = deadpool::managed::Pool<deadpool_redis::Manager, deadpool_redis::Connection>;
pub use crate::config::Config;
pub use crate::error::{Error, RootError, ServiceError, ServiceResult};
pub use crate::i18n::I18N;
pub use crate::middleware::req_meta::ReqMeta;
pub use crate::util::key_pair::{Pair, RefreshTokenPair};

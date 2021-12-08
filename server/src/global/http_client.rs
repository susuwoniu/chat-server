use once_cell::sync::OnceCell;
use std::time::Duration;
pub static HTTP_CLIENT: OnceCell<HttpClient> = OnceCell::new();
#[derive(Debug)]
pub struct HttpClient(pub reqwest::ClientBuilder);
impl HttpClient {
  pub fn global() -> &'static Self {
    HTTP_CLIENT.get().expect("read client failed")
  }
  pub fn init() {
    let reqwest_client_builder = reqwest::Client::builder()
      .connect_timeout(Duration::from_secs(10))
      .timeout(Duration::from_secs(30))
      .pool_max_idle_per_host(1);
    HTTP_CLIENT.set(HttpClient(reqwest_client_builder)).unwrap();
  }
}

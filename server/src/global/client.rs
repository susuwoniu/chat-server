use crate::global::ClientConfig;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
pub static CLIENT_MAP: OnceCell<Client> = OnceCell::new();
#[derive(Debug)]
pub struct Client(pub HashMap<i64, ClientConfig>);
impl Client {
  pub fn global() -> &'static Self {
    CLIENT_MAP.get().expect("read refresh token failed")
  }
  pub fn get(client_id: i64) -> Option<&'static ClientConfig> {
    Self::global().0.get(&client_id)
  }
  pub fn init(clients: &Vec<ClientConfig>) {
    let mut client_map: HashMap<i64, ClientConfig> = HashMap::new();
    for client in clients {
      client_map.insert(client.client_id, client.clone());
    }
    CLIENT_MAP.set(Client(client_map)).unwrap();
  }
}

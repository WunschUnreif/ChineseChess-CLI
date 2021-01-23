#[allow(unused_imports)]
use log::{info, error, warn};

use chess_datagram::*;

impl super::Handler {

  pub fn user_registry(&mut self, username: &String) -> DataPacketToClient {
    let mut model = self.model.lock();
    if model.is_err() {
      return DataPacketToClient::error(String::from("Internal Error"));
    }
    let model = model.as_mut().unwrap();

    let stream = self.stream.try_clone();
    if stream.is_err() {
      return DataPacketToClient::error(String::from("Internal Error"));
    }
    let stream = stream.unwrap();

    let result = model.user_manager.add_user(username, stream);
    if result.is_err() {
      return DataPacketToClient::error(String::from("Cannot register user. Try another username."));
    }
    
    self.username = Some(username.clone());

    DataPacketToClient::success()
  }

}

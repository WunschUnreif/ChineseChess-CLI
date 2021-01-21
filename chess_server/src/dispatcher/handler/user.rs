use std::{net::TcpStream, sync::{Arc, Mutex}};

#[allow(unused_imports)]
use log::{info, error, warn};

use chess_datagram::*;
use crate::model::Model;

pub fn user_registry(username: &String, stream: &TcpStream, model: Arc<Mutex<Model>>) -> DataPacketToClient {
  let mut model = model.lock();
  if model.is_err() {
    return DataPacketToClient::error(String::from("Internal Error"));
  }
  let model = model.as_mut().unwrap();

  let stream = stream.try_clone();
  if stream.is_err() {
    return DataPacketToClient::error(String::from("Internal Error"));
  }
  let stream = stream.unwrap();

  let result = model.user_manager.add_user(username, stream);
  if result.is_err() {
    return DataPacketToClient::error(String::from("Cannot register user. Try another username."));
  }

  DataPacketToClient::success()
}

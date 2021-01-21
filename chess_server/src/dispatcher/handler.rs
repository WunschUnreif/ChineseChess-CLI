use std::{io::{BufReader, BufRead}, net::TcpStream, sync::{self, Mutex}};

#[allow(unused_imports)]
use log::{info, error, warn};
use sync::Arc;

use chess_datagram::*;
use crate::model::*;

mod user;

pub fn handle_connection(stream: TcpStream, model: Arc<Mutex<Model>>) -> Result<(), std::io::Error> {
  let mut echo_stream = stream.try_clone()?;
  let reader = BufReader::new(stream);

  for line in reader.lines() {
    if let Ok(line) = line {
      let packet = DataPacketToServer::from_str(line.as_str());
      match packet {
        Ok(packet) => {
          handle_packet(packet, &mut echo_stream, model.clone());
        }
        Err(_) => {
          let _ = DataPacketToClient::error(String::from("Cannot resolve Data")).send(&mut echo_stream);
        }
      }
    }
  }

  Ok(())
}

fn handle_packet(packet: DataPacketToServer, stream: &mut TcpStream, model: Arc<Mutex<Model>>) {
  match packet.payload {

    PayloadToServer::Aloha => {
      let _ = DataPacketToClient::aloha().send(stream);
    }

    PayloadToServer::RegisterUser { username } => {
      let _ = user::user_registry(&username, stream, model.clone()).send(stream);
    }

  }
}

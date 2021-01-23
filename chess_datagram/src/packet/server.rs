use super::*;
use crate::payload::{PayloadToServer};
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacketToServer {
  pub payload:PayloadToServer
}

impl<'a> DataPacket<'a, PayloadToServer> for DataPacketToServer {
    fn new(payload: PayloadToServer) -> Self {
        DataPacketToServer {
          payload
        }
    }
}

impl DataPacketToServer {
  /// Receive a packet from the stream. Will block until a complete line is read.
  pub fn recv(stream: &mut TcpStream) -> Result<Self, std::io::Error> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let packet = Self::from_str(line.as_str())?;
    Ok(packet.clone())
  }

  
  pub fn aloha() -> Self {
    Self {
      payload: PayloadToServer::Aloha
    }
  }

  pub fn register_user(username: String) -> Self {
    Self {
      payload: PayloadToServer::RegisterUser { username }
    }
  }

  pub fn request_match(with: String) -> Self {
    Self {
      payload: PayloadToServer::RequestMatch{ with }
    }
  }

  pub fn accept_match(id: usize) -> Self {
    Self {
      payload: PayloadToServer::AcceptMatch { id }
    }
  }
}

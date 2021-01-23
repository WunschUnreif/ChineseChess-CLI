use super::*;
use crate::payload::{PayloadToClient};
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacketToClient {
  pub payload: PayloadToClient
}

impl<'a> DataPacket<'a, PayloadToClient> for DataPacketToClient {
    fn new(payload: PayloadToClient) -> Self {
      DataPacketToClient {
        payload
      }
    }
}

impl DataPacketToClient {
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
      payload: PayloadToClient::Aloha
    }
  }

  pub fn alive() -> Self {
    Self {
      payload: PayloadToClient::Alive
    }
  }

  pub fn error(msg: String) -> Self {
    Self {
      payload: PayloadToClient::Error { msg }
    }
  }

  pub fn success() -> Self {
    Self {
      payload: PayloadToClient::Success
    }
  }

  pub fn request_match(from: String, id: usize) -> Self {
    Self {
      payload: PayloadToClient::RequestMatch{ from, id }
    }
  }

  pub fn start_match(with: String, id: usize, is_red: bool) -> Self {
    Self {
      payload: PayloadToClient::StartMatch { with, id, is_red }
    }
  }
}

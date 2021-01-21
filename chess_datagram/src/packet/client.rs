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

  pub fn error(msg: String) -> Self {
    Self {
      payload: PayloadToClient::Error { msg }
    }
  }
}

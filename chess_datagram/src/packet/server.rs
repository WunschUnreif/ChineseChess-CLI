use super::*;
use crate::payload::{PayloadToServer};
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use chess_model::chess_move::ChessMove;

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

  pub fn exit() -> Self {
    Self {
      payload: PayloadToServer::Exit
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

  pub fn commit_move(id: usize, mov: ChessMove) -> Self {
    Self {
      payload: PayloadToServer::Move { id, mov }
    }
  }

  pub fn request_draw(id: usize) -> Self {
    Self {
      payload: PayloadToServer::RequestDraw { id }
    }
  }

  pub fn agree_draw(id: usize, accepted: bool) -> Self {
    Self {
      payload: PayloadToServer::AgreeDraw { id, accepted }
    }
  }

  pub fn request_fail(id: usize) -> Self {
    Self {
      payload: PayloadToServer::RequestFail { id }
    }
  }

  pub fn agree_fail(id: usize, accepted: bool) -> Self {
    Self {
      payload: PayloadToServer::AgreeFail { id, accepted }
    }
  }
}

use std::{net::TcpStream, time::{Duration, SystemTime}};
use chess_model::{board::ChessBoard, piece::PieceColor};
use log::{info};
use rand::prelude::*;

use chess_datagram::{DataPacketToClient, DataPacket};

use super::User;

#[derive(Debug)]
pub struct Matching {
  pub board: ChessBoard,
  pub red_player: User,
  pub black_player: User,
  pub turn: PieceColor,
  pub id: usize,
  pub active: bool,
  pub this: String,
  pub that: String,
}

impl Matching {
  pub fn new(player1: User, player2: User, id: usize) -> Matching {
    let mut rng = thread_rng();
    if rng.gen() {
      Matching {
        this: player1.username.clone(),
        that: player2.username.clone(),
        board: ChessBoard::init(),
        red_player: player1,
        black_player: player2,
        turn: PieceColor::RED,
        id,
        active: true,
      }
    } else {
      Matching {
        this: player1.username.clone(),
        that: player2.username.clone(),
        board: ChessBoard::init(),
        red_player: player2,
        black_player: player1,
        turn: PieceColor::RED,
        id,
        active: true,
      }
    }
  }

  pub fn start(&mut self) {
    let _ = DataPacketToClient::start_match(self.red_player.username.clone(), self.id, false)
      .send(&mut self.black_player.stream);
    let _ = DataPacketToClient::start_match(self.black_player.username.clone(), self.id, true)
      .send(&mut self.red_player.stream);
  }
}

#[derive(Debug)]
pub struct MatchingManager {
  pub matches: Vec<Matching>,
  pub id_allocation: usize,
}

impl MatchingManager {
  pub fn new() -> MatchingManager {
    MatchingManager {
      matches: vec![],
      id_allocation: 0
    }
  }

  pub fn new_matching(&mut self, player1: User, player2: User) -> &mut Matching {
    let matching = Matching::new(player1, player2, self.id_allocation);
    self.id_allocation += 1;
    self.matches.push(matching);
    let len = self.matches.len() - 1;
    &mut self.matches[len]
  }

  pub fn find_match_by_id(&mut self, id: usize) -> Option<&mut Matching> {
    self.matches.iter_mut().find(|x| x.id == id)
  }
}

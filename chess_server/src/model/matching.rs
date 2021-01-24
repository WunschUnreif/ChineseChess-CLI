#[allow(unused_imports)] use log::{info};

use chess_model::{board::ChessBoard, chess_move::ChessMove, piece::PieceColor};
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
  pub draw_requester: Option<User>,
  pub fail_requester: Option<User>,
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
        draw_requester: None,
        fail_requester: None,
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
        draw_requester: None,
        fail_requester: None,
      }
    }
  }

  pub fn start(&mut self) {
    let _ = DataPacketToClient::start_match(self.red_player.username.clone(), self.id, false)
      .send(&mut self.black_player.stream);
    let _ = DataPacketToClient::start_match(self.black_player.username.clone(), self.id, true)
      .send(&mut self.red_player.stream);
  }

  pub fn commit(&mut self, from: &User, mov: ChessMove) -> Result<(), String> {
    // Check the movement is from the right party
    if !self.user_is_in_turn(from) || self.turn != mov.party {
      return Err("It's not your turn, you cannot move!".into());
    }

    // Check the user is not proposing a draw/fail 
    if self.draw_requester.is_some() && self.draw_requester.as_ref().unwrap() == from {
      return Err("You can not move while proposing a draw.".into());
    }

    if self.fail_requester.is_some() && self.fail_requester.as_ref().unwrap() == from {
      return Err("You can not move while proposing a fail.".into());
    }

    // Commit to the board 
    let commit_result = mov.commit_to_board(&mut self.board);

    if commit_result.is_ok() {
      self.turn = self.turn.opponent();
      self.publish_board();

      // Clear the draw/fail proposals: make a move means reject proposal.
      self.fail_requester = None;
      self.draw_requester = None;

      Ok(())
    } else {
      Err("Invalid movement!".into())
    }
  }

  pub fn request_draw(&mut self, from: &User) -> Result<(), String> {
    // Check the movement is from the right party
    if !self.user_is_in_turn(from) {
      return Err("It's not your turn, you cannot propose a draw!".into());
    }

    // Make sure there is no other proposals 
    if self.fail_requester.is_some() || self.draw_requester.is_some() {
      return Err("You cannot make repeated requests!".into());
    }

    // Set the requester 
    self.draw_requester = Some(from.clone());

    // Notify the other 
    let _ = DataPacketToClient::request_draw(self.id).send(&mut self.get_opponent(from).stream);

    Ok(())
  }

  pub fn request_fail(&mut self, from: &User) -> Result<(), String> {
    // Check the movement is from the right party
    if !self.user_is_in_turn(from) {
      return Err("It's not your turn, you cannot propose a fail!".into());
    }

    // Make sure there is no other proposals 
    if self.fail_requester.is_some() || self.draw_requester.is_some() {
      return Err("You cannot make repeated requests!".into());
    }

    // Set the requester 
    self.fail_requester = Some(from.clone());

    // Notify the other 
    let _ = DataPacketToClient::request_fail(self.id).send(&mut self.get_opponent(from).stream);

    Ok(())
  }

  pub fn accept_draw(&mut self, from: &User, accepted: bool) -> Result<(), String> {
    // Check there is a proposal 
    if self.draw_requester.is_none() {
      return Err("There is no draw proposal!".into());
    }

    // Check the user is the opponent of the proposer
    let mut requester = self.draw_requester.as_ref().unwrap().clone();
    if self.get_opponent(&requester) != from {
      return Err("The proposal is not for you!".into());
    }

    // Deal the acceptance
    if accepted {
      let packet = DataPacketToClient::end_match("Draw".into());
      let _ = packet.send(&mut self.red_player.stream);
      let _ = packet.send(&mut self.black_player.stream);
      self.active = false;
    } else {
      let _ = DataPacketToClient::rejected().send(&mut requester.stream);
    }

    self.draw_requester = None;
    self.fail_requester = None;

    Ok(())
  }

  pub fn accept_fail(&mut self, from: &User, accepted: bool) -> Result<(), String> {
    // Check there is a proposal 
    if self.fail_requester.is_none() {
      return Err("There is no fail proposal!".into());
    }

    // Check the user is the opponent of the proposer
    let mut requester = self.fail_requester.as_ref().unwrap().clone();
    if self.get_opponent(&requester) != from {
      return Err("The proposal is not for you!".into());
    }

    // Deal the acceptance
    if accepted {
      let packet = DataPacketToClient::end_match(
        format!("Winner: {}", self.get_opponent(&requester).username)
      );
      let _ = packet.send(&mut self.red_player.stream);
      let _ = packet.send(&mut self.black_player.stream);
      self.active = false;
    } else {
      let _ = DataPacketToClient::rejected().send(&mut requester.stream);
    }

    self.draw_requester = None;
    self.fail_requester = None;

    Ok(())
  }

  fn user_is_in_turn(&self, user: &User) -> bool {
    let player = match self.turn {
      PieceColor::RED => { &self.red_player }
      PieceColor::BLACK => { &self.black_player }
    };

    player == user
  }

  fn get_opponent(&mut self, user: &User) -> &mut User {
    if self.red_player == *user {
      &mut self.black_player
    } else {
      &mut self.red_player
    }
  }

  fn publish_board(&mut self) {
    let _ = DataPacketToClient::new_chessboard(
      self.board.clone(), 
      self.turn == PieceColor::RED
    ).send(&mut self.red_player.stream);

    let _ = DataPacketToClient::new_chessboard(
      self.board.clone(), 
      self.turn == PieceColor::BLACK
    ).send(&mut self.black_player.stream);
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

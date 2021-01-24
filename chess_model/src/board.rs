pub mod default_board;
pub mod board_position;
pub mod initial_positions;

use crate::piece;
use console::Term;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChessBoard {
  // 9 columns, each is a col with 10 rows,
  // columns start from right, rows start from bottom, red facing lower rows
  pub configuration: [[Option<piece::Piece>; 10]; 9]
}

impl ChessBoard {

  /// Render the chess board.
  pub fn show(&self, term: &mut Term, for_red: bool) {
    default_board::show_base_chessboard(term);
    for col in 0..9 {
      for row in 0..10 {
        if let Some(piece) = self.configuration[col][row] {
          let (disp_x, disp_y) = board_position::BoardPosition::from((row + 1) as i8, (col + 1) as i8)
            .to_display_position(for_red);

          let _ = term.move_cursor_to(disp_x, disp_y);
          let _ = term.write(piece.get_styled_name().as_bytes());
        }
      }
    }
    let _ = term.move_cursor_to(0, 0);
  }

  /// Commit a movement
  pub fn commit(&mut self, from: board_position::BoardPosition, to: board_position::BoardPosition) {
    if !from.is_valid_position() || !to.is_valid_position() {
      panic!("Invalid positions");
    }

    if !self.has_piece_at(from) {
      panic!("Cannot commit a movement from position where no piece lies");
    }

    self.clear_just_moved();

    let (col, row) = from.to_configuration_index();
    let piece_from = self.configuration[col][row].unwrap();

    let (col, row) = to.to_configuration_index();
    self.configuration[col][row] = Some(piece_from.clone());

    let (col, row) = from.to_configuration_index();
    let piece_from = self.configuration[col][row].as_mut().unwrap();
    piece_from.state = piece::PieceState::JustMoved;
    
    let (col, row) = to.to_configuration_index();
    let piece_to = self.configuration[col][row].as_mut().unwrap();
    piece_to.state = piece::PieceState::Fresh;
  }

  /// Test whether there is a piece at the given position
  pub fn has_piece_at(&self, pos: board_position::BoardPosition) -> bool {
    self.configuration[(pos.col - 1) as usize][(pos.row - 1) as usize].is_some()
    && self.configuration[(pos.col - 1) as usize][(pos.row - 1) as usize].unwrap().state == piece::PieceState::Active
  }

  pub fn get_piece_at(&self, pos: board_position::BoardPosition) -> Option<piece::Piece> {
    let (col, row) = pos.to_configuration_index();
    self.configuration[col][row]
  }

  /// Clear the pieces just moved
  fn clear_just_moved(&mut self) {
    for col in 0..9 {
      for row in 0..10 {
        if let Some(piece) = self.configuration[col][row].as_mut() {
          if piece.state == piece::PieceState::JustMoved {
            self.configuration[col][row] = None;
          } else if piece.state == piece::PieceState::Fresh {
            piece.state = piece::PieceState::Active;
          }
        }
      }
    }
  }

  /// Convert to json string. 
  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

}

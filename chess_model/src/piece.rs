use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

/// Color of chess pieces
#[derive(Debug, EnumIter, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum PieceColor {
  RED,
  BLACK
}

impl PieceColor {
  /// Get the opponent's color 
  pub fn opponent(&self) -> PieceColor {
    match *self {
      PieceColor::RED => PieceColor::BLACK,
      PieceColor::BLACK => PieceColor::RED
    }
  }
}

/// State of chess pieces
#[derive(Debug, EnumIter, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum PieceState {
  /// The piece is in active
  Active,
  /// The piece was moved out in the last configuration
  JustMoved,
  /// The piece was moved in in the last configuration
  Fresh
}

/// Kind of chess pieces
#[derive(Debug, EnumIter, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum PieceKind {
  BING,
  PAO,
  JU,
  MA,
  XIANG,
  SHI,
  JIANG
}

/// Chess piece
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Piece {
  pub kind: PieceKind,
  pub color: PieceColor,
  pub state: PieceState
}

pub mod name;



#[cfg(test)]
mod piece_test {
  use super::*;

  #[test]
  fn piece_construct() {
    let piece = Piece {
      kind: PieceKind::JIANG,
      color: PieceColor::RED,
      state: PieceState::Active
    };

    println!("{:?}", &piece)
  }
}

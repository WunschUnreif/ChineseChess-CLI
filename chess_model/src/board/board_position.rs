use std::ops::{Add};
use crate::piece::PieceColor;

/// Position of pieces in the board
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardPosition {
  pub row: i8,    // row counts from bottom, starting from 1, the red party is at lower rows
  pub col: i8     // col counts from right, starting from 1
}

/// Position difference (row, col)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardPositionDiff(pub i8, pub i8);

impl BoardPositionDiff {
  /// Get the symmetrical difference, usually for converting a diff for the black party.
  pub fn symmetry(&self) -> BoardPositionDiff {
    BoardPositionDiff(-self.0, -self.1)
  }
}

impl BoardPosition {
  pub fn from(row: i8, col: i8) -> BoardPosition {
    BoardPosition { row, col }
  }

  /// Get the symmetry position, used to converting between red and black
  pub fn symmetry(&self) -> BoardPosition {
    BoardPosition {
      row: 11 - self.row,
      col: 10 - self.col
    }
  }

  /// Return the index in the configuration array, (col, row).
  /// 
  /// # Usage:
  /// ```
  /// use chess_model::board::board_position::BoardPosition;
  /// // let board: ChessBoard;
  /// let pos = BoardPosition::from(3, 2);
  /// let (col, row) = pos.to_configuration_index();
  /// // let _ = board.configuration[col][row];
  /// ```
  pub fn to_configuration_index(&self) -> (usize, usize) {
    self.panic_if_invalid("Cannot convert to config index for invalid positions");
    ((self.col - 1) as usize, (self.row - 1) as usize)
  }

  /// Return the canonical position: red at the bottom.
  pub fn to_canonical_position(&self, color: PieceColor) -> BoardPosition {
    if color == PieceColor::BLACK {
      self.symmetry()
    } else {
      self.clone()
    }
  }

  /// Test whether the position is inside the chessboard.
  pub fn is_valid_position(&self) -> bool {
    self.row > 0 && self.row <= 10 && self.col > 0 && self.col <= 9
  }

  /// return the corresponding position on the screen for rendering
  /// -> (x, y): x axis to the right, y axis to the bottom
  pub fn to_display_position(&self, red_bottom: bool) -> (usize, usize) {
    if !self.is_valid_position() {
      panic!("Cannot convert to display position for invalid positions");
    }

    if red_bottom {
      (((9 - self.col) * 5) as usize, (1 + (10 - self.row) * 2) as usize)
    } else {
      BoardPosition::from(11 - self.row, 10 - self.col).to_display_position(true)
    }
  }

  fn panic_if_invalid(&self, msg: &'static str) {
    if !self.is_valid_position() {
      panic!(msg);
    }
  }
}

impl Add<BoardPositionDiff> for BoardPosition {
  type Output = BoardPosition;

  fn add(self, other: BoardPositionDiff) -> BoardPosition {
    BoardPosition::from(self.row + other.0, self.col + other.1)
  }
}



#[test]
fn test_valid_position() {
    assert_eq!(BoardPosition::from(-3, 8).is_valid_position(), false);
    assert_eq!(BoardPosition::from(-3, -8).is_valid_position(), false);
    assert_eq!(BoardPosition::from(10, 10).is_valid_position(), false);
    assert_eq!(BoardPosition::from(0, 0).is_valid_position(), false);
    assert_eq!(BoardPosition::from(3, 6).is_valid_position(), true);
}

#[test]
fn test_position_add() {
    assert_eq!(BoardPosition::from(3, 6) + BoardPositionDiff(-1, -2), BoardPosition::from(2, 4));
}

#[test]
fn test_display_position() {
    let position = BoardPosition::from(8, 2);

    let (x, y) = position.to_display_position(false);
    assert_eq!((x, y), (5, 15));

    let (x, y) = position.to_display_position(true);
    assert_eq!((x, y), (35, 5));
}

#[test]
fn test_pos_symmetry() {
    let pos = BoardPosition::from(3, 2);

    assert_eq!(
      pos.symmetry(),
      BoardPosition::from(8, 8)
    );
}

#[test]
fn test_config_index() {
  let pos = BoardPosition::from(3, 2);

  assert_eq!(
    pos.to_configuration_index(),
    (1, 2)
  );

  assert_eq!(
    pos.symmetry().to_configuration_index(),
    (7, 7)
  )
}
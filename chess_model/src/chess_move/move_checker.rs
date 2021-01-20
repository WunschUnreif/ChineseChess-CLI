use crate::piece::*;
use crate::chess_move::ChessMove;
use crate::board::{ChessBoard, board_position::*};
use std::cmp::Ordering;

mod move_target;
use move_target::*;


/// Test whether a movement can happen, given a valid origin position.
/// Assuming `movement` and `from` are all canonical.
pub fn can_move(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  if !board.has_piece_at(from) 
      || board.get_piece_at(from).unwrap().color != movement.party
      || board.get_piece_at(from).unwrap().kind  != movement.kind {
    panic!("Position and movement mismatched!");
  }

  println!("{:?}", movement);

  match movement.kind {
    PieceKind::BING  => can_move_bing(board, movement, from),
    PieceKind::PAO   => can_move_pao(board, movement, from),
    PieceKind::JU    => can_move_ju(board, movement, from),
    PieceKind::MA    => can_move_ma(board, movement, from),
    PieceKind::XIANG => can_move_xiang(board, movement, from),
    PieceKind::SHI   => can_move_shi(board, movement, from),
    PieceKind::JIANG => can_move_jiang(board, movement, from),
  }
}

pub fn get_move_target(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  if !board.has_piece_at(from) 
      || board.get_piece_at(from).unwrap().color != movement.party
      || board.get_piece_at(from).unwrap().kind  != movement.kind {
    panic!("Position and movement mismatched!");
  }

  match movement.kind {
    PieceKind::BING  => get_bing_target(movement, from),
    PieceKind::PAO   => get_ju_pao_target(movement, from),
    PieceKind::JU    => get_ju_pao_target(movement, from),
    PieceKind::MA    => get_ma_target(movement, from),
    PieceKind::XIANG => get_xiang_target(movement, from),
    PieceKind::SHI   => get_shi_target(movement, from),
    PieceKind::JIANG => get_jiang_target(movement, from),
  }
}

fn count_piece_in_middle(from: BoardPosition, to: BoardPosition, board: &ChessBoard) -> i32 {
  let mut count = 0;

  let drow = match from.row.cmp(&to.row) {
    Ordering::Less => 1,
    Ordering::Equal => 0,
    Ordering::Greater => -1
  };
  let dcol = match from.col.cmp(&to.col) {
    Ordering::Less => 1,
    Ordering::Equal => 0,
    Ordering::Greater => -1
  };

  let mut r = from.row + drow;
  let mut c = from.col + dcol;

  while r != to.row || c != to.col {
    if board.has_piece_at(BoardPosition::from(r, c)) {
      count += 1;
    }
    r += drow;
    c += dcol;
  }

  count
}

/// Assuming `movement` and `from` are all canonical.
fn can_move_bing(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  if let Some(target) = get_bing_target(movement, from) {
    is_valid_target(board, target, movement.party)
  } else {
    false
  }
}

fn can_move_pao(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  if let Some(target) = get_ju_pao_target(movement, from) {
    if !is_valid_target(board, target, movement.party) { return false; }

    if board.has_piece_at(target) {
      count_piece_in_middle(from, target, board) == 1
    } else {
      count_piece_in_middle(from, target, board) == 0
    }
  } else {
    false
  }
}

fn can_move_ju(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  if let Some(target) = get_ju_pao_target(movement, from) {
    if !is_valid_target(board, target, movement.party) { return false; }
    count_piece_in_middle(from, target, board) == 0
  } else {
    false
  }
}

fn can_move_ma(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  let target = get_ma_target(movement, from);
  let leg_pos = get_ma_leg_pos(movement, from);

  println!("{:?}", target);


  if target.is_none() || target.is_none() {
    false
  } else {
    is_valid_target(board, target.unwrap(), movement.party)
      && !board.has_piece_at(leg_pos.unwrap())
  }
}

fn is_valid_xiang_target(board: &ChessBoard, target: BoardPosition, party: PieceColor) -> bool {
  if !is_valid_target(board, target.clone(), party.clone()) {
    return false;
  }

  if party == PieceColor::BLACK {
    target.row >= 6 && (target.col - 1) % 2 == 0
  } else {
    target.row <= 5 && (target.col - 1) % 2 == 0
  }
}

fn can_move_xiang(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  let target = get_xiang_target(movement, from);

  if let Some(target) = target {
    is_valid_xiang_target(board, target, movement.party) && count_piece_in_middle(from, target, board) == 0
  } else {
    false
  }

}

fn is_valid_shi_target(board: &ChessBoard, target: BoardPosition, party: PieceColor) -> bool {
  if !is_valid_target(board, target.clone(), party.clone()) {
    return false;
  }

  if party == PieceColor::BLACK {
    ((target.row == 8 || target.row == 10) && (target.col == 4 || target.col == 6))
      || (target.row == 9 && target.col == 5)
  } else {
    ((target.row == 1 || target.row == 3) && (target.col == 4 || target.col == 6))
      || (target.row == 2 && target.col == 5)
  }
}

fn can_move_shi(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  let target = get_shi_target(movement, from);
  if let Some(target) = target {
    is_valid_shi_target(board, target, movement.party)
  } else {
    false
  }
}

fn is_valid_jiang_target(board: &ChessBoard, target: BoardPosition, party: PieceColor) -> bool {
  if !is_valid_target(board, target.clone(), party.clone()) {
    return false;
  }

  if party == PieceColor::BLACK {
    target.row >= 8 && target.col >= 4 && target.col <= 6
  } else {
    target.row <= 3 && target.col >= 4 && target.col <= 6
  }
}

fn can_move_jiang(board: &ChessBoard, movement: &ChessMove, from: BoardPosition) -> bool {
  let target = get_jiang_target(movement, from);
  if let Some(target) = target {
    is_valid_jiang_target(board, target, movement.party)
  } else {
    false
  }
}


#[cfg(test)]
mod tests;

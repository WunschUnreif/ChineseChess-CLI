use crate::chess_move::ChessMove;
use crate::board::{ChessBoard, board_position::*};

pub fn find_move_piece_pos(board: &ChessBoard, movement: ChessMove) -> Result<BoardPosition, ()> {
  if movement.col.is_some() && movement.depth.is_none() {
    find_by_col(board, movement)
  } else if movement.col.is_none() && movement.depth.is_some() {
    find_by_depth(board, movement)
  } else if movement.col.is_some() && movement.depth.is_some() {
    find_by_col(board, movement)
  } else {
    Err(())
  }
}

fn find_by_col(board: &ChessBoard, movement: ChessMove) -> Result<BoardPosition, ()> {
  let col = movement.col.unwrap();
  let target_kind = movement.kind;
  let target_color = movement.party;

  let mut count = 0;
  let mut pos = BoardPosition::from(0, 0);

  for row in 1..=10 {
    if let Some(piece) = board.get_piece_at(BoardPosition::from(row, col)) {
      if piece.kind == target_kind && piece.color == target_color {
        count += 1;
        pos = BoardPosition::from(row, col);
      }
    }
  }

  if count == 1 {
    Ok(pos)
  } else {
    Err(())
  }
}

fn find_by_col_and_depth(board: &ChessBoard, movement: ChessMove) -> Result<BoardPosition, ()> {
  let col = movement.col.unwrap();
  let target_kind = movement.kind;
  let target_color = movement.party;

  let mut positions: Vec<BoardPosition> = Vec::new();

  let range;
  if movement.depth.unwrap() < 0 {
    range = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  } else {
    range = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
  }

  for row in range {
    if let Some(piece) = board.get_piece_at(BoardPosition::from(row, col)) {
      if piece.kind == target_kind && piece.color == target_color {
        let pos = BoardPosition::from(row, col);
        positions.push(pos);
      }
    }
  }

  let depth = if movement.depth.unwrap() < 0 { -movement.depth.unwrap() } else { movement.depth.unwrap() };

  if depth as usize <= positions.len() {
    Ok(positions[(depth - 1) as usize])
  } else {
    Err(())
  }
}

fn find_by_depth(board: &ChessBoard, movement: ChessMove) -> Result<BoardPosition, ()> {
  let mut count = 0;
  let mut pos = BoardPosition::from(0, 0);

  for col in 1..=9 {
    let new_movement = ChessMove {
      col: Some(col),
      ..movement
    };

    let result = find_by_col_and_depth(board, new_movement);
    if result.is_ok() {
      count += 1;
      pos = result.unwrap();
    }
  }

  if count == 1 {
    Ok(pos)
  } else {
    Err(())
  }
}


#[test]
fn test_piece_finder() {
  use crate::board::*;
  use crate::chess_move::move_parser;
  use crate::piece::*;

  let board = ChessBoard::init();

  assert_eq!(
    find_move_piece_pos(&board, move_parser::parse_move_str("j1+1", PieceColor::RED).unwrap()).unwrap(),
    BoardPosition::from(1, 1)
  );

  assert_eq!(
    find_move_piece_pos(&board, move_parser::parse_move_str("j1+1", PieceColor::BLACK).unwrap().to_canonical()).unwrap(),
    BoardPosition::from(10, 9)
  );
}

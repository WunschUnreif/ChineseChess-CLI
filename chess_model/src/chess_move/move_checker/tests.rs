use super::*;

#[test]
fn test_piece_move_1() {
  use crate::board::*;
  use crate::chess_move::move_parser;

  let board = ChessBoard::init();

  assert_eq!(
    can_move_pao(
      &board, 
      &move_parser::parse_move_str("p8~5", PieceColor::RED).unwrap(), 
      BoardPosition::from(3, 8)), 
    true
  );

  assert_eq!(
    can_move_bing(
      &board, 
      &move_parser::parse_move_str("b5+1", PieceColor::RED).unwrap(), 
      BoardPosition::from(4, 5)), 
    true
  );

  assert_eq!(
    can_move_bing(
      &board, 
      &move_parser::parse_move_str("b5~4", PieceColor::RED).unwrap(), 
      BoardPosition::from(4, 5)), 
    false
  );

  assert_eq!(
    can_move_bing(
      &board, 
      &move_parser::parse_move_str("b5~4", PieceColor::RED).unwrap(), 
      BoardPosition::from(6, 5)), 
    true
  );

  assert_eq!(
    can_move_ju(
      &board, 
      &move_parser::parse_move_str("j1+2", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 1)), 
    true
  );

  assert_eq!(
    can_move_ju(
      &board, 
      &move_parser::parse_move_str("j1+3", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 1)), 
    false
  );
}

#[test]
fn test_piece_move_2() {
  use crate::board::*;
  use crate::chess_move::move_parser;

  let board = ChessBoard::init();

  assert_eq!(
    can_move_ma(
      &board, 
      &move_parser::parse_move_str("m2+3", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 2)), 
    true
  );

  assert_eq!(
    can_move_ma(
      &board, 
      &move_parser::parse_move_str("m2+1", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 2)), 
    true
  );

  assert_eq!(
    can_move_ma(
      &board, 
      &move_parser::parse_move_str("m2+4", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 2)), 
    false
  );
}

#[test]
fn test_piece_move_3() {
  use crate::board::*;
  use crate::chess_move::move_parser;

  let board = ChessBoard::init();

  assert_eq!(
    can_move_xiang(
      &board, 
      &move_parser::parse_move_str("x3+5", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 3)), 
    true
  );

  assert_eq!(
    can_move_xiang(
      &board, 
      &move_parser::parse_move_str("x7+9", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 7)), 
    true
  );

  assert_eq!(
    can_move_xiang(
      &board, 
      &move_parser::parse_move_str("x5+7", PieceColor::RED).unwrap(), 
      BoardPosition::from(3, 5)),
    true
  );

  assert_eq!(
    can_move_shi(
      &board, 
      &move_parser::parse_move_str("s4+5", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 4)),
    true
  );

  assert_eq!(
    can_move_shi(
      &board, 
      &move_parser::parse_move_str("s6+5", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 4)),
    true
  );

  assert_eq!(
    can_move_shi(
      &board, 
      &move_parser::parse_move_str("s4+3", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 4)),
    false
  );
}

#[test]
fn test_piece_move_4() {
  use crate::board::*;
  use crate::chess_move::move_parser;

  let board = ChessBoard::init();

  assert_eq!(
    can_move_jiang(
      &board, 
      &move_parser::parse_move_str("k5+1", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 5)), 
    true
  );

  assert_eq!(
    can_move_jiang(
      &board, 
      &move_parser::parse_move_str("k5-1", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 5)), 
    false
  );

  assert_eq!(
    can_move_jiang(
      &board, 
      &move_parser::parse_move_str("k5~4", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 5)), 
    false
  );

  assert_eq!(
    can_move_jiang(
      &board, 
      &move_parser::parse_move_str("k4~3", PieceColor::RED).unwrap(), 
      BoardPosition::from(1, 5)), 
    false
  );
}
use crate::piece::*;
use crate::chess_move::ChessMove;
use crate::board::{ChessBoard, board_position::*};
// use std::cmp::Ordering;

pub fn is_valid_target(board: &ChessBoard, target: BoardPosition, party: PieceColor) -> bool {
  if !target.is_valid_position() {
    return false;
  }

  let (col, row) = target.to_configuration_index();
  if let Some(piece) = board.configuration[col][row] {
    if piece.color == party {
      return false;
    }
  }

  true
}

pub fn get_bing_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  if movement.advancing {
    if movement.target_or_diff != 1 && movement.target_or_diff != -1 { return None; }

    let diff = BoardPositionDiff(movement.target_or_diff, 0);

    target = from + diff;
  } else {
    if from.row <= 5 { return None; }

    let col_diff = from.col - movement.target_or_diff;
    if col_diff != -1 && col_diff != 1 {
      return None;
    }

    target = BoardPosition { row: from.row, col: movement.target_or_diff };
  }

  Some(target)
}

pub fn get_ju_pao_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  if movement.advancing {
    let diff = BoardPositionDiff(movement.target_or_diff, 0);
    target = from + diff;
  } else {
    target = BoardPosition { row: from.row, col: movement.target_or_diff };
  }

  Some(target)
}

pub fn get_ma_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  let advancing = if movement.target_or_diff > 0 { true } else { false };

  let col_diff = if movement.target_or_diff > 0 {
    movement.target_or_diff - from.col
  } else {
    -movement.target_or_diff - from.col
  };

  if advancing {
    match col_diff {
      1 => {
        target  = BoardPosition { row: from.row + 2, col: from.col + 1 };
      },
      2 => {
        target  = BoardPosition { row: from.row + 1, col: from.col + 2 };
      },
      -1 => {
        target  = BoardPosition { row: from.row + 2, col: from.col - 1 };
      },
      -2 => {
        target  = BoardPosition { row: from.row + 1, col: from.col - 2 };
      },
      _ => return None
    }
  } else {
    match col_diff {
      1 => {
        target  = BoardPosition { row: from.row - 2, col: from.col + 1 };
      },
      2 => {
        target  = BoardPosition { row: from.row - 1, col: from.col + 2 };
      },
      -1 => {
        target  = BoardPosition { row: from.row - 2, col: from.col - 1 };
      },
      -2 => {
        target  = BoardPosition { row: from.row - 1, col: from.col - 2 };
      },
      _ => return None
    }
  }

  Some(target)
}

pub fn get_ma_leg_pos(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let leg_pos: BoardPosition;

  let advancing = if movement.target_or_diff > 0 { true } else { false };

  let col_diff = if movement.target_or_diff > 0 {
    movement.target_or_diff - from.col
  } else {
    -movement.target_or_diff - from.col
  };

  if advancing {
    match col_diff {
      1 => {
        leg_pos = BoardPosition { row: from.row + 1, col: from.col + 0 };
      },
      2 => {
        leg_pos = BoardPosition { row: from.row + 0, col: from.col + 1 };
      },
      -1 => {
        leg_pos = BoardPosition { row: from.row + 1, col: from.col - 0 };
      },
      -2 => {
        leg_pos = BoardPosition { row: from.row + 0, col: from.col - 1 };
      },
      _ => return None
    }
  } else {
    match col_diff {
      1 => {
        leg_pos = BoardPosition { row: from.row - 1, col: from.col + 0 };
      },
      2 => {
        leg_pos = BoardPosition { row: from.row - 0, col: from.col + 1 };
      },
      -1 => {
        leg_pos = BoardPosition { row: from.row - 1, col: from.col - 0 };
      },
      -2 => {
        leg_pos = BoardPosition { row: from.row - 0, col: from.col - 1 };
      },
      _ => return None
    }
  }

  Some(leg_pos)
}

pub fn get_xiang_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  let advancing = if movement.target_or_diff > 0 { true } else { false };

  let col_diff = if movement.target_or_diff > 0 {
    movement.target_or_diff - from.col
  } else {
    -movement.target_or_diff - from.col
  };

  if advancing {
    match col_diff {
      2 => {
        target  = BoardPosition { row: from.row + 2, col: from.col + 2 };
      },
      -2 => {
        target  = BoardPosition { row: from.row + 2, col: from.col - 2 };
      },
      _ => return None
    }
  } else {
    match col_diff {
      2 => {
        target  = BoardPosition { row: from.row - 2, col: from.col + 2 };
      },
      -2 => {
        target  = BoardPosition { row: from.row - 2, col: from.col - 2 };
      },
      _ => return None
    }
  }

  Some(target)
}

pub fn get_shi_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  let drow = if movement.target_or_diff > 0 { 1 } else { -1 };

  let col_diff = if movement.target_or_diff > 0 {
    movement.target_or_diff - from.col
  } else {
    -movement.target_or_diff - from.col
  };

  if col_diff != 1 && col_diff != -1 { return None; }

  target  = BoardPosition { row: from.row + drow, col: from.col + col_diff };

  Some(target)
}

pub fn get_jiang_target(movement: &ChessMove, from: BoardPosition) -> Option<BoardPosition> {
  let target: BoardPosition;

  if movement.advancing {
    match movement.target_or_diff {
      1  => target = from + BoardPositionDiff(1, 0),
      -1 => target = from + BoardPositionDiff(-1, 0),
      _ => return None
    }
  } else {
    let col_diff = from.col - movement.target_or_diff;
    if col_diff != 1 && col_diff != -1 { return None; }

    target = BoardPosition::from(from.row, movement.target_or_diff);
  }

  Some(target)
}

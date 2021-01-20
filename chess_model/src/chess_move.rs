use crate::piece::*;
use crate::board::{ChessBoard};
use serde::{Serialize, Deserialize};

pub mod move_parser;
pub mod move_checker;
pub mod piece_finder;


/// Describing the movement in a round
/// 
/// # Examples:
/// 
/// 1. "红马八进七"
///     ```
///     use chess_model::chess_move::ChessMove;
///     use chess_model::piece::*;
///     let mov = ChessMove {
///       party: PieceColor::RED,
///       kind: PieceKind::MA,
///       depth: None,
///       col: Some(8),
///       advancing: true,
///       target_or_diff: 7
///     };
///     ```
/// 
/// 2. "黑前车退4"
///     ```
///     use chess_model::chess_move::ChessMove;
///     use chess_model::piece::*;
///     let mov = ChessMove {
///       party: PieceColor::BLACK,
///       kind: PieceKind::JU,
///       depth: Some(1),
///       col: None,
///       advancing: false,
///       target_or_diff: -4
///     };
///     ```
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChessMove {
  /// red or black
  pub party: PieceColor,

  /// piece kind
  pub kind: PieceKind,

  /// the depth in the column, positive => counts from top; negative => counts from bottom
  /// start from 1
  pub depth: Option<i8>,

  /// the column for piece, always in the view of the `party`
  pub col: Option<i8>,

  /// true ==> '进'; flase ==> '平'
  pub advancing: bool,

  /// the target of flat move when `advancing == false`, or
  /// the difference of advancing when otherwise, in this case negative values allowed
  pub target_or_diff: i8
}

impl ChessMove {
  /// Test whether the move is valid with only basic checks (leave the configuration)
  pub fn basically_valid(&self) -> bool {
    // 至少指定 前后 或 路数 之一
    !(self.depth.is_none() && self.col.is_none())
      // 路数在1～9之间
      &&  implies!(self.col.is_some() => self.col.unwrap() > 0 && self.col.unwrap() <= 9)
      // 同一路上，兵不超过5个，将不超过1个，其他不超过2个
      &&  implies!(self.depth.is_some() => match self.kind {
            PieceKind::BING  => self.depth.unwrap() > 0 && self.depth.unwrap() <= 5,
            PieceKind::JIANG => self.depth.unwrap() == 1,
            _ => self.depth.unwrap() > 0 && self.depth.unwrap() <= 2
          })
      // “进”的情况，前后最多移动9步
      &&  implies!(self.advancing => 
            self.target_or_diff != 0 && self.target_or_diff <= 9 && self.target_or_diff >= -9
          )
      // “平”的情况，不能平到原路，且目标路数在1～9之间
      &&  implies!(!self.advancing => 
            self.target_or_diff > 0 && self.target_or_diff <= 9
              && implies!(self.col.is_some() => self.target_or_diff != self.col.unwrap())
      )
      // 马、相、士只能进
      &&  implies!(
            self.kind == PieceKind::MA || self.kind == PieceKind::XIANG || self.kind == PieceKind::SHI => 
            self.advancing == true
      )
  }

  pub fn commit_to_board(&self, board: &mut ChessBoard) -> Result<(), ()> {
    use piece_finder::find_move_piece_pos;
    use move_checker::{can_move, get_move_target};

    let canonical = self.to_canonical();

    let piece_pos = find_move_piece_pos(board, canonical.clone());
    if piece_pos.is_err() { return Err(()); }

    if !can_move(board, &canonical, piece_pos.unwrap()) { return Err(()); }

    let target = get_move_target(board, &canonical, piece_pos.unwrap()).unwrap();
    board.commit(piece_pos.unwrap(), target);

    Ok(())
  }

  pub fn to_canonical(&self) -> ChessMove {
    if self.party == PieceColor::RED {
      self.clone()
    } else {
      ChessMove {
        depth: match self.depth {
          Some(d) => Some(-d),
          None => None
        },

        col: match self.col {
          Some(c) => Some(10 - c),
          None => None 
        },

        target_or_diff: match self.advancing {
          true => match self.kind {
            PieceKind::MA | PieceKind::SHI | PieceKind::XIANG => -(10 - self.target_or_diff),
            _ => -self.target_or_diff
          },
          false => 10 - self.target_or_diff
        },

        ..self.clone()
      }
    }
  }

  /// Convert to json string. 
  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
}



#[test]
fn test_implies() {
    assert_eq!(
      implies!(3 > 2 => 2 > 1),
      true
    );

    assert_eq!(
      implies!(3 > 2 => 2 < 1),
      false
    );

    assert_eq!(
      implies!(3 < 2 => 2 > 1),
      true
    );

    assert_eq!(
      implies!(3 < 2 => 2 < 1),
      true
    );
}

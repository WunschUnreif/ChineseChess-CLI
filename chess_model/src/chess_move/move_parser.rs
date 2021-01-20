use crate::chess_move::ChessMove;
use crate::piece::*;
use regex::Regex;

/// Parse a string of movement to ChessMove object
/// 
/// # Examples
/// 1. "马2进3": `m2+3`
/// 2. "兵3平2": `b3~2`
/// 3. "前车退4": `1j-4`
/// 4. "后车平6": `2j~6`
/// 
pub fn parse_move_str(move_str: &str, party: PieceColor) -> Result<ChessMove, ()> {
  lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"(?x)
      ^
      (?P<depth>\d)?
      (?P<kind>[jmxskpbzJMXSKPBZ])
      (?P<col>\d)?
      (?P<type>[+-~])
      (?P<target>\d)
      $
    ").unwrap();
  };

  let cap = MOVE_RE.captures(move_str);
  if cap.is_none() {
    return Err(())
  }

  let cap = cap.unwrap();

  let chess_move = ChessMove {
    party,

    kind: convert_kind_str(cap.name("kind").unwrap().as_str()),

    advancing: !(cap.name("type").unwrap().as_str() == "~"),

    target_or_diff: if cap.name("type").unwrap().as_str() == "-" {
      -cap.name("target").unwrap().as_str().parse::<i8>().unwrap()
    } else {
      cap.name("target").unwrap().as_str().parse::<i8>().unwrap()
    },

    col: if cap.name("col").is_none() {
      None
    } else {
      Some(cap.name("col").unwrap().as_str().parse::<i8>().unwrap())
    },

    depth: if cap.name("depth").is_none() {
      None
    } else {
      Some(cap.name("depth").unwrap().as_str().parse::<i8>().unwrap())
    }
  };

  if chess_move.basically_valid() {
    Ok(chess_move)
  } else {
    Err(())
  }
}

fn convert_kind_str(k: &str) -> PieceKind {
  match k.to_lowercase().as_str() {
    "j" => PieceKind::JU,
    "m" => PieceKind::MA,
    "x" => PieceKind::XIANG,
    "s" => PieceKind::SHI,
    "k" => PieceKind::JIANG,
    "p" => PieceKind::PAO,
    "b" | "z" => PieceKind::BING,
    _ => panic!("Invalid kind string in parsing")
  }
}



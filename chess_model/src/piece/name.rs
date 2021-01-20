use super::*;
use console::style;

impl PieceKind {
  fn get_name(&self, color: &PieceColor) -> char {
    match color {
      PieceColor::RED => match self {
        PieceKind::BING => '兵',
        PieceKind::PAO => '炮',
        PieceKind::JU => '车',
        PieceKind::MA => '马',
        PieceKind::XIANG => '相',
        PieceKind::SHI => '士',
        PieceKind::JIANG => '帅',
      },
      PieceColor::BLACK => match self {
        PieceKind::BING => '卒',
        PieceKind::PAO => '砲',
        PieceKind::JU => '車',
        PieceKind::MA => '馬',
        PieceKind::XIANG => '象',
        PieceKind::SHI => '仕',
        PieceKind::JIANG => '将',
      }
    }
  }

  fn get_styled_name(&self, color: &PieceColor) -> String {
    match color {
      PieceColor::RED => format!("{}", style(self.get_name(color)).on_color256(88)),
      PieceColor::BLACK => format!("{}", style(self.get_name(color)).on_color256(22)),
    }
  }
}

impl Piece {
  pub fn get_styled_name(&self) -> String {
    match self.state {
      PieceState::JustMoved => format!("{}", style(self.kind.get_name(&self.color)).dim()),
      PieceState::Fresh => format!("{}", style(self.kind.get_styled_name(&self.color)).blink()),
      PieceState::Active => self.kind.get_styled_name(&self.color)
    }
  }
}


#[cfg(test)]
mod test {
  use strum::IntoEnumIterator;
  use super::*;
  
  #[test]
  fn test_get_name() {
      for color in PieceColor::iter() {
        for kind in PieceKind::iter() {
          print!("{}", kind.get_name(&color));
        }
        println!("");
      }
      for color in PieceColor::iter() {
        for kind in PieceKind::iter() {
          print!("{} ", kind.get_styled_name(&color));
        }
        println!("");
      }
      for _ in 0..7 {
        print!("xxx");
      }
      println!("");
  }

  #[test]
  fn test_piece_get_styled_name() {
      let mut piece = Piece {
        color: PieceColor::RED,
        kind: PieceKind::MA,
        state: PieceState::Active
      };

      assert_eq!(
        piece.get_styled_name(),
        PieceKind::MA.get_styled_name(&PieceColor::RED)
      );

      println!("{}", piece.get_styled_name());

      piece.state = PieceState::JustMoved;
      println!("{}", piece.get_styled_name());
  }
}
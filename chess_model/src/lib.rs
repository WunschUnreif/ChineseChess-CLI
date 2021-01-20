#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

macro_rules! implies {
  ($cond: expr => $res: expr) => { if $cond { $res } else { true } };
}

pub mod piece;
pub mod board;
pub mod chess_move;


#[cfg(test)]
mod tests {
  #[test]
  fn main_test() {
      assert_eq!(2 + 2, 4);
  }
}

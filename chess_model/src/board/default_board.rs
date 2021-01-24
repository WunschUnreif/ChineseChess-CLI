use console::{Term};

pub const DEFAULT_CHESSBOARD: &'static str = "\
1    2    3    4    5    6    7    8    9
┏━━━━┳━━━━┳━━━━┳━━━━╦━━━━┳━━━━┳━━━━┳━━━━┓
┃    ┃    ┃    ┃  ╲ ┃ ╱  ┃    ┃    ┃    ┃
┣━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━┫
┃    ┃    ┃    ┃  ╱ ┃ ╲  ┃    ┃    ┃    ┃
┣━━━━╬━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╬━━━━┫
┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃
╠━━━━╋━━━━╬━━━━╋━━━━╬━━━━╋━━━━╬━━━━╋━━━━╣
┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃
┣━━━━┻━━━━┻━━━━┻━━━━┻━━━━┻━━━━┻━━━━┻━━━━┫
┃     楚     河           汉      界    ┃
┣━━━━┳━━━━┳━━━━┳━━━━┳━━━━┳━━━━┳━━━━┳━━━━┫
┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃
╠━━━━╋━━━━╬━━━━╋━━━━╬━━━━╋━━━━╬━━━━╋━━━━╣
┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃    ┃
┣━━━━╬━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╬━━━━┫
┃    ┃    ┃    ┃  ╲ ┃ ╱  ┃    ┃    ┃    ┃
┣━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━╋━━━━┫
┃    ┃    ┃    ┃  ╱ ┃ ╲  ┃    ┃    ┃    ┃
┣━━━━┻━━━━┻━━━━┻━━━━╩━━━━┻━━━━┻━━━━┻━━━━┫
9    8    7    6    5    4    3    2    1
";


/// Draw the chess board on the screen, after that, move cursor to the (0, 0) position
pub fn show_base_chessboard(term: &Term) {
  let _ = term.clear_screen();
  for (i, line) in DEFAULT_CHESSBOARD.lines().collect::<Vec<&str>>().iter().enumerate() {
    match i {
      0 => {
        let _ = term.write_line(format!("{}", console::style(line).dim()).as_str());
      },
      _ => {
        let _ = term.write_line(line);
      }
    }
  }
  let _ = term.move_cursor_to(0, 0);
}

#[test]
fn test_default_board() {
  println!("{}", DEFAULT_CHESSBOARD);
}

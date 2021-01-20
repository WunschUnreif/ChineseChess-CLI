use std::thread;
use std::time::Duration;
use chess_model::board::ChessBoard;
use console::Term;
use chess_model::chess_move::*;
use chess_model::piece::*;

fn main() {
    // board::default_board::show_base_chessboard(&Term::stdout());
    // thread::sleep(Duration::from_secs(2));
    // test_display_position();
    // thread::sleep(Duration::from_secs(2));
    // test_initial_board();

    // thread::sleep(Duration::from_secs(2));
  
  // let board = ChessBoard::init();
  // println!("{}", board.to_json());
    
  // loop {
  //   let mut input = String::new();
  //   let _ = std::io::stdin().read_line(&mut input);
  //   let movement = move_parser::parse_move_str(&input.trim(), PieceColor::RED);
  //   if movement.is_err() {
  //     break;
  //   }
  //   println!("{}", movement.unwrap().to_json());
  // }
  play();
}

fn play() {
  let mut board = ChessBoard::init();
  let mut term = Term::stdout();


  let mut is_red = true;
  loop {
    board.show(&mut term, is_red);
    let _ = term.move_cursor_to(0, 23);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input Error");
    let movement = move_parser::parse_move_str(input.trim(), if is_red {PieceColor::RED} else {PieceColor::BLACK});
    if movement.is_err() {
      println!("Invalid move!");
      continue;
    }

    if movement.unwrap().commit_to_board(&mut board).is_err() {
      println!("Invalid move!");
      continue;
    }
    
    is_red = !is_red;
  }
}

fn test_display_position() {
    use chess_model::board::default_board;
    use chess_model::board::board_position::BoardPosition;
    use chess_model::piece::*;
    use std::io::Write;

    let mut term = Term::stdout();
    let piece = Piece {
      kind: PieceKind::PAO,
      color: PieceColor::BLACK,
      state: PieceState::Active
    };
    let position = BoardPosition::from(8, 2);

    default_board::show_base_chessboard(&term);

    let (x, y) = position.to_display_position(false);
    let _ = term.move_cursor_to(x as usize, y as usize);
    let _ = term.write(piece.get_styled_name().as_bytes());

    thread::sleep(Duration::from_secs(2));
    let _ = term.clear_screen();
    default_board::show_base_chessboard(&term);

    let (x, y) = position.to_display_position(true);
    let _ = term.move_cursor_to(x as usize, y as usize);
    let _ = term.write(piece.get_styled_name().as_bytes());

    let _ = term.move_cursor_to(0, 22);
}

fn test_initial_board() {
  use chess_model::board::*;
  
  let mut board = ChessBoard::init();
  let mut term = Term::stdout();

  board.show(&mut term, true);
  let _ = term.move_cursor_to(0, 22);
  // let _ = term.write_line("--------");

  // thread::sleep(Duration::from_secs(2));
  // board.show(&mut term, false);
  // let _ = term.move_cursor_to(0, 22);
  // let _ = term.write_line("--------");

  thread::sleep(Duration::from_secs(2));
  board.commit(board_position::BoardPosition::from(2, 1), board_position::BoardPosition::from(2, 4));
  board.show(&mut term, true);
  let _ = term.move_cursor_to(0, 22);

  thread::sleep(Duration::from_secs(2));
  board.commit(board_position::BoardPosition::from(9, 1), board_position::BoardPosition::from(7, 2));
  board.show(&mut term, true);
  let _ = term.move_cursor_to(0, 22);
}

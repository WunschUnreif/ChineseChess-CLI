
#[macro_use] extern crate lazy_static;

mod config;
mod display;
mod server_comm;
mod data_model;

use std::{io, thread::sleep_ms};
use chess_model::{board::ChessBoard, piece::PieceColor};
use display::{board_widget::DisplayBoard, match_board_widget::MatchBoard, view_model::ViewModel};
use termion::raw::IntoRawMode;
use tui::{Terminal, layout::{Margin, Rect}, text::Text};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), std::io::Error> {
  // let connection = server_comm::connect_to_server()?;

  // println!("{:?}", connection);

  // let stdout = io::stdout().into_raw_mode()?;
  //   let backend = TermionBackend::new(stdout);
  //   let mut terminal = Terminal::new(backend)?;
  //   let _ = terminal.clear();
  //   let _ = terminal.draw(|f| {
  //       let area = Rect::new(15, 3, 58, 29);

  //       let disp_board = MatchBoard {
  //         board: DisplayBoard(ChessBoard::init(), PieceColor::RED),
  //         in_turn: true,
  //         is_red: true,
  //         this_name: "WunschUnreif".into(),
  //         that_name: "Zirconium".into()
  //       };
  //       f.render_widget(disp_board, area.inner(&Margin{vertical: 1, horizontal: 2}));
  //   });

    let mut vm = ViewModel::new()?;
    vm.render();

    sleep_ms(2000);

    Ok(())
}

use chess_model::{board::{ChessBoard}, piece::{ PieceColor, PieceState, Piece }};
use tui::{buffer::Buffer, layout::Rect, style::{Color, Modifier, Style}, text::Span, widgets::Widget};

mod default_board;
use default_board::DEFAULT_CHESSBOARD;

#[derive(Debug, Clone, Copy)]
pub struct DisplayBoard(pub ChessBoard, pub PieceColor);

impl Widget for DisplayBoard {
  fn render(self, area: Rect, buf: &mut Buffer) {
    (&self).write_base_board(area, buf);
    (&self).write_pieces(area, buf);
  }
}

impl DisplayBoard {
  fn write_base_board(self, area: Rect, buf: &mut Buffer) {
    let (x, mut y) = (area.x, area.y);
    let mut first_line = true;

    for line in DEFAULT_CHESSBOARD.lines() {
      let span = Span::styled(
        line, 
        if first_line {
          Style::default().bg(Color::Rgb(0x78, 0x90, 0x9c))
        } else { Style::default().bg(Color::Rgb(0x54, 0x6e, 0x7a)) }
      );

      buf.set_span(x, y, &span, area.width);

      first_line = false;
      y += 1;
    }

    let span = Span::styled(
      DEFAULT_CHESSBOARD.lines().last().unwrap(), 
      Style::default().bg(Color::Rgb(0x37, 0x47, 0x4f))
    );

    buf.set_span(x, y - 1, &span, area.width);
  }

  fn write_pieces(self, area: Rect, buf: &mut Buffer) {
    let (x0, y0) = (area.x, area.y);
    for row in 0..10 {
      for col in 0..9 {
        let piece = self.0.configuration[col][row];
        if let Some(piece) = piece {
          let span = piece_name(&piece);
          let (x, y) = display_pos(row, col, self.1);
          buf.set_span(x + x0, y + y0, &span, area.width);
        }
      }
    }
  }
}

fn display_pos(row: usize, col: usize, party: PieceColor) -> (u16, u16) {
  if party == PieceColor::RED {
    return display_pos(9 - row, 8 - col, PieceColor::BLACK);
  }

  ((2 + col * 5) as u16, (1 + row * 2) as u16)
}

fn piece_name(piece: &Piece) -> Span<'static> {
  let name = piece.kind.get_name(&piece.color);
  let mut style = Style::default();

  match piece.color {
    PieceColor::RED => {
      style = style.bg(Color::Rgb(0xc6, 0x28, 0x28)).fg(Color::White);
    }
    PieceColor::BLACK => {
      style = style.bg(Color::Rgb(0x1b, 0x5e, 0x20)).fg(Color::White);
    }
  }

  match piece.state {
    PieceState::Active => {}
    PieceState::JustMoved => {
      style = Style::default().add_modifier(Modifier::DIM);
    }
    PieceState::Fresh => {
      style = style.add_modifier(Modifier::SLOW_BLINK);
    }
  }

  Span::styled(name.to_string(), style)
}

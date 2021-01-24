use chess_model::piece::PieceColor;
use tui::{buffer::Buffer, layout::{Margin, Rect}, style::{Color, Modifier, Style}, text::Span, widgets::{Block, Borders, Widget}};

use crate::data_model::DataModel;

use super::board_widget::DisplayBoard;

/// Widget `MatchBoard`
/// 
/// width=53, height=25.
pub struct MatchBoard {
  pub board: DisplayBoard,
  pub in_turn: bool,
  pub is_red: bool,

  pub this_name: String,
  pub that_name: String,
}

impl Widget for MatchBoard {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
      .border_type(tui::widgets::BorderType::Rounded)
      .borders(Borders::ALL)
      .title("MATCH FIELD");

    block.clone().render(area, buf);
    
    let area = block.inner(area).inner(&Margin{ vertical: 0, horizontal: 2 });

    (&self).render_names(&area, buf);
    (&self).render_turn(&area, buf);
    self.board.render(Rect::new(area.x, area.y + 1, area.width, area.height), buf);
  }
}

impl MatchBoard {
  fn render_names(&self, area: &Rect, buf: &mut Buffer) {
    let (topx, topy) = (area.left(), area.top());

    let mut this_name_len = Span::raw(self.this_name.as_str()).width() as u16;
    if this_name_len > area.width {
      this_name_len = area.width
    }

    let (botx, boty) = (area.left() + 45 - this_name_len, area.top() + 22);

    buf.set_span(botx, boty, &Span::styled(
      self.this_name.as_str(), 
      Style::default().bg(
        if self.is_red {
          Color::Rgb(0xc6, 0x28, 0x28)  // red 
        } else {
          Color::Rgb(0x1b, 0x5e, 0x20)  // black
        }
      ).fg(Color::White)
    ), area.width - 2);

    buf.set_span(topx, topy, &Span::styled(
      self.that_name.as_str(), 
      Style::default().bg(
        if self.is_red {
          Color::Rgb(0x1b, 0x5e, 0x20)  // black
        } else {
          Color::Rgb(0xc6, 0x28, 0x28)  // red
        }
      ).fg(Color::White)
    ), area.width - 2);
  }

  fn render_turn(&self, area: &Rect, buf: &mut Buffer) {
    let turn_light_x = area.left() + 46;
    let that_turn_light_y = area.top() + 6;
    let this_turn_light_y = area.top() + 16;

    if self.in_turn {
      buf.set_span(turn_light_x, that_turn_light_y, 
        &Span::styled(
          "⚪️", 
          Style::default()
        ), 
      3);

      buf.set_span(turn_light_x, this_turn_light_y, 
        &Span::styled(
          "🟢", 
          Style::default().add_modifier(Modifier::RAPID_BLINK)
        ), 
      3);
    } else {
      buf.set_span(turn_light_x, that_turn_light_y, 
        &Span::styled(
          "🟢", 
          Style::default()
        ), 
      3);

      buf.set_span(turn_light_x, this_turn_light_y, 
        &Span::styled(
          "⚪️", 
          Style::default()
        ), 
      3);
    }
  }
}

impl From<&DataModel> for MatchBoard {
  fn from(model: &DataModel) -> Self {
    MatchBoard {
      board: DisplayBoard (
        model.board.clone(), 
        if model.is_red {
          PieceColor::RED
        } else {
          PieceColor::BLACK
        }
      ),

      in_turn: model.in_turn,
      is_red: model.is_red,
      this_name: model.this_name.clone().unwrap(),
      that_name: model.that_name.clone().unwrap(),
    }
  }
}

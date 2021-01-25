use std::{cell::RefCell, io::Stdout, sync::mpsc::Sender};
use std::sync::{Arc, Mutex};

use termion::{input::TermRead, raw::{RawTerminal, IntoRawMode}};
use termion::event::{Event};
use tui::{Terminal, backend::TermionBackend, layout::{Margin, Rect}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::{Block, BorderType, Borders, Paragraph}};
use crate::{config, data_model::DataModel};

use super::match_board_widget::MatchBoard;

pub struct ViewModel {
  terminal: RefCell<Terminal<TermionBackend<RawTerminal<Stdout>>>>,
  pub data_model: DataModel,
  command: String,
}

impl ViewModel {
  pub fn new() -> Result<ViewModel, std::io::Error> {
    let terminal = Terminal::new(
      TermionBackend::new(
        std::io::stdout().into_raw_mode()?
      )
    )?;

    Ok(ViewModel {
      terminal: RefCell::new(terminal),
      data_model: DataModel::new(),
      command: String::new(),
    })
  }

  pub fn update_data(&mut self, data_model: DataModel) {
    self.data_model = data_model;
    self.render()
  }

  pub fn update_command(&mut self, command: String) {
    self.command = command;
    self.render()
  }

  pub fn render(&mut self) {
    let _ = self.terminal.borrow_mut().draw(|frame| {
      let frame_size = frame.size();

      // Check the screen size : Require: 70 width * 38 height
      if frame_size.width < 70 || frame_size.height < 38 {
        // Draw the background
        let block = Block::default().style(Style::default().bg(Color::Rgb(0xf4, 0x51, 0x1e)));
        frame.render_widget(block, frame_size);

        // Draw the warnning
        let par = Paragraph::new(Text::raw(
          "Screen too small.\nAt least 70 * 38 is required!"
        ));
        frame.render_widget(par, frame_size);
        return;
      }

      // Draw the background
      let block = Block::default().style(Style::default().bg(Color::Rgb(0x45, 0x5a, 0x64)));
      frame.render_widget(block, frame_size);

      // Draw status bar 
      let mut information = if self.data_model.connection_good {
        format!("  🟢  Connected [{}]", config::server_url())
      } else {
        format!("  🟠  Connecting... [{}]", config::server_url())
      };
      let mut span = Span::raw(information.clone());
      while (span.width() as u16) < frame_size.width {
        information = format!("{} ", information);
        span = Span::raw(information.clone());
      }

      let mut empty_line = String::new();
      for _ in 0..frame_size.width {
        empty_line += " ";
      }

      frame.render_widget(
        Paragraph::new(Text::raw(format!("{}", information))), 
        Rect {
          y: frame_size.bottom() - 1,
          height: 1,
          ..frame_size
        }
      );

      // Draw the match board
      let match_area = Rect {
        height: 27,
        ..frame_size
      }.inner(&Margin { horizontal: 1, vertical: 0 });

      if self.data_model.matching == true {
        let block = Block::default()
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded)
          .title("MATCH");
        frame.render_widget(block, match_area);

        let board = MatchBoard::from(&self.data_model);
        let area = Rect {
          x: (frame_size.width - 53 - 4) / 2,
          y: 1,
          width: 53,
          height: 25,
        };
        frame.render_widget(board, area);
      } else {
        let block = Block::default()
          .borders(Borders::ALL)
          .title("NO MATCH");
        let par = Paragraph::new(Text::raw("No match. You may start a match by enter commands."));

        frame.render_widget(par, block.inner(match_area).inner(&Margin { vertical: 0, horizontal: 2 }));
        frame.render_widget(block, match_area);
      }

      // Draw the command area 
      let command_area = Rect {
        x: 0, y: 27,
        width: frame_size.width,
        height: 5
      }.inner(&Margin { horizontal: 1, vertical: 0 });

      let block = Block::default()
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded)
          .title("COMMAND");
      frame.render_widget(block.clone(), command_area);

      let par = {
        Paragraph::new(Spans::from(vec![
          Span::styled("   〉 ", Style::default().add_modifier(Modifier::BOLD)),
          Span::styled(self.command.clone(), Style::default().add_modifier(Modifier::BOLD))
        ]))
      };
      frame.render_widget(par, block.inner(command_area).inner(&Margin { horizontal: 0, vertical: 1 }));
      
      // Draw the message box 
      let message_area = Rect {
        x: 0, y: 32,
        width: frame_size.width,
        height: 5
      }.inner(&Margin { horizontal: 1, vertical: 0 });

      let block = Block::default()
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded)
          .title("SYSTEM MESSAGE");
      frame.render_widget(block.clone(), message_area);

      let mut par = Paragraph::new(Span::raw(""));
      if self.data_model.explicit_success {
        par = Paragraph::new(Span::raw("  ✅  Success!"))
      } else if let Some(error) = self.data_model.error_message.clone() {
        par = Paragraph::new(Span::raw(format!("  ❌  {}", error)))
      }

      frame.render_widget(par, block.inner(message_area).inner(&Margin { horizontal: 0, vertical: 1 }));

      // Finish
    });
  }
}

pub fn handle_events(comm_chan: Sender<String>, update_chan: Sender<String>, run: Arc<Mutex<bool>>) {
  let stdin = std::io::stdin();
  let mut command = String::new();
  for e in stdin.events() {
    if let Ok(Event::Key(e)) = e {
      if *run.lock().unwrap() == false {
        let _ = update_chan.send("$exit".into());
        let _ = comm_chan.send("$exit".into());
        break;
      }

      match e {
        termion::event::Key::Backspace
        | termion::event::Key::Delete => {
          let len = command.len();
          if len > 0 {
            command = String::from(&command[0..(len - 1)]);
            let _ = update_chan.send(command.clone());
          }
        }
        
        termion::event::Key::Char(c) => {
          if c == '\n' || c == '\r' {
            if command.len() > 0 {
              let _ = comm_chan.send(command.clone());
              command = String::new();
              let _ = update_chan.send(command.clone());
            }
            continue;
          }

          command = format!("{}{}", command, c);
          let _ = update_chan.send(command.clone());
        }

        termion::event::Key::Ctrl('c') => {
          let _ = comm_chan.send("exit".into());
        }

        _ => {}
      }
    }
  }
}

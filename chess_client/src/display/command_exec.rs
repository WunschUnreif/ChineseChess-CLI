use std::{net::TcpStream, sync::{Arc, Mutex}};

use chess_datagram::{DataPacketToServer, DataPacket};
use chess_model::{chess_move::move_parser::parse_move_str, piece::PieceColor};

use crate::data_model::DataModel;

use super::view_model::ViewModel;

pub fn exec_command(
  command: String, 
  vm: Arc<Mutex<ViewModel>>, 
  dm: Arc<Mutex<DataModel>>, 
  connection: &mut TcpStream,
  run: Arc<Mutex<bool>>
) {
  let new_dm: DataModel;

  {
    let mut dm = dm.lock().unwrap();
    
    match parse_command(command, dm.match_id, if dm.is_red { PieceColor::RED } else { PieceColor::BLACK }) {
      Ok(packet) => {
        if let chess_datagram::PayloadToServer::Exit = &packet.payload {
          *run.lock().unwrap() = false;
        } else if let chess_datagram::PayloadToServer::RegisterUser{ username } = &packet.payload {
          dm.this_name = Some(username.clone());
        }

        let result = packet.send(connection);

        dm.explicit_success = false;
        if result.is_err() {
          dm.error_message = Err("Packet delivery failed.".into());
        } else {
          dm.error_message = Ok("".into());
        }
      }

      Err(msg) => {
        dm.explicit_success = false;
        dm.error_message = Err(msg);
      }
    }

    new_dm = dm.clone();
  }

  vm.lock().unwrap().update_data(new_dm);
}

pub fn parse_command(command: String, match_id: usize, party: PieceColor) -> Result<DataPacketToServer, String> {
  let command = command.trim();
  if command.len() == 0 {
    return Err("Empty command.".into())
  }

  let splitter = command.find(' ').unwrap_or(command.len());
  let command_name = &command[0..splitter];

  match command_name {
    "register" => {
      if let Some(name) = command_argument(&command, splitter) {
        return Ok(DataPacketToServer::register_user(name.into()));
      } else {
        return Err("Missing argument. Format: [register <username>]".into());
      }
    }

    "match" => {
      if let Some(with) = command_argument(&command, splitter) {
        return Ok(DataPacketToServer::request_match(with.into()));
      } else {
        return Err("Missing argument. Format: [match <username>]".into());
      }
    }

    "accept" => {
      if let Some(id) = command_argument(&command, splitter) {
        if let Ok(id) = id.parse::<usize>() {
          return Ok(DataPacketToServer::accept_match(id))
        } else {
          return Err("Wrong argument. Format: [accept <id>]".into());
        }
      } else {
        return Err("Missing argument. Format: [accept <id>]".into());
      }
    }

    "draw" => {
      if splitter == command.len() {
        return Ok(DataPacketToServer::request_draw(match_id))
      } else {
        return Err("Extra argument. Format: [draw]".into())
      }
    }

    "fail" => {
      if splitter == command.len() {
        return Ok(DataPacketToServer::request_fail(match_id))
      } else {
        return Err("Extra argument. Format: [fail]".into())
      }
    }

    "admit" => {
      if let Some(prop) = command_argument(&command, splitter) {
        match prop {
          "draw" => return Ok(DataPacketToServer::agree_draw(match_id, true)),
          "fail" => return Ok(DataPacketToServer::agree_fail(match_id, true)),
          _      => return Err("Wrong argument. Format: [admit <fail|draw>]".into())
        }
      } else {
        return Err("Missing argument. Format: [admit <fail|draw>]".into());
      }
    }

    "reject" => {
      if let Some(prop) = command_argument(&command, splitter) {
        match prop {
          "draw" => return Ok(DataPacketToServer::agree_draw(match_id, false)),
          "fail" => return Ok(DataPacketToServer::agree_fail(match_id, false)),
          _      => return Err("Wrong argument. Format: [reject <fail|draw>]".into())
        }
      } else {
        return Err("Missing argument. Format: [reject <fail|draw>]".into());
      }
    }

    "move" => {
      if let Some(mov) = command_argument(&command, splitter) {
        if let Ok(mov) = parse_move_str(mov, party) {
          return Ok(DataPacketToServer::commit_move(match_id, mov))
        } else {
          return Err("Illegal movement. Format: [move <movement>]".into());
        }
      } else {
        return Err("Missing argument. Format: [move <movement>]".into());
      }
    }

    "exit" => {
      if splitter == command.len() {
        return Ok(DataPacketToServer::exit())
      } else {
        return Err("Extra argument. Format: [exit]".into())
      }
    }

    _ => {
      return Err("Unrecognized command.".into());
    }
  }
}

fn command_argument<'a>(command: &'a str, splitter: usize) -> Option<&'a str> {
  if splitter + 1 >= command.len() {
    None
  } else {
    Some(&command[(splitter + 1)..command.len()])
  }
}

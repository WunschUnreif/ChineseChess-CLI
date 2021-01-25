use std::{io::Error, net::TcpStream, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use std::io::{BufRead, BufReader};

use chess_datagram::{DataPacket, DataPacketToClient, DataPacketToServer};
use chess_model::board::ChessBoard;

use crate::{config, data_model::DataModel, display::view_model::ViewModel};

pub fn connect_to_server() -> Result<TcpStream, Error> {
  Ok(TcpStream::connect(config::server_url())?)
}

pub fn spawn_listener(
  mut stream: TcpStream, 
  dm: Arc<Mutex<DataModel>>, 
  vm: Arc<Mutex<ViewModel>>,
  run: Arc<Mutex<bool>>
) -> JoinHandle<()> {
  let _ = DataPacketToServer::aloha().send(&mut stream);

  thread::spawn(move || {
    let mut  stream_echo = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);
    for line in reader.lines() {
      if *run.lock().unwrap() == false { break; }
      if let Ok(line) = line {
        if let Ok(packet) = DataPacketToClient::from_str(line.as_str()) {
          handle_packets(packet, dm.clone(), vm.clone(), &mut stream_echo);
        }
      }
    }
  })
}

fn handle_packets(packet: DataPacketToClient, dm: Arc<Mutex<DataModel>>, vm: Arc<Mutex<ViewModel>>, stream: &mut TcpStream) {
  match packet.payload {
    chess_datagram::PayloadToClient::Aloha => {
      dm.lock().unwrap().connection_good = true;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::Alive => {
      let _ = DataPacketToServer::aloha().send(stream);
    }

    chess_datagram::PayloadToClient::Error { msg } => {
      dm.lock().unwrap().error_message = Err(msg);
      dm.lock().unwrap().explicit_success = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::Success => {
      dm.lock().unwrap().error_message = Ok("Success".into());
      dm.lock().unwrap().explicit_success = true;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::RequestMatch { from, id } => {
      dm.lock().unwrap().error_message = Ok(format!("Match request from user '{}', match id = {}", from, id));
      dm.lock().unwrap().explicit_success = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::StartMatch { with, id, is_red } => {
      dm.lock().unwrap().error_message = Ok("Match start".into());
      dm.lock().unwrap().explicit_success = false;
      dm.lock().unwrap().board = ChessBoard::init();
      dm.lock().unwrap().is_red = is_red;
      dm.lock().unwrap().in_turn = is_red;
      dm.lock().unwrap().that_name = Some(with);
      dm.lock().unwrap().match_id = id;
      dm.lock().unwrap().matching = true;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::RequestDraw { id: _ } => {
      dm.lock().unwrap().error_message = Ok(format!("Your opponent requested a draw."));
      dm.lock().unwrap().explicit_success = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::RequestFail { id: _ } => {
      dm.lock().unwrap().error_message = Ok(format!("Your opponent committed failure."));
      dm.lock().unwrap().explicit_success = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::RequestRejected => {
      dm.lock().unwrap().error_message = Ok(format!("Your proposal has been rejected."));
      dm.lock().unwrap().explicit_success = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::EndMatch { result } => {
      dm.lock().unwrap().error_message = Ok(format!("Match end with result: {}", result));
      dm.lock().unwrap().explicit_success = false;
      dm.lock().unwrap().board = ChessBoard::init();
      dm.lock().unwrap().that_name = None;
      dm.lock().unwrap().match_id = 0;
      dm.lock().unwrap().matching = false;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }

    chess_datagram::PayloadToClient::NewChessboard { board, in_turn } => {
      dm.lock().unwrap().error_message = Ok("Match start".into());
      dm.lock().unwrap().explicit_success = false;
      dm.lock().unwrap().board = board;
      dm.lock().unwrap().in_turn = in_turn;
      vm.lock().unwrap().update_data(dm.lock().unwrap().clone());
    }
  }
}

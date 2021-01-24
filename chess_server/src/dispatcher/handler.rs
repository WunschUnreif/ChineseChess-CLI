use std::{io::{BufReader, BufRead}, net::TcpStream, sync::{self, Mutex}};

#[allow(unused_imports)]
use log::{info, error, warn};
use sync::Arc;

use chess_datagram::*;
use crate::model::*;

mod user;
mod matching;

pub fn handle_connection(stream: TcpStream, model: Arc<Mutex<Model>>) -> Result<(), std::io::Error> {
  let mut handler = Handler::from(stream, model);
  handler.run()
}

pub struct Handler {
  pub model: Arc<Mutex<Model>>,
  pub username: Option<String>,
  pub stream: TcpStream,
}

impl Handler {
  pub fn from(stream: TcpStream, model: Arc<Mutex<Model>>) -> Handler {
    Handler {
      model,
      stream,
      username: None,
    }
  }

  pub fn run(&mut self) -> Result<(), std::io::Error> {
    let reader = BufReader::new(self.stream.try_clone()?);

    for line in reader.lines() {
      if let Ok(line) = line {
        let packet = DataPacketToServer::from_str(line.as_str());
        
        match packet {
          Ok(packet) => {
            if self.handle_packet(packet).is_err() {
              break;
            }
          }
          Err(_) => {
            let _ = DataPacketToClient::error(String::from("Cannot resolve Data")).send(&mut self.stream);
          }
        }
      }
    }

    Ok(())
  }

  pub fn handle_packet(&mut self, packet: DataPacketToServer) -> Result<(), ()> {
    match packet.payload {
      PayloadToServer::Aloha => {
        if self.username.is_some() {
          let mut model = self.model.lock();
          let user = model.as_mut().unwrap().user_manager.find_user_by_name(&self.username.clone().unwrap());
          user.map(|u| u.touch());
        }

        let _ = DataPacketToClient::aloha().send(&mut self.stream);
      }
  
      PayloadToServer::RegisterUser { username } => {
        let _ = self.user_registry(&username).send(&mut self.stream);
      }
      
      PayloadToServer::Exit => {
        return Err(());
      }

      PayloadToServer::RequestMatch {with} => {
        let _ = self.request_match(with).send(&mut self.stream);
      }

      PayloadToServer::AcceptMatch {id} => {
        let _ = self.accept_match(id).send(&mut self.stream);
      }

      PayloadToServer::Move {id, mov} => {
        let _ = self.commit_move(id, mov).send(&mut self.stream);
      }

      PayloadToServer::RequestDraw { id } => {
        let _ = self.request_draw(id).send(&mut self.stream);
      }

      PayloadToServer::AgreeDraw { id, accepted } => {
        let _ = self.agree_draw(id, accepted).send(&mut self.stream);
      }

      PayloadToServer::RequestFail { id } => {
        let _ = self.request_fail(id).send(&mut self.stream);
      }

      PayloadToServer::AgreeFail { id, accepted } => {
        let _ = self.agree_fail(id, accepted).send(&mut self.stream);
      }

      // _ => {
      //   let _ = DataPacketToClient
      //     ::error("Cannot handle this request in the current state.".into())
      //     .send(&mut self.stream);
      // }
    }

    Ok(())
  }
}

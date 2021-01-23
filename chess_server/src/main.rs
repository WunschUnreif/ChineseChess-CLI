use std::{io::{BufRead, BufReader}, net::TcpStream, thread::{self, sleep}, time::Duration};

use chess_datagram::{DataPacketToServer, DataPacket};
use log::info;

#[macro_use]
extern crate lazy_static;

mod model;
mod dispatcher;
mod config;

fn main() -> std::io::Result<()> {
  pretty_env_logger::init();

  thread::spawn(adversary);

  dispatcher::run_server()?;

  Ok(())
}

fn adversary() {
  sleep(Duration::from_secs(2));

  let mut stream = TcpStream::connect("127.0.0.1:3058").unwrap();

  let _ = DataPacketToServer::aloha().send(&mut stream);
  let _ = DataPacketToServer::register_user(String::from("UserTest")).send(&mut stream);
  let _ = DataPacketToServer::register_user(String::from("UserTest")).send(&mut stream);
  let _ = DataPacketToServer::aloha().send(&mut stream);


  let bufreader = BufReader::new(stream.try_clone().unwrap());
  for line in bufreader.lines() {
    if line.is_ok() {
      info!("Recieved response: {}", line.unwrap());
    }
  }
}

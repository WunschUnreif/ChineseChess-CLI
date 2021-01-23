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
  thread::spawn(adversary2);

  dispatcher::run_server()?;

  Ok(())
}

fn adversary() {
  sleep(Duration::from_secs(2));

  let mut stream = TcpStream::connect("127.0.0.1:3058").unwrap();

  let _ = DataPacketToServer::aloha().send(&mut stream);
  let _ = DataPacketToServer::register_user(String::from("UserTest1")).send(&mut stream);
  let _ = DataPacketToServer::request_match("UserTest2".into()).send(&mut stream);


  let bufreader = BufReader::new(stream.try_clone().unwrap());
  for line in bufreader.lines() {
    if line.is_ok() {
      info!("[User 1] Recieved response: {}", line.unwrap());
    }
  }
}

fn adversary2() {
  sleep(Duration::from_secs(1));

  let mut stream = TcpStream::connect("127.0.0.1:3058").unwrap();

  let _ = DataPacketToServer::aloha().send(&mut stream);
  let _ = DataPacketToServer::register_user(String::from("UserTest2")).send(&mut stream);

  sleep(Duration::from_secs(3));
  let _ = DataPacketToServer::accept_match(0).send(&mut stream);


  let bufreader = BufReader::new(stream.try_clone().unwrap());
  for line in bufreader.lines() {
    if line.is_ok() {
      info!("[User 2] Recieved response: {}", line.unwrap());
    }
  }
}

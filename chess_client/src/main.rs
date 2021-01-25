
#[macro_use] extern crate lazy_static;

mod config;
mod display;
mod server_comm;
mod data_model;

use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};

use chess_datagram::{DataPacketToServer, DataPacket};
use data_model::DataModel;
use display::View;

fn main() -> Result<(), std::io::Error> {
  let dm = Arc::new(Mutex::new(DataModel::new()));
  let mut v = View::new()?;
  let mut comm = server_comm::connect_to_server()?;
  let run = v.running.clone();

  v.run(dm.clone(), comm.try_clone()?);
  let listener = server_comm::spawn_listener(comm.try_clone()?, dm.clone(), v.vm.clone(), run.clone());

  loop {
    if *run.lock().unwrap() == false { break; }
    sleep(Duration::from_millis(500));
  }

  v.stop();

  comm = server_comm::connect_to_server()?;
  let _ = DataPacketToServer::aloha().send(&mut comm);
  let _ = DataPacketToServer::exit().send(&mut comm);
  let _ = listener.join();

  Ok(())
}

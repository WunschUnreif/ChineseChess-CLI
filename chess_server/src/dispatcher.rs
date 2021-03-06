use std::{net::{SocketAddr, TcpListener}, thread};

#[allow(unused_imports)]
use log::{info, error, warn};

use crate::config;

mod handler;

pub fn run_server() -> std::io::Result<()> {
  let listener = create_listener()?;

  for connection in listener.incoming() {
    match connection {
        Ok(stream) => {
          info!("Accepted new connection from {:?}.", stream.peer_addr());
          thread::spawn(move || {
            handler::handle_connection(stream, crate::model::SERVER_MODEL.clone())
          });
        }
        Err(_) => {
          warn!("An attemption of connection has failed to accept!");
        }
    };
  }

  Ok(())
}

fn create_listener() -> std::io::Result<TcpListener> {
  let server = config::CONFIG
    .get("server")
    .unwrap_or(&toml::Value::Boolean(false));

  let host = server
    .get("host")
    .unwrap_or(&toml::Value::Boolean(false))
    .as_str()
    .unwrap_or("127.0.0.1");

  let port = server
    .get("port")
    .unwrap_or(&toml::Value::Boolean(false))
    .as_str()
    .unwrap_or("3058");
  
  let listener = TcpListener::bind(SocketAddr::new(
    host.parse().expect("Cannot parse host addr."), 
    port.parse().expect("Cannot parse port.")
  ))?;

  info!("Server listening at {}:{}", host, port);

  Ok(listener)
}

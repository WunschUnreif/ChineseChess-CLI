use std::{io::Error, net::TcpStream};

use crate::config;

pub fn connect_to_server() -> Result<TcpStream, Error> {
  Ok(TcpStream::connect(config::server_url())?)
}

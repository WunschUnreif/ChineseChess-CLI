use std::net::TcpStream;
use std::io::{Write};
use serde::{Serialize, Deserialize};

pub trait DataPacket<'a, P> where Self: Sized + Serialize + Deserialize<'a> + Clone {
  fn from_str(s: &'a str) -> Result<Self, std::io::Error> {
    Ok(serde_json::from_str::<Self>(s.trim())?)
  }

  fn to_string(&self) -> Result<String, std::io::Error> {
    Ok(serde_json::to_string(self)? + "\r\n")
  }

  fn new(payload: P) -> Self;

  fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    stream.write(self.to_string()?.as_bytes())?;
    stream.flush()?;
    Ok(())
  }
}

mod client;
mod server;

pub use client::*;
pub use server::*;


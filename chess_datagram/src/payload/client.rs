use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToClient {
  Aloha,
  Alive,
  Error { msg: String },
  Success,

  RequestMatch { from: String, id: usize },
  StartMatch { with: String, id: usize, is_red: bool },
  RequestDraw,
  RequestFail,
  EndMatch { result: String },
}

impl PayloadToClient {
}

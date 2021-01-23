use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToClient {
  Aloha,
  Error { msg: String },
  Success,

  RequestMatch { from: String },
  StartMatch { with: String },
  RequestDraw,
  RequestFail,
  EndMatch { result: String },
}

impl PayloadToClient {
}

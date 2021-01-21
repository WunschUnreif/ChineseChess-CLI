use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToClient {
  Aloha,
  Error { msg: String }
}

impl PayloadToClient {
}

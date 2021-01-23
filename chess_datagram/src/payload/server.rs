use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToServer {
  Aloha,
  RegisterUser { username: String },

  RequestMatch { with: String },
  AcceptMatch { id: usize },
  RequestDraw,
  RequestFail,

  Exit,
}
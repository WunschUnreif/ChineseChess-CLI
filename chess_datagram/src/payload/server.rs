use serde::{Serialize, Deserialize};
use chess_model::chess_move::ChessMove;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToServer {
  Aloha,
  RegisterUser { username: String },

  RequestMatch { with: String },
  AcceptMatch { id: usize },
  RequestDraw { id: usize },
  AgreeDraw   { id: usize, accepted: bool },
  RequestFail { id: usize },
  AgreeFail   { id: usize, accepted: bool },

  Move { id: usize, mov: ChessMove},

  Exit,
}
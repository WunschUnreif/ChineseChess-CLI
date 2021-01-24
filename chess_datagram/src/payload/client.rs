use chess_model::board::ChessBoard;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadToClient {
  Aloha,
  Alive,
  Error { msg: String },
  Success,

  RequestMatch { from: String, id: usize },
  StartMatch { with: String, id: usize, is_red: bool },
  RequestDraw { id: usize },
  RequestFail { id: usize },
  RequestRejected,
  EndMatch { result: String },

  NewChessboard { board: ChessBoard, in_turn: bool },
}

impl PayloadToClient {
}

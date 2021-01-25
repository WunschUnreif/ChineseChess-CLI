use chess_model::board::ChessBoard;


#[derive(Debug, Clone)]
pub struct DataModel {
  pub connection_good: bool,
  pub explicit_success: bool,
  pub error_message: Result<String, String>,

  pub matching: bool,
  pub match_id: usize,
  pub board: ChessBoard,
  pub is_red: bool,
  pub in_turn: bool,
  pub this_name: Option<String>,
  pub that_name: Option<String>,
}

impl DataModel {
  pub fn new() -> DataModel {
    DataModel {
      connection_good: false,
      error_message: Ok(String::new()),
      explicit_success: false,

      matching: false,
      match_id: 0,
      board: ChessBoard::init(),
      is_red: true,
      in_turn: true,
      this_name: None,
      that_name: None,
    }
  }
}

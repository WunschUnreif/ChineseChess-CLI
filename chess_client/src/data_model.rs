use chess_model::board::ChessBoard;


#[derive(Debug)]
pub struct DataModel {
  pub connection_good: bool,
  pub explicit_success: bool,
  pub error_message: Option<String>,

  pub matching: bool,
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
      error_message: None,
      explicit_success: true,

      matching: true,
      board: ChessBoard::init(),
      is_red: true,
      in_turn: true,
      this_name: Some("aaa".into()),
      that_name: Some("bbb".into()),
    }
  }
}

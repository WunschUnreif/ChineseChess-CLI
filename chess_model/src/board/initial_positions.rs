use crate::piece::{Piece, PieceKind, PieceColor, PieceState};
use crate::board::ChessBoard;

impl ChessBoard {
  pub fn init() -> ChessBoard {
    let mut result = ChessBoard {
      configuration: [[None; 10]; 9]
    };

    result.configuration[0][0] = Some(Piece { kind: PieceKind::JU, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[1][0] = Some(Piece { kind: PieceKind::MA, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[2][0] = Some(Piece { kind: PieceKind::XIANG, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[3][0] = Some(Piece { kind: PieceKind::SHI, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[4][0] = Some(Piece { kind: PieceKind::JIANG, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[5][0] = Some(Piece { kind: PieceKind::SHI, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[6][0] = Some(Piece { kind: PieceKind::XIANG, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[7][0] = Some(Piece { kind: PieceKind::MA, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[8][0] = Some(Piece { kind: PieceKind::JU, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[1][2] = Some(Piece { kind: PieceKind::PAO, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[7][2] = Some(Piece { kind: PieceKind::PAO, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[0][3] = Some(Piece { kind: PieceKind::BING, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[2][3] = Some(Piece { kind: PieceKind::BING, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[4][3] = Some(Piece { kind: PieceKind::BING, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[6][3] = Some(Piece { kind: PieceKind::BING, color: PieceColor::RED, state: PieceState::Active });
    result.configuration[8][3] = Some(Piece { kind: PieceKind::BING, color: PieceColor::RED, state: PieceState::Active });

    result.configuration[0][9] = Some(Piece { kind: PieceKind::JU, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[1][9] = Some(Piece { kind: PieceKind::MA, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[2][9] = Some(Piece { kind: PieceKind::XIANG, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[3][9] = Some(Piece { kind: PieceKind::SHI, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[4][9] = Some(Piece { kind: PieceKind::JIANG, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[5][9] = Some(Piece { kind: PieceKind::SHI, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[6][9] = Some(Piece { kind: PieceKind::XIANG, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[7][9] = Some(Piece { kind: PieceKind::MA, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[8][9] = Some(Piece { kind: PieceKind::JU, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[1][7] = Some(Piece { kind: PieceKind::PAO, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[7][7] = Some(Piece { kind: PieceKind::PAO, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[0][6] = Some(Piece { kind: PieceKind::BING, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[2][6] = Some(Piece { kind: PieceKind::BING, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[4][6] = Some(Piece { kind: PieceKind::BING, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[6][6] = Some(Piece { kind: PieceKind::BING, color: PieceColor::BLACK, state: PieceState::Active });
    result.configuration[8][6] = Some(Piece { kind: PieceKind::BING, color: PieceColor::BLACK, state: PieceState::Active });

    result
  }
}

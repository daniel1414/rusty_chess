use super::{match_state::PlayerColor, square::SquarePosition};

/// Represents a chess piece.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

/// Represents a colored chess piece.
#[derive(Copy, Clone, Debug)]
pub struct ColoredChessPiece {
    pub piece: ChessPiece,
    pub color: PlayerColor,
    pub label: &'static str,
}

impl ColoredChessPiece {
    pub fn new(piece: ChessPiece, color: PlayerColor, label: &'static str) -> Self {
        Self {
            piece,
            color,
            label,
        }
    }
}

#[derive(Debug)]
pub struct PositionedChessPiece {
    pub col_figure: ColoredChessPiece,
    pub position: SquarePosition,
}

impl PositionedChessPiece {
    pub fn new(piece: ColoredChessPiece, position: SquarePosition) -> Self {
        Self {
            col_figure: piece,
            position,
        }
    }
}

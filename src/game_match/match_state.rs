use rusty_engine::prelude::bevy::utils::hashbrown::HashSet;

use super::{board::Board, square::SquarePosition};

/// Represents a chess piece.
#[derive(Copy, Clone, Debug)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

/// Represents a player's color.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerColor {
    White,
    Black,
}

/// Represents a colored chess piece.
#[derive(Copy, Clone, Debug)]
pub struct ColoredFigure {
    pub piece: ChessPiece,
    pub color: PlayerColor,
    pub label: &'static str,
}

impl ColoredFigure {
    pub fn new(piece: ChessPiece, color: PlayerColor, label: &'static str) -> Self {
        Self {
            piece,
            color,
            label,
        }
    }
}

#[derive(Debug)]
pub struct PositionedFigure {
    pub col_figure: ColoredFigure,
    pub position: SquarePosition,
}

impl PositionedFigure {
    pub fn new(piece: ColoredFigure, position: SquarePosition) -> Self {
        Self {
            col_figure: piece,
            position,
        }
    }
}

/// Represents the state of a chess match.
pub struct MatchState {
    /// Color of the player that is playing the whole match.
    pub player_color: PlayerColor,

    /// Representation of the board. On every square there can be one piece.
    pub board: Board,

    /// Current turn.
    pub turn: PlayerColor,

    /// Currently selected piece.
    pub selected_piece: Option<PositionedFigure>,

    /// The squares the current piece can go to.
    pub available_moves: HashSet<SquarePosition>,

    /// Whether to re-render the board.
    pub is_dirty: bool,
}

impl MatchState {
    // Creates a new match state, depending on the player's color.
    pub fn new(player_color: PlayerColor) -> Self {
        Self {
            player_color,
            board: Board::new(player_color),
            turn: PlayerColor::White,
            selected_piece: None,
            is_dirty: true,
            available_moves: HashSet::new(),
        }
    }
}

use rusty_engine::prelude::bevy::utils::hashbrown::HashSet;

use super::{board::Board, chess_piece::PositionedChessPiece, square::SquarePosition};

/// Represents a player's color.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn toggle(&mut self) {
        match self {
            PlayerColor::White => *self = Self::Black,
            PlayerColor::Black => *self = Self::White,
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
    pub selected_piece: Option<PositionedChessPiece>,

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

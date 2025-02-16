/// Represents a chess figure.
#[derive(Copy, Clone, Debug)]
pub enum Figure {
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

/// Represents a colored chess figure.
#[derive(Copy, Clone, Debug)]
pub struct ColoredFigure {
    pub figure: Figure,
    pub color: PlayerColor,
    pub label: &'static str,
}

impl ColoredFigure {
    pub fn new(figure: Figure, color: PlayerColor, label: &'static str) -> Self {
        Self {
            figure,
            color,
            label,
        }
    }
}

#[derive(Debug)]
pub struct SquarePosition {
    x: u8,
    y: u8,
}

impl SquarePosition {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn to_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }
}

/// Represents the state of a chess match.
pub struct MatchState {
    /// Color of the player that is playing the whole match.
    pub player_color: PlayerColor,

    /// Representation of the board. On every square there can be one figure.
    pub board: [Option<ColoredFigure>; 64],

    /// Current turn.
    pub turn: PlayerColor,

    /// Currently selected figure.
    pub selected_piece: Option<ColoredFigure>,

    /// Whether to re-render the board.
    pub is_dirty: bool,
}

impl MatchState {
    // Creates a new match state, depending on the player's color.
    pub fn new(player_color: PlayerColor) -> Self {
        let mut board: [Option<ColoredFigure>; 64] = [
            // First row
            Some(ColoredFigure::new(
                Figure::Rook,
                PlayerColor::White,
                "white_rook1",
            )),
            Some(ColoredFigure::new(
                Figure::Knight,
                PlayerColor::White,
                "white_knight1",
            )),
            Some(ColoredFigure::new(
                Figure::Bishop,
                PlayerColor::White,
                "white_bishop1",
            )),
            Some(ColoredFigure::new(
                Figure::King,
                PlayerColor::White,
                "white_king1",
            )),
            Some(ColoredFigure::new(
                Figure::Queen,
                PlayerColor::White,
                "white_queen1",
            )),
            Some(ColoredFigure::new(
                Figure::Bishop,
                PlayerColor::White,
                "white_bishop2",
            )),
            Some(ColoredFigure::new(
                Figure::Knight,
                PlayerColor::White,
                "white_knight2",
            )),
            Some(ColoredFigure::new(
                Figure::Rook,
                PlayerColor::White,
                "white_rook2",
            )),
            // Second row
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn1",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn2",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn3",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn4",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn5",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn6",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn7",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::White,
                "white_pawn8",
            )),
            // Third, fourth, fifth and sixth row
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // Seventh row
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn1",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn2",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn3",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn4",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn5",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn6",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn7",
            )),
            Some(ColoredFigure::new(
                Figure::Pawn,
                PlayerColor::Black,
                "black_pawn8",
            )),
            // Eigth row
            Some(ColoredFigure::new(
                Figure::Rook,
                PlayerColor::Black,
                "black_rook1",
            )),
            Some(ColoredFigure::new(
                Figure::Knight,
                PlayerColor::Black,
                "black_knight1",
            )),
            Some(ColoredFigure::new(
                Figure::Bishop,
                PlayerColor::Black,
                "black_bishop1",
            )),
            Some(ColoredFigure::new(
                Figure::King,
                PlayerColor::Black,
                "black_king1",
            )),
            Some(ColoredFigure::new(
                Figure::Queen,
                PlayerColor::Black,
                "black_queen1",
            )),
            Some(ColoredFigure::new(
                Figure::Bishop,
                PlayerColor::Black,
                "black_bishop2",
            )),
            Some(ColoredFigure::new(
                Figure::Knight,
                PlayerColor::Black,
                "black_knight2",
            )),
            Some(ColoredFigure::new(
                Figure::Rook,
                PlayerColor::Black,
                "black_rook2",
            )),
        ];

        if player_color == PlayerColor::Black {
            board.reverse();
        }

        Self {
            player_color,
            board,
            turn: PlayerColor::White,
            selected_piece: None,
            is_dirty: true,
        }
    }
}

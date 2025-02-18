use super::{
    chess_piece::{ChessPiece, ColoredChessPiece},
    match_state::PlayerColor,
    square::SquarePosition,
};

pub struct Board {
    pub squares: [Option<ColoredChessPiece>; 64],
    pub taken_pieces: Vec<ColoredChessPiece>,
}

impl Board {
    pub fn get(&self, pos: &SquarePosition) -> &Option<ColoredChessPiece> {
        &self.squares[pos.to_index()]
    }

    pub fn get_mut(&mut self, pos: &SquarePosition) -> &mut Option<ColoredChessPiece> {
        &mut self.squares[pos.to_index()]
    }

    pub fn new(player_color: PlayerColor) -> Self {
        let mut board: [Option<ColoredChessPiece>; 64] = [
            // First row
            Some(ColoredChessPiece::new(
                ChessPiece::Rook,
                PlayerColor::White,
                "white_rook1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Knight,
                PlayerColor::White,
                "white_knight1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Bishop,
                PlayerColor::White,
                "white_bishop1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Queen,
                PlayerColor::White,
                "white_queen1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::King,
                PlayerColor::White,
                "white_king1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Bishop,
                PlayerColor::White,
                "white_bishop2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Knight,
                PlayerColor::White,
                "white_knight2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Rook,
                PlayerColor::White,
                "white_rook2",
            )),
            // Second row
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn3",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn4",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn5",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn6",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::White,
                "white_pawn7",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
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
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn3",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn4",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn5",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn6",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn7",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Pawn,
                PlayerColor::Black,
                "black_pawn8",
            )),
            // Eigth row
            Some(ColoredChessPiece::new(
                ChessPiece::Rook,
                PlayerColor::Black,
                "black_rook1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Knight,
                PlayerColor::Black,
                "black_knight1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Bishop,
                PlayerColor::Black,
                "black_bishop1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Queen,
                PlayerColor::Black,
                "black_queen1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::King,
                PlayerColor::Black,
                "black_king1",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Bishop,
                PlayerColor::Black,
                "black_bishop2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Knight,
                PlayerColor::Black,
                "black_knight2",
            )),
            Some(ColoredChessPiece::new(
                ChessPiece::Rook,
                PlayerColor::Black,
                "black_rook2",
            )),
        ];

        if player_color == PlayerColor::Black {
            board.reverse();
        }

        Self {
            squares: board,
            taken_pieces: vec![],
        }
    }

    pub fn is_valid_pos(pos: &SquarePosition) -> bool {
        pos.x < 8 && pos.y < 8
    }
}

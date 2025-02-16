use super::match_state::{ColoredFigure, Figure, PlayerColor, SquarePosition};

pub struct Board {
    pub squares: [Option<ColoredFigure>; 64],
    index: usize,
}

impl Board {
    pub fn get(&self, pos: &SquarePosition) -> &Option<ColoredFigure> {
        &self.squares[pos.to_index()]
    }

    pub fn get_mut(&mut self, pos: &SquarePosition) -> &mut Option<ColoredFigure> {
        &mut self.squares[pos.to_index()]
    }

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
                Figure::Queen,
                PlayerColor::White,
                "white_queen1",
            )),
            Some(ColoredFigure::new(
                Figure::King,
                PlayerColor::White,
                "white_king1",
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
                Figure::Queen,
                PlayerColor::Black,
                "black_queen1",
            )),
            Some(ColoredFigure::new(
                Figure::King,
                PlayerColor::Black,
                "black_king1",
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
            squares: board,
            index: 0,
        }
    }

    pub fn is_valid_pos(pos: &SquarePosition) -> bool {
        pos.x < 8 && pos.y < 8
    }
}

impl Iterator for Board {
    type Item = Option<ColoredFigure>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.squares.len() {
            self.index += 1;
            Some(self.squares[self.index - 1])
        } else {
            None
        }
    }
}

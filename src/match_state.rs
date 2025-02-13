use rusty_engine::prelude::bevy::render::color::Color;


#[derive(Copy, Clone, Debug)]
pub enum Figure {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FigureColor {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct ColoredFigure{
    pub figure: Figure, 
    pub color: FigureColor, 
    pub label: &'static str,
}

impl ColoredFigure {
    pub fn new(figure: Figure, color: FigureColor, label: &'static str) -> Self {
        Self { figure, color, label }
    }
}

pub struct MatchState {
    pub board: [Option<ColoredFigure>; 64],
}

impl MatchState {
    pub fn new(player_color: FigureColor) -> Self {

        let mut board: [Option<ColoredFigure>; 64] = [
            // First row
            Some(ColoredFigure::new(Figure::Rook, FigureColor::Black, "black_rook1")),
            Some(ColoredFigure::new(Figure::Knight, FigureColor::Black, "black_knight1")),
            Some(ColoredFigure::new(Figure::Bishop, FigureColor::Black, "black_bishop1")),
            Some(ColoredFigure::new(Figure::Queen, FigureColor::Black, "black_queen1")),
            Some(ColoredFigure::new(Figure::King, FigureColor::Black, "black_king1")),
            Some(ColoredFigure::new(Figure::Bishop, FigureColor::Black, "black_bishop2")),
            Some(ColoredFigure::new(Figure::Knight, FigureColor::Black, "black_knight2")),
            Some(ColoredFigure::new(Figure::Rook, FigureColor::Black, "black_rook2")),

            // Second row
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn1")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn2")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn3")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn4")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn5")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn6")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn7")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::Black, "black_pawn8")),

            // Third, fourth, fifth and sixth row
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            // Seventh row
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn1")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn2")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn3")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn4")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn5")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn6")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn7")),
            Some(ColoredFigure::new(Figure::Pawn, FigureColor::White, "white_pawn8")),

            // Eigth row
            Some(ColoredFigure::new(Figure::Rook, FigureColor::White, "white_rook1")),
            Some(ColoredFigure::new(Figure::Knight, FigureColor::White, "white_knight1")),
            Some(ColoredFigure::new(Figure::Bishop, FigureColor::White, "white_bishop1")),
            Some(ColoredFigure::new(Figure::King, FigureColor::White, "white_king1")),
            Some(ColoredFigure::new(Figure::Queen, FigureColor::White, "white_queen1")),
            Some(ColoredFigure::new(Figure::Bishop, FigureColor::White, "white_bishop2")),
            Some(ColoredFigure::new(Figure::Knight, FigureColor::White, "white_knight2")),
            Some(ColoredFigure::new(Figure::Rook, FigureColor::White, "white_rook2")),
        ];

        if player_color == FigureColor::White { 
            board.reverse();
        }

        Self { 
            board: board,
        }
    }
}
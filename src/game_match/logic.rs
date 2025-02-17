use rusty_engine::prelude::bevy::utils::HashSet;

use super::{
    board::Board,
    match_state::{MatchState, PlayerColor, PositionedFigure},
    square::SquarePosition,
};

pub fn update_available_moves(match_state: &mut MatchState) {
    match_state.available_moves.clear();
    if let Some(pos_figure) = match_state.selected_piece.as_ref() {
        match_state.available_moves = get_available_moves(pos_figure, &match_state.board);
    }
}

fn get_available_moves(active_piece: &PositionedFigure, board: &Board) -> HashSet<SquarePosition> {
    let mut available_moves: HashSet<SquarePosition> = HashSet::new();

    let pos = &active_piece.position;
    match active_piece.col_figure.figure {
        super::match_state::Figure::Pawn => {
            let front_moves = if active_piece.col_figure.color == PlayerColor::White {
                let mut result = vec![pos.try_add(0, 1)];
                if pos.y == 1 && board.get(&pos.try_add(0, 1).unwrap()).is_none() {
                    result.push(pos.try_add(0, 2));
                }
                result
            } else {
                let mut result = vec![pos.try_add(0, -1)];
                if pos.y == 6 && board.get(&pos.try_add(0, -1).unwrap()).is_none() {
                    result.push(pos.try_add(0, -2));
                }
                result
            };

            front_moves
                .into_iter()
                .filter(|m| m.is_some())
                .for_each(|m| {
                    if board.get(&m.unwrap()).is_none() {
                        available_moves.insert(m.unwrap());
                    }
                });

            // Check the square to the left and right.
            let corners = [pos.try_add(-1, 1), pos.try_add(1, 1)];

            corners
                .iter()
                .filter(|corner| corner.is_some())
                .for_each(|corner| {
                    let corner = corner.unwrap();
                    if Board::is_valid_pos(&corner) {
                        if let Some(figure) = board.get(&corner) {
                            if figure.color == PlayerColor::Black {
                                available_moves.insert(corner.clone());
                            }
                        }
                    }
                });
        }
        super::match_state::Figure::Rook => todo!(),
        super::match_state::Figure::Bishop => todo!(),
        super::match_state::Figure::Knight => todo!(),
        super::match_state::Figure::Queen => todo!(),
        super::match_state::Figure::King => todo!(),
    }

    available_moves
}

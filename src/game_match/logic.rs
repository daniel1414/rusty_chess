use rusty_engine::prelude::bevy::utils::HashSet;

use super::{
    board::Board,
    match_state::{MatchState, PlayerColor, PositionedFigure, SquarePosition},
};

pub fn update_available_moves(match_state: &mut MatchState) {
    match_state.available_moves.clear();
    if let Some(pos_figure) = match_state.selected_piece.as_ref() {
        let available_moves = get_available_moves(pos_figure, &match_state.board);
        match_state.available_moves = available_moves;
    }
}

fn get_available_moves(active_piece: &PositionedFigure, board: &Board) -> HashSet<SquarePosition> {
    let mut available_moves = HashSet::new();

    let pos = &active_piece.position;
    match active_piece.col_figure.figure {
        super::match_state::Figure::Pawn => {
            if active_piece.col_figure.color == PlayerColor::White {
                // Check the square in the front.
                let forward = pos.add(0, 1);
                if board.get(&forward).is_none() && Board::is_valid_pos(&forward) {
                    available_moves.insert(forward);
                }

                // Check the square in the front.
                if pos.y == 1 {
                    if let Some(double_forward) = pos.try_add(0, 2) {
                        if board.get(&double_forward).is_none()
                            && Board::is_valid_pos(&double_forward)
                        {
                            available_moves.insert(double_forward);
                        }
                    };
                }

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
        }
        super::match_state::Figure::Rook => todo!(),
        super::match_state::Figure::Bishop => todo!(),
        super::match_state::Figure::Knight => todo!(),
        super::match_state::Figure::Queen => todo!(),
        super::match_state::Figure::King => todo!(),
    }

    available_moves
}

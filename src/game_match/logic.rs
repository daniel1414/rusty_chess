use rusty_engine::prelude::bevy::utils::HashSet;

use super::{
    board::Board,
    chess_piece::{ChessPiece, PositionedChessPiece},
    match_state::{MatchState, PlayerColor},
    square::SquarePosition,
};

pub fn update_available_moves(match_state: &mut MatchState) {
    match_state.available_moves.clear();
    if let Some(pos_figure) = match_state.selected_piece.as_ref() {
        match_state.available_moves = get_available_moves(pos_figure, &match_state.board);
    }
}

fn get_available_moves(
    active_piece: &PositionedChessPiece,
    board: &Board,
) -> HashSet<SquarePosition> {
    let mut available_moves: HashSet<SquarePosition> = HashSet::new();

    let pos = &active_piece.position;
    match active_piece.col_figure.piece {
        ChessPiece::Pawn => {
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
            let corners = if active_piece.col_figure.color == PlayerColor::White {
                [pos.try_add(-1, 1), pos.try_add(1, 1)]
            } else {
                [pos.try_add(-1, -1), pos.try_add(1, -1)]
            };

            corners
                .into_iter()
                .filter(|corner| corner.is_some())
                .for_each(|corner| {
                    let corner = corner.unwrap();
                    if Board::is_valid_pos(&corner) {
                        if let Some(piece) = board.get(&corner) {
                            if piece.color != active_piece.col_figure.color {
                                available_moves.insert(corner.clone());
                            }
                        }
                    }
                });
        }
        ChessPiece::Rook => todo!(),
        ChessPiece::Bishop => todo!(),
        ChessPiece::Knight => todo!(),
        ChessPiece::Queen => todo!(),
        ChessPiece::King => todo!(),
    }

    available_moves
}

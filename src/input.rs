use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::{
        chess_piece::PositionedChessPiece, logic::update_available_moves, match_state::MatchState,
        square::SquarePosition,
    },
    game_state::GameState,
    render::{is_pixel_on_board, pixel_to_square},
};

/// Handles all user input.
pub fn handle_input(engine: &mut Engine, state: &mut GameState) {
    match state {
        GameState::Lobby(lobby_state) => handle_lobby_input(engine, lobby_state),
        GameState::Match(match_state) => handle_match_input(engine, match_state),
    };
}

/// Handles user input while in the lobby (NO LOBBY YET!)
fn handle_lobby_input(engine: &mut Engine, _: &mut LobbyState) {
    for mouse_event in &engine.mouse_button_events {
        println!("{:?}", mouse_event);
    }
}

/// Handles user input while in a match.
fn handle_match_input(engine: &mut Engine, match_state: &mut MatchState) {
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(location) = engine.mouse_state.location() {
            if is_pixel_on_board(location) {
                let pos = pixel_to_square(location);
                handle_square_pressed(pos, match_state);
            }
        }
    }

    // Mark as dirty for the renderer to redraw the selected piece.
    if !engine.mouse_location_events.is_empty() && match_state.selected_piece.is_some() {
        match_state.is_dirty = true;
    }
}

fn handle_square_pressed(square: SquarePosition, match_state: &mut MatchState) {
    match match_state.board.get_mut(&square) {
        Some(piece) => {
            if match_state.selected_piece.is_none() && piece.color == match_state.player_color {
                // The player selected a chess piece.
                match_state.selected_piece = Some(PositionedChessPiece::new(
                    match_state.board.get_mut(&square).take().unwrap(),
                    square,
                ));
                update_available_moves(match_state);
                match_state.is_dirty = true;
            }
        }
        None => {
            if match_state.selected_piece.is_some() {
                *match_state.board.get_mut(&square) =
                    Some(match_state.selected_piece.take().unwrap().col_figure);
                update_available_moves(match_state);
                match_state.is_dirty = true;
            }
        }
    }
}

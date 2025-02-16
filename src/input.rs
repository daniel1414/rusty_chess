use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::match_state::MatchState,
    game_state::GameState,
    render::{is_pixel_on_board, pixel_to_square},
};

pub fn handle_input(engine: &mut Engine, state: &mut GameState) {
    match state {
        GameState::Lobby(lobby_state) => handle_lobby_input(engine, lobby_state),
        GameState::Match(match_state) => handle_match_input(engine, match_state),
    };
}

fn handle_lobby_input(engine: &mut Engine, _: &mut LobbyState) {
    for mouse_event in &engine.mouse_button_events {
        println!("{:?}", mouse_event);
    }
}

fn handle_match_input(engine: &mut Engine, match_state: &mut MatchState) {
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(location) = engine.mouse_state.location() {
            if is_pixel_on_board(location) {
                let pos = pixel_to_square(location);
                dbg!(&pos);
                let index = pos.to_index();
                match match_state.board.get_mut(index) {
                    Some(figure) => match_state.selected_piece = figure.take(),
                    None => {
                        if match_state.selected_piece.is_some() {
                            match_state.board[index] = match_state.selected_piece.take();
                        }
                    }
                }
                match_state.is_dirty = true;
            }
        }
    }

    if match_state.selected_piece.is_some() && engine.mouse_location_events.len() > 0 {
        match_state.is_dirty = true;
    }
}

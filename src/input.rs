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

fn handle_match_input(engine: &mut Engine, _: &mut MatchState) {
    for mouse_event in &engine.mouse_button_events {
        match mouse_event {
            MouseButtonInput {
                button,
                state,
                window,
            } => match button {
                MouseButton::Left => {
                    if engine.mouse_state.pressed(MouseButton::Left) {
                        if let Some(location) = engine.mouse_state.location() {
                            if is_pixel_on_board(location) {
                                print!("{:?} {:?} {:?}\n", button, state, window);
                                println!("{:?}", engine.mouse_state.location());
                                println!("{:?}", pixel_to_square(location));
                            }
                        }
                    }
                }
                _ => {}
            },
        }
    }
}

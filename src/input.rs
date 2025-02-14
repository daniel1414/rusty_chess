use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState, game_match::match_state::MatchState, game_state::GameState,
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
        println!("{:?}", mouse_event);
    }
}

use rusty_engine::prelude::*;

use crate::{game_state::{GameState, LobbyState}, match_state::MatchState};

pub fn handle_input(engine: &mut Engine, state: &mut GameState) {
    match state {
        GameState::Lobby(lobby_state) => handle_lobby_input(engine, lobby_state),
        GameState::Match(match_state) => handle_match_input(engine, match_state),
    };
}

fn handle_lobby_input(engine: &mut Engine, state: &mut LobbyState) {
    for mouse_event in &engine.mouse_button_events {
        println!("{:?}", mouse_event);
    }
}

fn handle_match_input(engine: &mut Engine, state: &mut MatchState) {
    for mouse_event in &engine.mouse_button_events {
        println!("{:?}", mouse_event);
    }
}
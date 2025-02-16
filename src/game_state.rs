use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::match_state::{MatchState, PlayerColor},
};

/// Represents the game state.
#[derive(Resource)]
pub enum GameState {
    Lobby(LobbyState),
    Match(MatchState),
}

impl GameState {
    pub fn new() -> Self {
        Self::Match(MatchState::new(PlayerColor::White))
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

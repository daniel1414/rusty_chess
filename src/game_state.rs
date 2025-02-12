use rusty_engine::prelude::*;


pub struct LobbyState {

}

impl LobbyState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct MatchState {

}

impl MatchState {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Resource)]
pub enum GameState {
    Lobby(LobbyState),
    Match(MatchState),
}

impl GameState {
    pub fn new() -> Self {
        Self::Lobby( LobbyState {  } )
    }
}
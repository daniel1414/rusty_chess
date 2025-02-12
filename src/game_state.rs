use rusty_engine::prelude::*;


pub struct LobbyState {

}

pub struct MatchState {

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
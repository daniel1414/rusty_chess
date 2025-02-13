use rusty_engine::prelude::*;

use crate::match_state::{FigureColor, MatchState};


pub struct LobbyState {

}

impl LobbyState {
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
        Self::Match( MatchState::new(FigureColor::White) )
    }
}
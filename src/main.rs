use render::render::{on_startup, render};
use rusty_engine::prelude::*;

pub mod game_lobby;
pub mod game_match;
pub mod game_state;
pub mod input;
pub mod render;

use input::handle_input;

use crate::game_state::GameState;

fn main() {
    let initial_game_state = GameState::new();

    let mut game = Game::new();
    on_startup(&mut game, &initial_game_state);

    game.add_logic(game_logic);
    game.run(initial_game_state);
}

/// Performs the game logic.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    handle_input(engine, game_state);

    render(engine, game_state);
}

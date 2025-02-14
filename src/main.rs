use rusty_engine::prelude::*;

pub mod game_lobby;
pub mod game_match;
pub mod game_state;
pub mod input;

use input::handle_input;

use crate::game_state::GameState;

fn main() {
    let initial_game_state = GameState::new();

    let mut game = Game::new();
    initial_game_state.draw(&mut game);

    game.add_logic(game_logic);
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Handle input
    handle_input(engine, game_state);
}

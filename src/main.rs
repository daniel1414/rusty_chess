use std::path::Path;

use input::handle_input;
use rusty_engine::prelude::*;

pub mod game_state;
pub mod input;

use crate::game_state::GameState;


fn load_assets(game: &mut Game<GameState>) {
    let base_path = Path::new("sprite/chess");

    let assets = &[
            "black_rook",
            "black_knight",
            "black_bishop",
            "black_queen",
            "black_king",
            "black_pawn",
            "white_rook",
            "white_knight",
            "white_bishop",
            "white_queen",
            "white_king",
            "white_pawn",
        ];

    for label in assets {
        let label_str = label.to_string();
        let sprite = game.add_sprite(label_str.clone(), base_path.join(label_str + ".png"));
        sprite.collision = false;
        sprite.scale = 0.5;
    }
}

fn main() {

    let initial_game_state = GameState::new();
    let mut game = Game::new();
    load_assets(&mut game);
    game.add_logic(game_logic);
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Handle input
    handle_input(engine, game_state);
}

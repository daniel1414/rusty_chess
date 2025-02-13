use std::path::Path;

use input::handle_input;
use rusty_engine::prelude::{bevy::math::vec2, *};

pub mod match_state;
pub mod game_state;
pub mod input;

use crate::game_state::GameState;

const FIGURE_SCALE: f32 = 0.25;
const SQUARE_SIZE: f32 = 75.0;

fn load_sprites(game: &mut Game<GameState>, game_state: &GameState) {
    let base_path = Path::new("sprite/chess");
    let board_offset = vec2(-150.0, 0.0);

    // Board frame
    let label = "board_frame";
    let sprite = game.add_sprite(label.to_string(), base_path.join(label.to_string() + ".png"));
    sprite.scale = 0.38;
    sprite.translation += board_offset;

    // Board itself
    let label = "just_board";
    let sprite = game.add_sprite(label.to_string(), base_path.join(label.to_string() + ".png"));
    sprite.scale = 0.5;
    sprite.translation += board_offset;

    match game_state {
        GameState::Lobby(lobby_state) => todo!(),
        GameState::Match(match_state) => {

            for i in 0.. match_state.board.len() {
                if let Some(figure) = match_state.board[i] {
                    let x = (i % 8) as f32;
                    let y = (i / 8) as f32;
                    let offset = board_offset + (vec2((x) * SQUARE_SIZE,(y) * SQUARE_SIZE)) + vec2(-262.5, -262.5);
                    let sprite = game.add_sprite(figure.label.to_string(), base_path.join(figure.label[0..figure.label.len() - 1].to_string() + ".png"));
                    sprite.scale = FIGURE_SCALE;
                    sprite.translation = offset;
                    sprite.layer = 1.0;
                }
            }
        },
    }

    //// Every piece is 300 in width and height, so to fit into 75, we must scale it by 0.25
    //let label = "black_pawn";
    //let sprite = game.add_sprite(label.to_string(), base_path.join(label.to_string() + ".png"));
    //sprite.scale = FIGURE_SCALE;
//
    //// Move the pawn half a square to the right and up.
    //sprite.translation += vec2(-37.5, -37.5) + board_offset;

}

fn main() {

    let initial_game_state = GameState::new();
    
    let mut game = Game::new();
    load_sprites(&mut game, &initial_game_state);

    game.add_logic(game_logic);
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Handle input
    handle_input(engine, game_state);
}

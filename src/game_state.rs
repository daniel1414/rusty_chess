use rusty_engine::prelude::*;
use std::path::Path;
use rusty_engine::prelude::bevy::math::vec2;

use crate::{game_lobby::lobby_state::LobbyState, game_match::match_state::{PlayerColor, MatchState}};


/// Offset of the center of the board from the center of the window.
const BOARD_OFFSET: Vec2 = vec2(-150.0, 0.0);

/// Scale of every figure.
const FIGURE_SCALE: f32 = 0.25;

/// Size of a square (precalculated)
const SQUARE_SIZE: f32 = 75.0;

/// Shift to the first position on the board - top left.
const FIGURE_SHIFT_FROM_CENTER: Vec2 = vec2(-(4.0 * SQUARE_SIZE - 37.5), -(4.0 * SQUARE_SIZE - 37.5));



#[derive(Resource)]
pub enum GameState {
    Lobby(LobbyState),
    Match(MatchState),
}

impl GameState {
    pub fn new() -> Self {
        Self::Match( MatchState::new(PlayerColor::Black) )
    }

    pub fn draw(&self, game: &mut Game<GameState>) {
        let base_path = Path::new("sprite/chess");

        // Board frame
        let label = "board_frame";
        let sprite = game.add_sprite(label.to_string(), base_path.join(label.to_string() + ".png"));
        sprite.scale = 0.38;
        sprite.translation += BOARD_OFFSET;

        if let GameState::Match(state) = self {
            if state.player_color == PlayerColor::Black {
                sprite.rotation = LEFT;
            }
        }

        // Board itself
        let label = "just_board";
        let sprite = game.add_sprite(label.to_string(), base_path.join(label.to_string() + ".png"));
        sprite.scale = 0.5;
        sprite.translation += BOARD_OFFSET;

        match self {
            GameState::Lobby(_) => todo!(),
            GameState::Match(match_state) => {

                for i in 0.. match_state.board.len() {
                    if let Some(figure) = match_state.board[i] {
                        let x = (i % 8) as f32;
                        let y = (i / 8) as f32;
                        let offset = (vec2((x) * SQUARE_SIZE,(y) * SQUARE_SIZE)) + FIGURE_SHIFT_FROM_CENTER + BOARD_OFFSET;
                        let sprite = game.add_sprite(figure.label.to_string(), base_path.join(figure.label[0..figure.label.len() - 1].to_string() + ".png"));
                        sprite.scale = FIGURE_SCALE;
                        sprite.translation = offset;
                        sprite.layer = 1.0;
                    }
                }
            },
        }
    }
}
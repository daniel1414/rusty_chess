use std::path::Path;

use rusty_engine::prelude::bevy::math::vec2;
use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::match_state::{MatchState, PlayerColor},
    game_state::GameState,
};

/// Offset of the center of the board from the center of the window.
const BOARD_OFFSET: Vec2 = vec2(-150.0, 0.0);

/// Scale of every figure.
const FIGURE_SCALE: f32 = 0.25;

/// Size of a square (precalculated)
const SQUARE_SIZE: f32 = 75.0;

/// Shift to the first position on the board - top left.
const FIGURE_SHIFT_FROM_CENTER: Vec2 =
    vec2(-(4.0 * SQUARE_SIZE - 37.5), -(4.0 * SQUARE_SIZE - 37.5));

const ASSETS_PATH: &str = "sprite/chess";

pub fn on_startup(game: &mut Game<GameState>, game_state: &GameState) {
    // Board frame
    let label = "board_frame";
    let base_path = Path::new(ASSETS_PATH);
    let sprite = game.add_sprite(
        label.to_string(),
        base_path.join(label.to_string() + ".png"),
    );
    sprite.scale = 0.38;
    sprite.translation += BOARD_OFFSET;

    if let GameState::Match(state) = &game_state {
        if state.player_color == PlayerColor::Black {
            sprite.rotation = LEFT;
        }
    }

    // Board itself
    let label = "just_board";
    let sprite = game.add_sprite(
        label.to_string(),
        base_path.join(label.to_string() + ".png"),
    );
    sprite.scale = 0.5;
    sprite.translation += BOARD_OFFSET;
}

pub fn render(engine: &mut Engine, game_state: &mut GameState) {
    match game_state {
        GameState::Lobby(state) => render_lobby(engine, state),
        GameState::Match(state) => render_match(engine, state),
    }
}

fn render_lobby(_: &mut Engine, _: &mut LobbyState) {
    todo!()
}

fn render_match(engine: &mut Engine, match_state: &mut MatchState) {
    let base_path = Path::new(ASSETS_PATH);

    if !match_state.is_dirty {
        return;
    }

    for i in 0..match_state.board.len() {
        if let Some(figure) = match_state.board[i] {
            let x = (i % 8) as f32;
            let y = (i / 8) as f32;
            let offset = (vec2((x) * SQUARE_SIZE, (y) * SQUARE_SIZE))
                + FIGURE_SHIFT_FROM_CENTER
                + BOARD_OFFSET;
            let sprite = engine.add_sprite(
                figure.label.to_string(),
                base_path.join(figure.label[0..figure.label.len() - 1].to_string() + ".png"),
            );
            sprite.scale = FIGURE_SCALE;
            sprite.translation = offset;
            sprite.layer = 1.0;
        }
    }

    match_state.is_dirty = false;
}

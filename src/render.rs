use std::path::Path;

use rusty_engine::prelude::bevy::math::vec2;
use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::match_state::{MatchState, PlayerColor, SquarePosition},
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

const ASSET_PATH: &str = "sprite/chess";

pub fn is_pixel_on_board(mut pos: Vec2) -> bool {
    pos -= BOARD_OFFSET + FIGURE_SHIFT_FROM_CENTER - 37.5;
    (pos.x > 0.0 && pos.x < 8.0 * SQUARE_SIZE) && (pos.y > 0.0 && pos.y < 8.0 * SQUARE_SIZE)
}

pub fn pixel_to_square(mut pos: Vec2) -> SquarePosition {
    pos -= BOARD_OFFSET + FIGURE_SHIFT_FROM_CENTER - 37.5;
    pos /= SQUARE_SIZE;

    SquarePosition::new(pos.x as u8, pos.y as u8)
}

fn square_to_pixel(pos: (u8, u8)) -> Vec2 {
    vec2(pos.0 as f32, pos.1 as f32) * SQUARE_SIZE + BOARD_OFFSET + FIGURE_SHIFT_FROM_CENTER
}

pub fn on_startup(game: &mut Game<GameState>, game_state: &GameState) {
    // Board frame
    let label = "board_frame";
    let base_path = Path::new(ASSET_PATH);
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
    let base_path = Path::new(ASSET_PATH);

    if !match_state.is_dirty {
        return;
    }

    // Draw all pieces.
    for i in 0..match_state.board.squares.len() {
        if let Some(figure) = match_state.board.squares[i] {
            let x = (i % 8) as u8;
            let y = (i / 8) as u8;
            let offset = square_to_pixel((x, y));
            let sprite = engine.add_sprite(
                figure.label.to_string(),
                base_path.join(figure.label[0..figure.label.len() - 1].to_string() + ".png"),
            );
            sprite.scale = FIGURE_SCALE;
            sprite.translation = offset;
            sprite.layer = 1.0;
        }
    }

    // Draw selected piece.
    if let (Some(pos_figure), Some(location)) = (
        match_state.selected_piece.as_ref(),
        engine.mouse_state.location(),
    ) {
        let figure = pos_figure.col_figure;
        let offset = location;
        let sprite = engine.add_sprite(
            figure.label.to_string(),
            base_path.join(figure.label[0..figure.label.len() - 1].to_string() + ".png"),
        );
        sprite.scale = FIGURE_SCALE;
        sprite.translation = offset;
        sprite.layer = 2.0;
    }

    // Draw all possible moves.

    if !match_state.available_moves.is_empty() {
        let mut i = 0;
        for pos in match_state.available_moves.iter() {
            let sprite = engine.add_sprite(
                format!("possible_move{}", i),
                base_path.join("possible_move.png"),
            );
            sprite.scale = FIGURE_SCALE * 0.75;
            sprite.translation = square_to_pixel((pos.x, pos.y));
            sprite.layer = 0.9;

            i += 1;
        }
    } else {
        engine.sprites.remove_entry("possible_move0");
        engine.sprites.remove_entry("possible_move1");
        engine.sprites.remove_entry("possible_move2");
        engine.sprites.remove_entry("possible_move3");
        engine.sprites.remove_entry("possible_move4");
    }

    match_state.is_dirty = false;
}

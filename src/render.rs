use std::path::Path;

use rusty_engine::prelude::bevy::math::vec2;
use rusty_engine::prelude::*;

use crate::{
    game_lobby::lobby_state::LobbyState,
    game_match::{
        match_state::{MatchState, PlayerColor},
        square::SquarePosition,
    },
    game_state::GameState,
};

/// Size of the board in pixels.
const BOARD_SIZE: u32 = 1200;

/// Scale of the board frame sprite.
const BOARD_FRAME_SCALE: f32 = 0.38;

/// Scale of the chess board sprite.
const BOARD_SCALE: f32 = 0.5;

/// Offset of the center of the board from the center of the window.
const BOARD_OFFSET: Vec2 = vec2(-150.0, 20.0);

/// Scale of every piece.
const CHESS_PIECE_SCALE: f32 = BOARD_SCALE * 0.5;

/// Size of a square.
const SQUARE_SIZE: f32 = (BOARD_SCALE * (BOARD_SIZE as f32)) / 8.0;

/// Scale of the move indicator. A little smaller than the actual chess piece.
const MOVE_INDICATOR_SCALE: f32 = CHESS_PIECE_SCALE * 0.75;

/// Size of half a square.
const SQUARE_HALF_SIZE: f32 = SQUARE_SIZE / 2.0;

/// Shift to the first position on the board - bottom left, as the y axis goes up.
const CHESS_PIECE_SHIFT_FROM_CENTER: Vec2 = vec2(
    -(4.0 * SQUARE_SIZE - SQUARE_HALF_SIZE),
    -(4.0 * SQUARE_SIZE - SQUARE_HALF_SIZE),
);

/// Layer of the active (selected) chess piece.
const ACTIVE_CHESS_PIECE_LAYER: f32 = 2.0;

/// Layer of the all the inactive chess pieces (on the board, but not selected).
const INACTIVE_CHESS_PIECE_LAYER: f32 = 1.0;

/// Layer of the available move indicators.
const MOVE_INDICATOR_LAYER: f32 = 0.5;

/// Path to chess assets.
const ASSET_PATH: &str = "sprite/chess";

/// Determines if a pixel location is a location on the chess board.
pub fn is_pixel_on_board(mut pos: Vec2) -> bool {
    pos -= BOARD_OFFSET + CHESS_PIECE_SHIFT_FROM_CENTER - SQUARE_HALF_SIZE;
    (pos.x > 0.0 && pos.x < 8.0 * SQUARE_SIZE) && (pos.y > 0.0 && pos.y < 8.0 * SQUARE_SIZE)
}

/// Converts a pixel location to a board square position.
pub fn pixel_to_square(mut pos: Vec2) -> SquarePosition {
    pos -= BOARD_OFFSET + CHESS_PIECE_SHIFT_FROM_CENTER - SQUARE_HALF_SIZE;
    pos /= SQUARE_SIZE;

    SquarePosition::new(pos.x as u8, pos.y as u8)
}

/// Returns the center of a board square position in pixels.
fn square_to_pixel(pos: (u8, u8)) -> Vec2 {
    vec2(pos.0 as f32, pos.1 as f32) * SQUARE_SIZE + BOARD_OFFSET + CHESS_PIECE_SHIFT_FROM_CENTER
}

/// Does the initial rendering on app startup.
pub fn on_startup(game: &mut Game<GameState>, game_state: &GameState) {
    // Board frame
    let label = "board_frame";
    let base_path = Path::new(ASSET_PATH);
    let sprite = game.add_sprite(
        label.to_string(),
        base_path.join(label.to_string() + ".png"),
    );
    sprite.scale = BOARD_FRAME_SCALE;
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
    sprite.scale = BOARD_SCALE;
    sprite.translation += BOARD_OFFSET;
}

/// Renders the game.
pub fn render(engine: &mut Engine, game_state: &mut GameState) {
    match game_state {
        GameState::Lobby(state) => render_lobby(engine, state),
        GameState::Match(state) => render_match(engine, state),
    }
}

/// Renders the lobby (NO LOBBY YET)
fn render_lobby(_: &mut Engine, _: &mut LobbyState) {
    todo!()
}

/// Renders the match (board, pieces, etc.)
fn render_match(engine: &mut Engine, match_state: &mut MatchState) {
    let base_path = Path::new(ASSET_PATH);

    if !match_state.is_dirty {
        return;
    }

    // Draw all pieces.
    for i in 0..match_state.board.squares.len() {
        if let Some(piece) = match_state.board.squares[i] {
            let x = (i % 8) as u8;
            let y = (i / 8) as u8;
            let offset = square_to_pixel((x, y));
            let sprite = engine.add_sprite(
                piece.label.to_string(),
                base_path.join(piece.label[0..piece.label.len() - 1].to_string() + ".png"),
            );
            sprite.scale = CHESS_PIECE_SCALE;
            sprite.translation = offset;
            sprite.layer = INACTIVE_CHESS_PIECE_LAYER;
        }
    }

    // Draw selected piece.
    if let (Some(pos_figure), Some(location)) = (
        match_state.selected_piece.as_ref(),
        engine.mouse_state.location(),
    ) {
        let piece = pos_figure.col_figure;
        let offset = location;
        let sprite = engine.add_sprite(
            piece.label.to_string(),
            base_path.join(piece.label[0..piece.label.len() - 1].to_string() + ".png"),
        );
        sprite.scale = CHESS_PIECE_SCALE;
        sprite.translation = offset;
        sprite.layer = ACTIVE_CHESS_PIECE_LAYER;
    }

    // Draw all possible moves.
    if !match_state.available_moves.is_empty() {
        let mut i = 0;
        for pos in match_state.available_moves.iter() {
            let sprite = engine.add_sprite(
                format!("possible_move{}", i),
                base_path.join("possible_move.png"),
            );
            sprite.scale = MOVE_INDICATOR_SCALE;
            sprite.translation = square_to_pixel((pos.x, pos.y));
            sprite.layer = MOVE_INDICATOR_LAYER;

            i += 1;
        }
    } else {
        // TODO: Need to store the labels in use for future removal.
        engine.sprites.remove_entry("possible_move0");
        engine.sprites.remove_entry("possible_move1");
        engine.sprites.remove_entry("possible_move2");
        engine.sprites.remove_entry("possible_move3");
        engine.sprites.remove_entry("possible_move4");
    }

    match_state.is_dirty = false;
}

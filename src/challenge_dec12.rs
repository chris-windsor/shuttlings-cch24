use std::{sync::Arc, usize};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::AppState;

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardTile {
    Cookie,
    Milk,
}

#[derive(Debug)]
pub struct MilkCookieGame {
    board_state: Vec<Vec<BoardTile>>,
    size: usize,
}

const WALL_TILE: &str = "⬜";
const BLANK_TILE: &str = "⬛";
const COOKIE_TILE: &str = "🍪";
const MILK_TILE: &str = "🥛";

impl MilkCookieGame {
    pub fn new(size: usize) -> Self {
        MilkCookieGame {
            board_state: vec![Vec::with_capacity(size); size],
            size,
        }
    }

    fn reset(&mut self) {
        self.board_state = vec![Vec::with_capacity(self.size); self.size]
    }

    fn place(&mut self, tile: BoardTile, column: usize) {
        let column = self.board_state.get_mut(column - 1).unwrap();
        column.push(tile);
    }

    fn to_string(&self) -> String {
        let mut board = String::new();
        for row in 0..self.size {
            let mut new_line: String = WALL_TILE.to_string();
            for column in 0..self.size {
                let column = self.board_state.get(column).unwrap();
                let cell = column.get(self.size - row - 1);
                if let Some(existing_tile) = cell {
                    match existing_tile {
                        &BoardTile::Cookie => new_line.push_str(COOKIE_TILE),
                        &BoardTile::Milk => new_line.push_str(MILK_TILE),
                    }
                } else {
                    new_line.push_str(BLANK_TILE);
                }
            }

            new_line.push_str(WALL_TILE);
            new_line.push_str("\n");
            board.push_str(&new_line);
        }

        board.push_str(&WALL_TILE.repeat(self.board_state.len() + 2));
        board.push_str("\n");
        board
    }
}

pub async fn milk_cookie_game_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.milk_cookie_game.try_read().unwrap().to_string()
}

pub async fn milk_cookie_game_reset(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.milk_cookie_game.try_write().unwrap().reset();
    state.milk_cookie_game.try_read().unwrap().to_string()
}

pub async fn milk_cookie_game_place(
    State(state): State<Arc<AppState>>,
    Path((team, column)): Path<(String, usize)>,
) -> impl IntoResponse {
    let tile_kind = match team.as_str() {
        "cookie" => BoardTile::Cookie,
        "milk" => BoardTile::Milk,
        _ => return Err(Day12AppError::BadPlacement),
    };

    if column > state.milk_cookie_game.try_read().unwrap().size || column < 1 {
        return Err(Day12AppError::BadPlacement);
    }

    state
        .milk_cookie_game
        .try_write()
        .unwrap()
        .place(tile_kind, column);
    Ok(state.milk_cookie_game.try_read().unwrap().to_string())
}

pub enum Day12AppError {
    BadPlacement,
}

impl IntoResponse for Day12AppError {
    fn into_response(self) -> Response {
        match self {
            Day12AppError::BadPlacement => (StatusCode::BAD_REQUEST, ""),
        }
        .into_response()
    }
}

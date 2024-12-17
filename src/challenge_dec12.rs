use std::{fmt::Display, sync::Arc, usize};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::AppState;

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardTile {
    Wall,
    Blank,
    Cookie,
    Milk,
}

impl Display for BoardTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            BoardTile::Wall => "⬜",
            BoardTile::Blank => "⬛",
            BoardTile::Cookie => "🍪",
            BoardTile::Milk => "🥛",
        };
        write!(f, "{}", emoji)
    }
}

#[derive(Debug)]
pub struct MilkCookieGame {
    board_state: Vec<Vec<BoardTile>>,
    size: usize,
}

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

    fn check_columns(&self, tile: BoardTile) -> bool {
        for column in self.board_state.clone() {
            if column.len() == self.size && column.iter().all(|cell| cell == &tile) {
                return true;
            }
        }
        false
    }

    fn check_rows(&self, tile: BoardTile) -> bool {
        for row in 0..self.size {
            let row = self
                .board_state
                .iter()
                .filter_map(|column| {
                    if let Some(cell) = column.get(row) {
                        return Some(*cell);
                    }
                    None
                })
                .collect::<Vec<_>>();
            if row.len() == self.size && row.iter().all(|cell| cell == &tile) {
                return true;
            }
        }
        false
    }

    fn check_diagonals(&self, tile: BoardTile) -> bool {
        let mut row = 0;
        let mut diag: Vec<BoardTile> = Vec::new();
        for column in 0..self.size {
            let cell = self.board_state.get(column).unwrap().get(row);
            if let Some(cell) = cell {
                diag.push(*cell);
            }
            row += 1;
        }

        if diag.len() == self.size && diag.iter().all(|cell| cell == &tile) {
            return true;
        }

        let mut diag: Vec<BoardTile> = Vec::new();
        let mut row = self.size;

        for column in 0..self.size {
            let cell = self.board_state.get(column).unwrap().get(row - 1);
            if let Some(cell) = cell {
                diag.push(*cell);
            }
            row -= 1;
        }

        if diag.len() == self.size && diag.iter().all(|cell| cell == &tile) {
            return true;
        }

        false
    }

    fn is_game_over(&self) -> bool {
        self.board_state
            .iter()
            .all(|column| column.len() == self.size)
    }

    fn get_winner(&self) -> Option<String> {
        for tile in [BoardTile::Cookie, BoardTile::Milk] {
            let won_column = self.check_columns(tile);
            let won_row = self.check_rows(tile);
            let won_diag = self.check_diagonals(tile);

            if won_column || won_row || won_diag {
                return Some(format!("{tile} wins!"));
            }
        }

        None
    }

    fn can_play(&self) -> bool {
        !self.get_winner().is_some()
    }

    fn to_string(&self) -> String {
        let mut board = String::new();
        for row in 0..self.size {
            let mut new_line: String = BoardTile::Wall.to_string();
            for column in 0..self.size {
                let column = self.board_state.get(column).unwrap();
                let cell = column.get(self.size - row - 1);
                if let Some(existing_tile) = cell {
                    new_line.push_str(&existing_tile.to_string());
                } else {
                    new_line.push_str(&BoardTile::Blank.to_string());
                }
            }

            new_line.push_str(&BoardTile::Wall.to_string());
            new_line.push_str("\n");
            board.push_str(&new_line);
        }

        board.push_str(
            &BoardTile::Wall
                .to_string()
                .repeat(self.board_state.len() + 2),
        );
        board.push_str("\n");

        if let Some(winner_message) = self.get_winner() {
            board.push_str(&winner_message);
            board.push_str("\n");
        }

        if self.is_game_over() {
            board.push_str(&format!("No winner.\n"));
        }

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

    if !state.milk_cookie_game.try_read().unwrap().can_play() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            state.milk_cookie_game.try_read().unwrap().to_string(),
        ));
    }

    state
        .milk_cookie_game
        .try_write()
        .unwrap()
        .place(tile_kind, column);

    Ok((
        StatusCode::OK,
        state.milk_cookie_game.try_read().unwrap().to_string(),
    ))
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

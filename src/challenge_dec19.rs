use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};

use crate::AppState;

pub async fn quotes_reset(state: State<Arc<AppState>>) -> impl IntoResponse {
    sqlx::query("TRUNCATE quotes")
        .execute(&state.pool)
        .await
        .unwrap();
    StatusCode::OK
}

#[derive(Deserialize)]
pub struct QuoteInsert {
    author: String,
    quote: String,
}

#[derive(FromRow, Serialize)]
struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

async fn retrieve_quote_by_id(id: &Uuid, pool: &sqlx::PgPool) -> Result<Quote, Day19AppError> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(quote)
}

pub async fn quote_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Day19AppError> {
    let quote = retrieve_quote_by_id(&id, &state.pool).await?;
    Ok((StatusCode::OK, Json(quote)))
}

pub async fn remove_quote_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Day19AppError> {
    let quote = retrieve_quote_by_id(&id, &state.pool).await?;

    sqlx::query("DELETE FROM quotes WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok((StatusCode::OK, Json(quote)))
}

pub async fn undo_quote_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(new_quote): Json<QuoteInsert>,
) -> Result<impl IntoResponse, Day19AppError> {
    sqlx::query(
        "UPDATE quotes SET (version, quote, author) = (version+1, $1, $2)
  WHERE id = $3",
    )
    .bind(new_quote.quote)
    .bind(new_quote.author)
    .bind(&id)
    .execute(&state.pool)
    .await?;

    let quote = retrieve_quote_by_id(&id, &state.pool).await?;

    Ok((StatusCode::OK, Json(quote)))
}

pub async fn draft_quote(
    state: State<Arc<AppState>>,
    Json(new_quote): Json<QuoteInsert>,
) -> Result<impl IntoResponse, Day19AppError> {
    let id = Uuid::new_v4();
    let utc: DateTime<Utc> = Utc::now();

    sqlx::query("INSERT INTO quotes (id, created_at, author, quote) VALUES ($1, $2, $3, $4)")
        .bind(&id)
        .bind(utc)
        .bind(new_quote.author)
        .bind(new_quote.quote)
        .execute(&state.pool)
        .await?;

    let quote = retrieve_quote_by_id(&id, &state.pool).await?;

    Ok((StatusCode::CREATED, Json(quote)))
}

#[derive(Deserialize)]
pub struct PaginationParams {
    token: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedQuotes {
    quotes: Vec<Quote>,
    page: i32,
    next_token: Option<String>,
}

const PAGINATION_PAGE_SIZE: i32 = 3;

async fn retrieve_quotes_page(
    offset: &i32,
    pool: &sqlx::PgPool,
) -> Result<Vec<Quote>, Day19AppError> {
    let quote = sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes ORDER BY created_at ASC OFFSET $1 LIMIT $2",
    )
    .bind(offset)
    .bind(PAGINATION_PAGE_SIZE)
    .fetch_all(pool)
    .await?;

    Ok(quote)
}

pub async fn paginated_quotes(
    state: State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, Day19AppError> {
    let new_token = random_token();
    let offset: i32;

    if let Some(token) = params.token {
        let pagination_map = state.quote_pagination.try_read().unwrap();
        match pagination_map.get(&token) {
            Some(offset_value) => offset = offset_value.clone() + PAGINATION_PAGE_SIZE,
            None => return Err(Day19AppError::BadToken),
        }
    } else {
        offset = 0;
    };
    state
        .quote_pagination
        .try_write()
        .unwrap()
        .insert(new_token.clone(), offset);

    let quotes = retrieve_quotes_page(&offset, &state.pool).await?;
    let next_quotes = retrieve_quotes_page(&(offset + PAGINATION_PAGE_SIZE), &state.pool).await?;
    let next_token = if next_quotes.len() > 0 {
        Some(new_token)
    } else {
        None
    };

    Ok((
        StatusCode::OK,
        Json(PaginatedQuotes {
            quotes,
            page: (offset / PAGINATION_PAGE_SIZE) + 1,
            next_token,
        }),
    ))
}

fn random_token() -> String {
    let mut rng = rand::thread_rng();
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    (0..16)
        .map(|_| {
            let index = rng.gen_range(0..alphabet.len());
            alphabet[index].to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}

pub enum Day19AppError {
    SqlXError(sqlx::Error),
    BadToken,
}

impl IntoResponse for Day19AppError {
    fn into_response(self) -> Response {
        match self {
            Day19AppError::SqlXError(rejection) => {
                println!("{}", rejection);
                match rejection {
                    sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, ""),
                    _ => (StatusCode::BAD_REQUEST, ""),
                }
            }
            Day19AppError::BadToken => (StatusCode::BAD_REQUEST, ""),
        }
        .into_response()
    }
}

impl From<sqlx::Error> for Day19AppError {
    fn from(rejection: sqlx::Error) -> Self {
        Self::SqlXError(rejection)
    }
}

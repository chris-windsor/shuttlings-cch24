use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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

pub enum Day19AppError {
    SqlXError(sqlx::Error),
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
        }
        .into_response()
    }
}

impl From<sqlx::Error> for Day19AppError {
    fn from(rejection: sqlx::Error) -> Self {
        Self::SqlXError(rejection)
    }
}

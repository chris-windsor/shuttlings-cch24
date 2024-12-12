use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum ConversionPayload {
    liters(f32),
    gallons(f32),
    litres(f32),
    pints(f32),
}

fn convert_unit(unit: ConversionPayload) -> Value {
    match unit {
        ConversionPayload::liters(val) => json!({
            "gallons": val * 0.264172
        }),
        ConversionPayload::gallons(val) => json!({
            "liters": val * 3.78541
        }),
        ConversionPayload::litres(val) => json!({
            "pints": val * 1.7597539864
        }),
        ConversionPayload::pints(val) => json!({
            "litres": val * 0.568261
        }),
    }
}

pub async fn milk_bucket_leaky(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, Day9AppError> {
    if state.leaky_milk_bucket.tokens() == 0 {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            String::from("No milk available\n"),
        ));
    }

    state.leaky_milk_bucket.acquire_one().await;

    match headers.get(CONTENT_TYPE) {
        Some(content_type) if content_type == "application/json" => {
            let conversion_payload: ConversionPayload = serde_json::from_slice(&body)?;
            let conversion_result = convert_unit(conversion_payload);
            return Ok((StatusCode::OK, conversion_result.to_string()));
        }
        _ => {}
    }

    Ok((StatusCode::OK, String::from("Milk withdrawn\n")))
}

pub enum Day9AppError {
    JSONError(serde_json::Error),
}

impl IntoResponse for Day9AppError {
    fn into_response(self) -> Response {
        match self {
            Day9AppError::JSONError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, "")
            }
        }
        .into_response()
    }
}

impl From<serde_json::Error> for Day9AppError {
    fn from(rejection: serde_json::Error) -> Self {
        Self::JSONError(rejection)
    }
}

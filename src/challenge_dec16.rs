use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use biscotti::{Processor, ProcessorConfig, RequestCookies};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    present: String,
    exp: usize,
}

const SIGNING_SECRET: &str = "santa";

pub async fn wrap_present(present: Json<Value>) -> impl IntoResponse {
    let claims = Claims {
        present: present.to_string(),
        exp: 1_000_000_000_000,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SIGNING_SECRET.as_ref()),
    )
    .unwrap();

    (
        StatusCode::OK,
        [(header::SET_COOKIE, format!("gift={token}"))],
    )
}

pub async fn unwrap_present(headers: HeaderMap) -> impl IntoResponse {
    let processor: Processor = ProcessorConfig::default().into();

    let cookie_header = headers.get(header::COOKIE);

    if !cookie_header.is_some() {
        return (StatusCode::BAD_REQUEST, Json(json!({})));
    }

    let cookie_header = cookie_header.unwrap().to_str().unwrap();
    let cookies = RequestCookies::parse_header(cookie_header, &processor).unwrap();
    let gift_cookie = cookies.get("gift");

    if !gift_cookie.is_some() {
        return (StatusCode::BAD_REQUEST, Json(json!({})));
    }

    let gift_cookie = gift_cookie.unwrap();
    let gift_encoded = gift_cookie.value();

    let token = decode::<Claims>(
        &gift_encoded,
        &DecodingKey::from_secret(SIGNING_SECRET.as_ref()),
        &Validation::default(),
    )
    .unwrap();

    let gift = serde_json::from_str::<Value>(&token.claims.present).unwrap();

    (StatusCode::OK, Json(gift))
}

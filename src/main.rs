use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{
    routing::{get, post},
    Router,
};
use challenge_dec12::{
    milk_cookie_game_place, milk_cookie_game_reset, milk_cookie_game_state, milk_cookie_not_random,
    MilkCookieGame,
};
use challenge_dec2::{
    egregious_encryption_dest, egregious_encryption_dest_v6, egregious_encryption_key,
    egregious_encryption_key_v6,
};
use challenge_dec5::car_go_festivity;
use challenge_dec9::{milk_bucket_leaky, milk_bucket_refill};
use challenge_intro::{hello_bird, seek_and_find};
use leaky_bucket_lite::LeakyBucket;

mod challenge_dec12;
mod challenge_dec2;
mod challenge_dec5;
mod challenge_dec9;
mod challenge_intro;

struct AppState {
    leaky_milk_bucket: LeakyBucket,
    milk_cookie_game: RwLock<MilkCookieGame>,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let leaky_milk_bucket = LeakyBucket::builder()
        .max(5)
        .tokens(5)
        .refill_interval(Duration::from_secs(1))
        .refill_amount(1)
        .build();

    let milk_cookie_game = RwLock::new(MilkCookieGame::new(4));

    let app_state = Arc::new(AppState {
        leaky_milk_bucket,
        milk_cookie_game,
    });

    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(seek_and_find))
        .route("/2/dest", get(egregious_encryption_dest))
        .route("/2/key", get(egregious_encryption_key))
        .route("/2/v6/dest", get(egregious_encryption_dest_v6))
        .route("/2/v6/key", get(egregious_encryption_key_v6))
        .route("/5/manifest", post(car_go_festivity))
        .route("/9/milk", post(milk_bucket_leaky))
        .route("/9/refill", post(milk_bucket_refill))
        .route("/12/board", get(milk_cookie_game_state))
        .route("/12/reset", post(milk_cookie_game_reset))
        .route("/12/place/:team/:column", post(milk_cookie_game_place))
        .route("/12/random-board", get(milk_cookie_not_random))
        .with_state(app_state);

    Ok(router.into())
}

use std::{sync::Arc, time::Duration};

use axum::{
    routing::{get, post},
    Router,
};
use challenge_dec2::{
    egregious_encryption_dest, egregious_encryption_dest_v6, egregious_encryption_key,
    egregious_encryption_key_v6,
};
use challenge_dec5::car_go_festivity;
use challenge_dec9::milk_bucket_leaky;
use challenge_intro::{hello_bird, seek_and_find};
use leaky_bucket_lite::LeakyBucket;

mod challenge_dec2;
mod challenge_dec5;
mod challenge_dec9;
mod challenge_intro;

struct AppState {
    leaky_milk_bucket: LeakyBucket,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let leaky_milk_bucket = LeakyBucket::builder()
        .max(5)
        .tokens(5)
        .refill_interval(Duration::from_secs(1))
        .refill_amount(1)
        .build();

    let app_state = Arc::new(AppState { leaky_milk_bucket });

    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(seek_and_find))
        .route("/2/dest", get(egregious_encryption_dest))
        .route("/2/key", get(egregious_encryption_key))
        .route("/2/v6/dest", get(egregious_encryption_dest_v6))
        .route("/2/v6/key", get(egregious_encryption_key_v6))
        .route("/5/manifest", post(car_go_festivity))
        .route("/9/milk", post(milk_bucket_leaky))
        .with_state(app_state);

    Ok(router.into())
}

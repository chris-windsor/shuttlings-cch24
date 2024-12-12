use axum::{
    routing::{get, post},
    Router,
};
use challenge_dec2::{
    egregious_encryption_dest, egregious_encryption_dest_v6, egregious_encryption_key,
    egregious_encryption_key_v6,
};
use challenge_dec5::car_go_festivity;
use challenge_intro::{hello_bird, seek_and_find};

mod challenge_dec2;
mod challenge_dec5;
mod challenge_intro;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(seek_and_find))
        .route("/2/dest", get(egregious_encryption_dest))
        .route("/2/key", get(egregious_encryption_key))
        .route("/2/v6/dest", get(egregious_encryption_dest_v6))
        .route("/2/v6/key", get(egregious_encryption_key_v6))
        .route("/5/manifest", post(car_go_festivity));

    Ok(router.into())
}

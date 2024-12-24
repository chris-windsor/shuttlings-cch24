use axum::{
    body::Bytes,
    extract::{Multipart, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

pub async fn htmx_star() -> impl IntoResponse {
    fhtml::format! {
        <div id="star" class="lit"></div>
    }
}

pub async fn htmx_present_color(
    Path(current_color): Path<String>,
) -> Result<impl IntoResponse, Day23AppError> {
    let current_color = fhtml::escape(current_color);
    let next_color = match &current_color[..] {
        "red" => "blue",
        "blue" => "purple",
        "purple" => "red",
        _ => return Err(Day23AppError::BadColor),
    };
    Ok(fhtml::format! {
        <div class={format_args!("present {}", current_color)} hx-get={format_args!("/23/present/{}", next_color)} hx-swap="outerHTML">
            <div class="ribbon"></div>
            <div class="ribbon"></div>
            <div class="ribbon"></div>
            <div class="ribbon"></div>
        </div>
    })
}

pub async fn htmx_css_animations(
    Path((animation_state, index)): Path<(String, String)>,
) -> Result<impl IntoResponse, Day23AppError> {
    let animation_state = fhtml::escape(animation_state);
    let index = fhtml::escape(index);
    let next_state = match &animation_state[..] {
        "on" => "off",
        "off" => "on",
        _ => return Err(Day23AppError::BadState),
    };
    let extra_class = match &animation_state[..] {
        "on" => " on",
        "off" => "",
        _ => return Err(Day23AppError::BadState),
    };
    Ok(fhtml::format! {
        <div class={format_args!("ornament{}", extra_class)} id={format_args!("ornament{}", index)} hx-trigger="load delay:2s once" hx-get={format_args!("/23/ornament/{}/{}", next_state, index)} hx-swap="outerHTML"></div>
    })
}

#[derive(Deserialize)]
struct CargoLock {
    package: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    checksum: Option<String>,
}

pub async fn htmx_form(mut multipart: Multipart) -> Result<impl IntoResponse, Day23AppError> {
    let mut data = Bytes::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Day23AppError::BadFile)?
    {
        let name = field.name().unwrap();
        if name == "lockfile" {
            data = field.bytes().await.map_err(|_| Day23AppError::BadFile)?;
        }
    }

    if data.is_empty() {
        return Err(Day23AppError::BadFile);
    }

    let file_contents = String::from_utf8(data.to_vec()).map_err(|_| Day23AppError::BadFile)?;

    let cargo_lock: CargoLock =
        toml::from_str(&file_contents).map_err(|_| Day23AppError::BadFile)?;

    let divs: Result<Vec<String>, Day23AppError> = cargo_lock
        .package
        .iter()
        .filter(|package| package.checksum.is_some())
        .map(|package| -> Result<String, Day23AppError> {
            let checksum = package.checksum.as_ref().unwrap();
            if checksum.len() < 10 {
                return Err(Day23AppError::BadChecksum);
            }
            let color_hex = i64::from_str_radix(&checksum[0..6], 16).map_err(|_| Day23AppError::BadChecksum)?;
            let color_hex = format!("{:06x}", color_hex);
            let top =
                i64::from_str_radix(&checksum[6..8], 16).map_err(|_| Day23AppError::BadChecksum)?;
            let left = i64::from_str_radix(&checksum[8..10], 16)
                .map_err(|_| Day23AppError::BadChecksum)?;

            let div: String = fhtml::format! {
                <div style={format_args!("background-color:#{};top:{}px;left:{}px;", color_hex, top, left)}></div>
            }.to_string();

            Ok(div)
        })
        .collect();

    let divs = divs.map_err(|e| e)?.join("");
    Ok(divs)
}

pub enum Day23AppError {
    BadColor,
    BadState,
    BadFile,
    BadChecksum,
}

impl IntoResponse for Day23AppError {
    fn into_response(self) -> Response {
        match self {
            Day23AppError::BadColor => (StatusCode::IM_A_TEAPOT, ""),
            Day23AppError::BadState => (StatusCode::IM_A_TEAPOT, ""),
            Day23AppError::BadFile => (StatusCode::BAD_REQUEST, ""),
            Day23AppError::BadChecksum => (StatusCode::UNPROCESSABLE_ENTITY, ""),
        }
        .into_response()
    }
}

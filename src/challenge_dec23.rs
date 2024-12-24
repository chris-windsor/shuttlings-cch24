use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

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

pub enum Day23AppError {
    BadColor,
    BadState,
}

impl IntoResponse for Day23AppError {
    fn into_response(self) -> Response {
        match self {
            Day23AppError::BadColor => (StatusCode::IM_A_TEAPOT, ""),
            Day23AppError::BadState => (StatusCode::IM_A_TEAPOT, ""),
        }
        .into_response()
    }
}

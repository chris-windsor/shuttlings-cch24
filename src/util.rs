use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    CargoManifestError(cargo_manifest::Error),
    NoMagicKeyword,
    NoContent,
    TomlParseError(toml::de::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::CargoManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, "Invalid manifest")
            }
            AppError::NoMagicKeyword => (StatusCode::BAD_REQUEST, "Magic keyword not provided"),
            AppError::NoContent => (StatusCode::NO_CONTENT, ""),
            AppError::TomlParseError(rejection) => {
                println!("{}", rejection);
                (StatusCode::NO_CONTENT, "")
            }
        }
        .into_response()
    }
}

impl From<cargo_manifest::Error> for AppError {
    fn from(rejection: cargo_manifest::Error) -> Self {
        Self::CargoManifestError(rejection)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(rejection: toml::de::Error) -> Self {
        Self::TomlParseError(rejection)
    }
}

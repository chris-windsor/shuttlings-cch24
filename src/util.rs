use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    UnsupportedMediaType,
    CargoManifestError(cargo_manifest::Error),
    YAMLManifestError(serde_yaml::Error),
    JSONManifestError(serde_json::Error),
    ManifestError,
    NoMagicKeyword,
    NoContent,
    TomlParseError(toml::de::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        const INVALID_MANIFEST_DETAIL: &str = "Invalid manifest";

        match self {
            AppError::UnsupportedMediaType => (StatusCode::UNSUPPORTED_MEDIA_TYPE, ""),
            AppError::CargoManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            AppError::YAMLManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            AppError::JSONManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            AppError::ManifestError => (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL),
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

impl From<serde_yaml::Error> for AppError {
    fn from(rejection: serde_yaml::Error) -> Self {
        Self::YAMLManifestError(rejection)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(rejection: serde_json::Error) -> Self {
        Self::JSONManifestError(rejection)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(rejection: toml::de::Error) -> Self {
        Self::TomlParseError(rejection)
    }
}

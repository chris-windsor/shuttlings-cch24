use std::str::FromStr;

use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use cargo_manifest::Manifest;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    package: Package,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
    metadata: Metadata,
    keywords: Option<Vec<String>>,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    orders: Vec<Table>,
}

#[axum::debug_handler]
pub async fn car_go_festivity(headers: HeaderMap, body: String) -> Result<String, Day5AppError> {
    let content_header = headers.get(CONTENT_TYPE);
    let content_type = content_header.unwrap().to_str().unwrap();

    let content: String;
    let config: Config;
    let manifest: Manifest;

    match content_type {
        "application/toml" => {
            manifest = Manifest::from_slice(&body.as_bytes())?;
            content = String::from(body.as_str());
        }
        "application/yaml" => {
            config = serde_yaml::from_slice(&body.as_bytes())?;
            let re_toml = toml::to_string(&config).unwrap();
            content = re_toml.clone();
            manifest = Manifest::from_str(&re_toml)?;
        }
        "application/json" => {
            config = serde_json::from_slice(&body.as_bytes())?;
            let re_toml = toml::to_string(&config).unwrap();
            content = re_toml.clone();
            manifest = Manifest::from_str(&re_toml)?;
        }
        _ => return Err(Day5AppError::UnsupportedMediaType),
    }

    let package = manifest.package.unwrap();

    if let Some(version) = package.rust_version {
        if version
            .as_local()
            .unwrap()
            .chars()
            .into_iter()
            .any(|char| char.is_alphabetic())
        {
            return Err(Day5AppError::ManifestError);
        }
    }

    if let Some(keywords) = package.keywords {
        let keywords = keywords.as_local().unwrap();
        if !keywords.contains(&String::from("Christmas 2024")) {
            return Err(Day5AppError::NoMagicKeyword);
        }
    } else {
        return Err(Day5AppError::NoMagicKeyword);
    }

    let config = toml::from_str::<Config>(&content)?;

    let order_items = config
        .package
        .metadata
        .orders
        .into_iter()
        .filter_map(|order| {
            let item = order.get("item").unwrap();
            let quantity = order.get("quantity");

            if quantity.is_some_and(|q| q.is_integer()) {
                return Some(format!(
                    "{}: {}",
                    item.as_str().unwrap(),
                    quantity.unwrap().as_integer().unwrap()
                ));
            }

            None
        })
        .collect::<Vec<_>>();

    if order_items.len() == 0 {
        return Err(Day5AppError::NoContent);
    }

    Ok(order_items.join("\n"))
}

pub enum Day5AppError {
    UnsupportedMediaType,
    CargoManifestError(cargo_manifest::Error),
    YAMLManifestError(serde_yaml::Error),
    JSONManifestError(serde_json::Error),
    ManifestError,
    NoMagicKeyword,
    NoContent,
    TomlParseError(toml::de::Error),
}

impl IntoResponse for Day5AppError {
    fn into_response(self) -> Response {
        const INVALID_MANIFEST_DETAIL: &str = "Invalid manifest";

        match self {
            Day5AppError::UnsupportedMediaType => (StatusCode::UNSUPPORTED_MEDIA_TYPE, ""),
            Day5AppError::CargoManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            Day5AppError::YAMLManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            Day5AppError::JSONManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            }
            Day5AppError::ManifestError => (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL),
            Day5AppError::NoMagicKeyword => (StatusCode::BAD_REQUEST, "Magic keyword not provided"),
            Day5AppError::NoContent => (StatusCode::NO_CONTENT, ""),
            Day5AppError::TomlParseError(rejection) => {
                println!("{}", rejection);
                (StatusCode::NO_CONTENT, "")
            }
        }
        .into_response()
    }
}

impl From<cargo_manifest::Error> for Day5AppError {
    fn from(rejection: cargo_manifest::Error) -> Self {
        Self::CargoManifestError(rejection)
    }
}

impl From<serde_yaml::Error> for Day5AppError {
    fn from(rejection: serde_yaml::Error) -> Self {
        Self::YAMLManifestError(rejection)
    }
}

impl From<serde_json::Error> for Day5AppError {
    fn from(rejection: serde_json::Error) -> Self {
        Self::JSONManifestError(rejection)
    }
}

impl From<toml::de::Error> for Day5AppError {
    fn from(rejection: toml::de::Error) -> Self {
        Self::TomlParseError(rejection)
    }
}

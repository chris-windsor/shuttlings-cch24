use std::str::FromStr;

use axum::http::{header::CONTENT_TYPE, HeaderMap};
use cargo_manifest::Manifest;
use serde::{Deserialize, Serialize};
use toml::Table;

use crate::util::AppError;

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
pub async fn car_go_festivity(headers: HeaderMap, body: String) -> Result<String, AppError> {
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
        _ => return Err(AppError::UnsupportedMediaType),
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
            return Err(AppError::ManifestError);
        }
    }

    if let Some(keywords) = package.keywords {
        let keywords = keywords.as_local().unwrap();
        if !keywords.contains(&String::from("Christmas 2024")) {
            return Err(AppError::NoMagicKeyword);
        }
    } else {
        return Err(AppError::NoMagicKeyword);
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
        return Err(AppError::NoContent);
    }

    Ok(order_items.join("\n"))
}

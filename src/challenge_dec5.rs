use axum::http::{header::CONTENT_TYPE, HeaderMap};
use cargo_manifest::Manifest;
use serde::Deserialize;
use toml::Table;

use crate::util::AppError;

#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

const MAGIC_KEYWORD: &str = "Christmas 2024";
impl Config {
    fn has_magic_keyword(&self) -> bool {
        self.package.keywords.contains(&String::from(MAGIC_KEYWORD))
    }
}

#[derive(Debug, Deserialize)]
struct Package {
    metadata: Metadata,
    keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    orders: Vec<Table>,
}

#[axum::debug_handler]
pub async fn car_go_festivity(headers: HeaderMap, body: String) -> Result<String, AppError> {
    let content_header = headers.get(CONTENT_TYPE);
    let content_type = content_header.unwrap().to_str().unwrap();

    dbg!(content_type);

    let config: Config;

    match content_type {
        "application/toml" => {
            let manifest = Manifest::from_slice(&body.as_bytes())?;

            if let Some(keywords) = manifest.package.unwrap().keywords {
                let keywords = keywords.as_local().unwrap();
                if !keywords.contains(&String::from(MAGIC_KEYWORD)) {
                    return Err(AppError::NoMagicKeyword);
                }
            } else {
                return Err(AppError::NoMagicKeyword);
            }

            config = toml::from_str::<Config>(&body)?;
        }
        "application/yaml" => {
            config = serde_yaml::from_slice(&body.as_bytes())?;

            if !config.has_magic_keyword() {
                return Err(AppError::NoMagicKeyword);
            }
        }
        "application/json" => {
            config = serde_json::from_slice(&body.as_bytes())?;

            if !config.has_magic_keyword() {
                return Err(AppError::NoMagicKeyword);
            }
        }
        _ => return Err(AppError::UnsupportedMediaType),
    }

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

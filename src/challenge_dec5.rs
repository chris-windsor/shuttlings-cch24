use cargo_manifest::Manifest;
use serde::Deserialize;
use toml::Table;

use crate::util::AppError;

#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    orders: Vec<Table>,
}

#[axum::debug_handler]
pub async fn car_go_festivity(body: String) -> Result<String, AppError> {
    let manifest = Manifest::from_slice(&body.as_bytes())?;

    if let Some(keywords) = manifest.package.unwrap().keywords {
        let keywords = keywords.as_local().unwrap();
        if !keywords.contains(&String::from("Christmas 2024")) {
            return Err(AppError::NoMagicKeyword);
        }
    } else {
        return Err(AppError::NoMagicKeyword);
    }

    let manifest = toml::from_str::<Config>(&body)?;

    let order_items = manifest
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

use std::collections::HashMap;

use axum::{
    http::{HeaderMap, StatusCode},
    response::{ErrorResponse, Result},
    Json,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use tracing::info;

pub async fn cookie(headers: HeaderMap) -> Result<String> {
    extract_cookie_header(&headers)
}

fn extract_cookie_header(headers: &HeaderMap) -> Result<String> {
    let cookie_b64 = headers
        .get("cookie")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .trim_start_matches("recipe=");
    info!("cookie b64: {cookie_b64}");
    let cookie_bytes = general_purpose::STANDARD
        .decode(cookie_b64)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let cookie_string = String::from_utf8(cookie_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(cookie_string)
}

#[derive(Deserialize)]
pub struct BakingRequest {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

impl TryFrom<&HeaderMap> for BakingRequest {
    type Error = ErrorResponse;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        let cookie_string = extract_cookie_header(value)?;
        let request: BakingRequest = serde_json::from_str(&cookie_string).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("failed to parse baking request, err: {err}"),
            )
        })?;
        Ok(request)
    }
}

#[derive(Serialize, Deserialize)]
struct Ingredients {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Serialize, Default)]
pub struct BakingResult {
    cookies: u32,
    pantry: HashMap<String, u32>,
}

impl BakingResult {
    fn from_request(request: BakingRequest) -> Self {
        let cookies = request
            .recipe
            .iter()
            .map(|(name, quantity)| {
                if let Some(stock) = request.pantry.get(name) {
                    stock / quantity
                } else {
                    0
                }
            })
            .min();
        let Some(cookies) = cookies else {
            return Self {
                cookies: 0,
                pantry: request.pantry,
            };
        };
        let pantry =
            request
                .pantry
                .into_iter()
                .fold(HashMap::new(), |mut acc, (name, quantity)| {
                    if let Some(recipe_quantity) = request.recipe.get(&name) {
                        acc.insert(name, quantity - *recipe_quantity * cookies);
                    } else {
                        acc.insert(name, quantity);
                    }
                    acc
                });
        Self { cookies, pantry }
    }
}

pub async fn bake(headers: HeaderMap) -> Result<Json<BakingResult>> {
    let cookie_string = extract_cookie_header(&headers)?;
    info!("cookie string: {cookie_string}");
    let request = serde_json::from_str::<BakingRequest>(&cookie_string)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(BakingResult::from_request(request)))
}

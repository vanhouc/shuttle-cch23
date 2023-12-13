use std::{collections::HashMap, fmt::Display};

use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{ErrorResponse, Result},
    routing::{get, post},
    Json, Router,
};
use base64::{
    engine::general_purpose::{self},
    Engine,
};
use serde::{Deserialize, Serialize};
use tracing::info;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn hello_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn nums(Path(nums): Path<String>) -> Result<String> {
    let ids: Vec<u32> = nums
        .split('/')
        .map(|id| {
            id.parse()
                .map_err(|_| ErrorResponse::from(StatusCode::BAD_REQUEST))
        })
        .collect::<Result<Vec<_>>>()?;
    let response = ids
        .into_iter()
        .reduce(|acc, id| acc ^ id)
        .ok_or(StatusCode::BAD_REQUEST)?
        .pow(3)
        .to_string();
    Ok(response)
}
#[derive(Deserialize)]
struct ReindeerSummary {
    // name: String,
    strength: u32,
}

async fn strength(Json(reindeer): Json<Vec<ReindeerSummary>>) -> String {
    reindeer
        .into_iter()
        .map(|r| r.strength)
        .sum::<u32>()
        .to_string()
}

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    // favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

impl Display for Reindeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize)]
struct CompetitionResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(Json(reindeer): Json<Vec<Reindeer>>) -> Result<Json<CompetitionResults>> {
    let fastest = reindeer
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .ok_or(StatusCode::BAD_REQUEST)?;
    let tallest = reindeer
        .iter()
        .max_by_key(|r| r.height)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let magician = reindeer
        .iter()
        .max_by_key(|r| r.snow_magic_power)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let consumer = reindeer
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let results = CompetitionResults {
        fastest: format!(
            "Speeding past the finish line with a strength of {strength} is {fastest}",
            strength = fastest.strength
        ),
        tallest: format!(
            "{tallest} is standing tall with his {width} cm wide antlers",
            width = tallest.antler_width
        ),
        magician: format!(
            "{magician} could blast you away with a snow magic power of {magic}",
            magic = magician.snow_magic_power
        ),
        consumer: format!("{consumer} ate lots of candies, but also some grass"),
    };
    Ok(Json(results))
}

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelves: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelves: usize,
}

async fn elf(body: String) -> Json<ElfCount> {
    let elf = body.matches("elf").count();
    let elf_shelves = body.matches("elf on a shelf").count();
    let shelves = body.matches("shelf").count() - elf_shelves;
    Json(ElfCount {
        elf,
        elf_shelves,
        shelves,
    })
}

async fn cookie(headers: HeaderMap) -> Result<String> {
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
struct BakingRequest {
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
struct BakingResult {
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

async fn bake(headers: HeaderMap) -> Result<Json<BakingResult>> {
    let cookie_string = extract_cookie_header(&headers)?;
    info!("cookie string: {cookie_string}");
    let request = serde_json::from_str::<BakingRequest>(&cookie_string)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(BakingResult::from_request(request)))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(hello_error))
        .route("/1/*nums", get(nums))
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
        .route("/6", post(elf))
        .route("/7/decode", get(cookie))
        .route("/7/bake", get(bake));
    Ok(router.into())
}

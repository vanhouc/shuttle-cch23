use std::fmt::Display;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{ErrorResponse, Result},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(hello_error))
        .route("/1/*nums", get(nums))
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest));
    Ok(router.into())
}

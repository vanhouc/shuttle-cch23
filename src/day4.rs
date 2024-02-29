use std::fmt::Display;

use axum::{http::StatusCode, response::Result, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReindeerSummary {
    // name: String,
    strength: u32,
}

pub async fn strength(Json(reindeer): Json<Vec<ReindeerSummary>>) -> String {
    reindeer
        .into_iter()
        .map(|r| r.strength)
        .sum::<u32>()
        .to_string()
}

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

impl Display for Reindeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize)]
pub struct CompetitionResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn contest(Json(reindeer): Json<Vec<Reindeer>>) -> Result<Json<CompetitionResults>> {
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
        consumer: format!(
            "{consumer} ate lots of candies, but also some {favorite_food}",
            favorite_food = consumer.favorite_food
        ),
    };
    Ok(Json(results))
}

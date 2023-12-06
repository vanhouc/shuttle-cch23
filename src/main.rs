use axum::{
    extract::Path,
    http::StatusCode,
    response::{ErrorResponse, Result},
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn hello_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day1(Path(ids): Path<String>) -> Result<String> {
    let ids: Vec<u32> = ids
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(hello_error))
        .route("/1/*id", get(day1));

    Ok(router.into())
}

use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn hello_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day1(Path((num1, num2)): Path<(u64, u64)>) -> String {
    (num1 ^ num2).pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(hello_error))
        .route("/1/:num1/:num2", get(day1));

    Ok(router.into())
}

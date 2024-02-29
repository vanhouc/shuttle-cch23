use axum::http::StatusCode;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn hello_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

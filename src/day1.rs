use axum::{
    extract::Path,
    http::StatusCode,
    response::{ErrorResponse, Result},
};

pub async fn nums(Path(nums): Path<String>) -> Result<String> {
    let ids: Vec<i64> = nums
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

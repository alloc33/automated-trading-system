use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::App;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth<B>(
    State(app): State<Arc<App>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // let api_key = app.config.api_key.clone();

    // Get the value of the "Authorization" header from the request
    if let Some(auth_value) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(header) = header::HeaderValue::from_str(&app.config.api_key) {
            if auth_value == header {
                return Ok(next.run(req).await);
            }
        }
    }

    // API key is invalid, return an error
    let json_error = ErrorResponse {
        status: "fail",
        message: "Invalid API key".to_string(),
    };
    Err((StatusCode::UNAUTHORIZED, Json(json_error)))
}

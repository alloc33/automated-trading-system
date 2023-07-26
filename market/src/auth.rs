use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

use crate::{error::ApiError, App};

pub async fn auth<B>(
    State(app): State<Arc<App>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(auth_value) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(header) = header::HeaderValue::from_str(&app.config.api_key) {
            if auth_value == header {
                return Ok(next.run(req).await);
            }
        }
    }

    Err(ApiError::Unauthorized(
        "API key isn't correct or not found".to_string(),
    ))
}

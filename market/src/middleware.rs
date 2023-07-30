use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::{error::ApiError, App};

pub async fn auth<B>(
    State(app): State<Arc<App>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    // NOTE: skip auth for post /alert endpoint. We don't need to check auth for tradingview
    // webhook as we use it's own secret key
    if req.uri().path() == "/alert" && req.method() == axum::http::Method::POST {
        return Ok(next.run(req).await);
    }

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

pub async fn print_request_body(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, Response> {
    if request.method() == axum::http::Method::GET {
        return Ok(next.run(request).await);
    }

    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request<Body>) -> Result<Request<Body>, Response> {
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    let json = serde_json::from_slice::<serde_json::Value>(&bytes)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    tracing::debug!("\n\n{json:#?}\n");

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

// NOTE: Trying to find a way to change error response body to a specific if it's not satisfy json
// format: 
// {
//    "error": ...
// }

// pub async fn handle_error<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
//     let resp = next.run(req).await;
//     match resp.status() {
//         StatusCode::UNPROCESSABLE_ENTITY => {
//             let body = Json(serde_json::json!({
//                 "error": "it works"
//             }));

//             Err((StatusCode::UNPROCESSABLE_ENTITY, body).into_response())
//         }
//         _ => Ok(resp),
//     }
// }

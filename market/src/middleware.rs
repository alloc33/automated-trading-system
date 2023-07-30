use std::{io::Write, sync::Arc};

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct JsonResponse {
    message: String,
}

pub async fn log_request(
    mut request: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, Response> {
    let (parts, body) = request.into_parts();
    let bytes = body_to_bytes(body).await?;
    let json = serde_json::from_slice::<serde_json::Value>(&bytes)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    let pretty_json = match serde_json::from_slice::<serde_json::Value>(&bytes) {
        Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_default(),
        Err(_) => String::from_utf8_lossy(&bytes).into_owned(),
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Log separator for request
    let separator = "\n\n-----------------------request-----------------------\n";
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    writeln!(&mut stdout, "{}", separator).unwrap();

    // Log method and URI
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .unwrap();
    writeln!(&mut stdout, "{} {}", parts.method, parts.uri).unwrap();

    // Log headers
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        .unwrap();
    writeln!(&mut stdout, "{:#?}", parts.headers).unwrap();

    // Log JSON body
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
    writeln!(&mut stdout, "{}", pretty_json).unwrap();

    stdout.reset().unwrap();

    request = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(request).await)
}

pub async fn log_response(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, Response> {
    let method = request.method();
    let response = next.run(request).await;
    let status = response.status();

    let (parts, body) = response.into_parts();
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    let pretty_json = match method {
        &axum::http::Method::GET => "",
        _ => match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_default(),
            Err(_) => String::from_utf8_lossy(&bytes).into_owned(),
        },
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Log separator for response
    let separator = "\n\n-----------------------response-----------------------\n";
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    writeln!(&mut stdout, "{}", separator).unwrap();

    match status.as_u16() {
        200..=299 => {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            writeln!(&mut stdout, "response: {}", status).unwrap();
        }
        _ => {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            writeln!(&mut stdout, "response: {}", status).unwrap();
        }
    }

    stdout.reset().unwrap();

    // Log headers
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        .unwrap();
    writeln!(&mut stdout, "{:#?}", parts.headers).unwrap();

    // Log JSON body
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
    writeln!(&mut stdout, "{}", pretty_json).unwrap();

    stdout.reset().unwrap();

    Ok(Response::from_parts(parts, Body::from(bytes)))
}

async fn body_to_bytes(body: Body) -> Result<Vec<u8>, Response> {
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    Ok(bytes.to_vec())
}

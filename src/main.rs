use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::url::Url;

mod dynamo;
mod url;

struct AppState {
    client: Client,
}

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let app_state = Arc::new(AppState { client });

    let app = Router::new()
        .route("/search/:short_url", get(handle_get))
        .route("/create", post(handle_post))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_get(
    State(state): State<Arc<AppState>>,
    Path(short_url): Path<String>,
) -> impl IntoResponse {
    match dynamo::query_item(&short_url, &state.client).await {
        Ok(data) => (
            StatusCode::OK,
            Json(json!({"url": data.url.to_owned(), "short_url": data.short_url.to_owned()})),
        ),
        Err(_e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "error getting url"})),
        ),
    }

    // return Err(AppError(anyhow!("some error")));

    // return Ok(Json(url));
}

#[derive(Debug, Deserialize)]
struct CreatePost {
    url: String,
    short_url: Option<String>,
}

async fn handle_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePost>,
) -> Result<(), AppError> {
    if payload.short_url.is_some() {
        if let Ok(_) = dynamo::query_item(
            &payload.short_url.clone().unwrap().to_owned(),
            &state.client,
        )
        .await
        {
            return Err(AppError(anyhow::anyhow!("short url already in use")));
        }
    }

    let url = Url {
        url: payload.url,
        short_url: payload.short_url.unwrap_or(String::from("some generator")),
    };

    dynamo::store_item(&url, &state.client).await?;

    return Ok(());
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

use crate::models::{Data, Response};
use axum::{http::StatusCode, response::IntoResponse, routing, Router};
use tracing::info;

pub fn health_router() -> Router {
    Router::new().route("/", routing::get(check_health))
}

async fn check_health() -> impl IntoResponse {
    info!("Health check.");
    Response::create(StatusCode::OK, "Up and running", Data::None)
}

use axum::{routing,  Router, response::IntoResponse, http::StatusCode};
use tracing::info;
use crate::models::{Data, Response};


pub fn health_router() -> Router {
    Router::new()
        .route("/", routing::get(check_health))
}

async fn check_health() -> impl IntoResponse {
    info!("Health check.");
    Response::create(StatusCode::OK, "Up and running", Data::None)
}

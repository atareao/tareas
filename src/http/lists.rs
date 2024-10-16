use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};

use axum::{
    extract::{
        State,
        Query,
        Extension
    },
    http::StatusCode,
    Router,
    routing,
    response::IntoResponse,
    Json,
    middleware::from_fn_with_state
};

use crate::models::User;

use super::super::{
    models::{
        List,
        AppState,
    },
    http::jwt_auth::auth
};
use tracing::{error, debug, info};

#[derive(Deserialize)]
struct Params {
    id: i64,
}


pub fn router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/lists",
            routing::get(read)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists",
            routing::post(create_or_update)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists",
            routing::delete(delete)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
}

pub async fn read(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    List::read_all(&app_state.pool, user.id)
        .await
        .map(|podcasts| Json(json!({
            "result": "ok",
            "content": podcasts
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),
        })))
}

async fn create_or_update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(podcast): Json<List>,
) -> impl IntoResponse{
    List::create(pool, name, user_id)
    Podcast::create_or_update(&app_state.pool, &podcast.name, &podcast.url, podcast.active)
        .await
        .map(|podcasts| (StatusCode::OK, Json(json!({
            "result": "ok",
            "content": podcasts
        }))))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "result": "ko",
            "content": e.to_string(),
        }))))
}
async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<Params>,
) -> impl IntoResponse{
    debug!("podcast: {}", params.id);
    List::delete(&app_state.pool, params.id)
        .await
        .map(|podcasts| Json(json!({
            "result": "ok",
            "content": podcasts
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),

        })))
}

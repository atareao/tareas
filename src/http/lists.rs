use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};

use axum::{
    extract::{
        State,
        Path,
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
        SimpleList,
        List,
        AppState,
        Task,
        SimpleTask,
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
            routing::get(read_all)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists/:id",
            routing::get(read_one)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists",
            routing::post(create_list)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists",
            routing::put(update_list)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists/:id",
            routing::delete(delete_list)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/lists/:id",
            routing::post(create_task)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
        .route("/tasks/:id",
            routing::delete(delete_task)
            .route_layer(from_fn_with_state(app_state.clone(), auth))
        )
}

pub async fn read_all(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    List::read_all(&app_state.pool, user.id)
        .await
        .map(|list| Json(json!({
            "result": "ok",
            "content": list
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),
        })))
}

async fn create_list(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(list): Json<SimpleList>,
) -> impl IntoResponse{
    List::create(&app_state.pool, &list.name, user.id)
        .await
        .map(|list| (StatusCode::OK, Json(json!({
            "result": "ok",
            "content": list
        }))))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "result": "ko",
            "content": e.to_string(),
        }))))
}

pub async fn read_one(
    Extension(_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Path(list_id): Path<i64>,
) -> impl IntoResponse{
    List::read_by_id(&app_state.pool, list_id)
        .await
        .map(|list| Json(json!({
            "result": "ok",
            "content": list
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),
        })))
}

async fn update_list(
    Extension(_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(list): Json<List>,
) -> impl IntoResponse{
    debug!("list: {:?}", list);
    list.update(&app_state.pool)
        .await
        .map(|list| Json(json!({
            "result": "ok",
            "content": list
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),
        })))
}
async fn delete_list(
    Extension(_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Path(list_id): Path<i64>,
) -> impl IntoResponse{
    debug!("list: {}", list_id);
    List::delete(&app_state.pool, list_id)
        .await
        .map(|list| Json(json!({
            "result": "ok",
            "content": list
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),

        })))
}

async fn create_task(
    Extension(_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Path(list_id): Path<i64>,
    Json(task): Json<SimpleTask>,
) -> impl IntoResponse{
    Task::create(&app_state.pool, &task.name, list_id)
        .await
        .map(|task| (StatusCode::OK, Json(json!({
            "result": "ok",
            "content": task
        }))))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "result": "ko",
            "content": e.to_string(),
        }))))
}

async fn delete_task(
    Extension(_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i64>,
) -> impl IntoResponse{
    debug!("task: {}", task_id);
    Task::delete(&app_state.pool, task_id)
        .await
        .map(|list| Json(json!({
            "result": "ok",
            "content": list
        })))
        .map_err(|e| Json(json!({
            "result": "ko",
            "content": e.to_string(),

        })))
}

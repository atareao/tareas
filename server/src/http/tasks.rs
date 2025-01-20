use crate::models::{AppState, Data, Task, OptionalId, Response};
use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use std::sync::Arc;
use tracing::debug;

pub fn task_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{list_id}",
            routing::post(create))
        .route("/{list_id}",
            routing::get(read))
        .route("/list_id",
            routing::put(update))
        .route("/{list_id}",
            routing::delete(delete))
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Path(list_id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    debug!("{:?}", body);
    let name = match body["name"].as_str() {
        Some(name) => name,
        None => return Response::create(StatusCode::BAD_REQUEST, "Name is required", Data::None),
    };
    let position = body["position"].as_i64().unwrap_or_default();
    match Task::create(&app_state.pool, name, position, list_id).await {
        Ok(task) => Response::create(
            StatusCode::CREATED, 
            "Created",
            Data::One(task.to_json())),
        Err(e) => Response::create(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().as_str(),
            Data::None,
        ),
    }
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(list_id): Path<i64>,
    Query(option_id): Query<OptionalId>,
) -> impl IntoResponse {
    debug!("{:?}", option_id);
    match option_id.id {
        Some(id) => Task::read(&app_state.pool, id)
            .await
            .map(|task| Response::create(StatusCode::OK, "Ok", Data::One(task.to_json())))
            .unwrap_or(Response::create(
                StatusCode::NOT_FOUND,
                "Not found",
                Data::None,
            )),
        None => Task::read_all(&app_state.pool, list_id)
            .await
            .map(|tasks| {
                let tasks = tasks
                    .into_iter()
                    .map(|task| task.to_json())
                    .collect::<Vec<serde_json::Value>>();
                debug!("{:?}", tasks);
                Response::create(StatusCode::OK, "Ok", Data::Some(tasks))
            })
            .unwrap_or(Response::create(
                StatusCode::NOT_FOUND,
                "Not found",
                Data::None,
            )),
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let id = match body["id"].as_i64() {
        Some(id) => id,
        None => return Response::create(StatusCode::BAD_REQUEST, "Id is required", Data::None),
    };
    let name = match body["name"].as_str() {
        Some(name) => name,
        None => return Response::create(StatusCode::BAD_REQUEST, "Name is required", Data::None),
    };
    let position = match body["position"].as_i64() {
        Some(position) => position,
        None => {
            return Response::create(StatusCode::BAD_REQUEST, "Position is required", Data::None)
        }
    };
    let done = match body["done"].as_bool() {
        Some(done) => done,
        None => {
            return Response::create(StatusCode::BAD_REQUEST, "Done is required", Data::None)
        }
    };
    match Task::update(&app_state.pool, id, name, position, done).await {
        Ok(task) => Response::create(StatusCode::OK, "Updated", Data::One(task.to_json())),
        Err(e) => Response::create(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().as_str(),
            Data::None,
        ),
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Query(id): Query<i64>,
) -> impl IntoResponse {
    match Task::delete(&app_state.pool, id).await {
        Ok(task) => Response::create(StatusCode::OK, "Deleted", Data::One(task.to_json())),
        Err(e) => {
            let message = format!("Can not delete task. {e}");
            Response::create(StatusCode::INTERNAL_SERVER_ERROR, &message, Data::None)
        }
    }
}

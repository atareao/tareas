use crate::models::{AppState, Data, List, Response, OptionalId};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use std::sync::Arc;
use tracing::debug;

pub fn list_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::get(read))
        .route("/", routing::put(update))
        .route("/", routing::delete(delete))
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    match body["name"].as_str() {
        Some(name) => List::create(&app_state.pool, name)
            .await
            .map(|list| Response::create(StatusCode::CREATED, "Created", Data::One(list.to_json())))
            .unwrap_or(Response::create(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can not create list",
                Data::None,
            )),
        None => Response::create(StatusCode::BAD_REQUEST, "Name is required", Data::None),
    }
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Query(option_id): Query<OptionalId>,
) -> impl IntoResponse {
    debug!("{:?}", option_id);
    match option_id.id {
        Some(id) => List::read(&app_state.pool, id)
            .await
            .map(|list| Response::create(StatusCode::OK, "Ok", Data::One(list.to_json())))
            .unwrap_or(Response::create(
                StatusCode::NOT_FOUND,
                "Not found",
                Data::None,
            )),
        None => List::read_all(&app_state.pool)
            .await
            .map(|lists| {
                let lists = lists
                    .into_iter()
                    .map(|list| list.to_json())
                    .collect::<Vec<serde_json::Value>>();
                debug!("{:?}", lists);
                Response::create(StatusCode::OK, "Ok", Data::Some(lists))
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
    match List::update(&app_state.pool, id, name).await {
            Ok(list) => Response::create(StatusCode::OK, "Updated", Data::One(list.to_json())),
            Err(e) => Response::create(StatusCode::INTERNAL_SERVER_ERROR, e.to_string().as_str(), Data::None)
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Query(id): Query<i64>,
) -> impl IntoResponse {
    List::delete(&app_state.pool, id)
        .await
        .map(|list| Response::create(StatusCode::OK, "Deleted", Data::One(list.to_json())))
        .unwrap_or(Response::create(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can not delete list",
            Data::None,
        ))
}

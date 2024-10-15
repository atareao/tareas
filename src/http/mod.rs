pub mod jwt_auth;
pub mod estatic;
pub mod user;
pub mod root;

use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use axum::{
    Router,
    http::{
        header::{
            ACCEPT,
            AUTHORIZATION,
            CONTENT_TYPE
        },
        HeaderValue,
        Method,
    },
};
use minijinja::{Environment, path_loader};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use once_cell::sync::Lazy;
use std::env;

use crate::models::{
    AppState,
    Error
};

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

pub async fn serve(pool: &SqlitePool) -> Result<(), Error>{

    let url = env::var("URL")
        .unwrap_or("".to_string());
    let port = env::var("PORT")
        .unwrap_or("".to_string());
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or("a-secret-very-secret".to_string());
    let jwt_expires_in = env::var("60m")
        .unwrap_or("a-secret-very-secret".to_string());
    let jwt_max_age = env::var("JWT_MAXAGE")
        .unwrap_or("60".to_string()).parse().unwrap_or(60);

    let cors = CorsLayer::new()
        .allow_origin(url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let app = api_router(
            AppState::new(
               pool.clone(),
                jwt_secret,
                jwt_expires_in,
                jwt_max_age
            ))
            .layer(TraceLayer::new_for_http())
            .layer(cors);

    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(address)
        .await?;
    Ok(axum::serve(listener, app)
        .await?)
}

fn api_router(app_state: AppState) -> Router {
    estatic::router()
        .merge(root::router(Arc::new(app_state.clone())))
        .merge(user::router())
        .with_state(Arc::new(app_state.clone()))
}

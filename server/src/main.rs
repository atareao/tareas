mod models;
mod http;

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
};

use axum::{debug_handler, routing::get, Router};
use std::sync::Arc;
use std::{env::var, path::Path, str::FromStr};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::{
    http::{
        list_router,
        health_router,
    },
    models::{
        Error,
        AppState,
    }
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_level: String = var("RUST_LOG").unwrap_or("debug".to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = var("DB_URL").unwrap_or("tareas.db".to_string());
    info!("DB url: {}", db_url);

    // Create database
    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap() {
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    // Migrations
    let migrations = if var("RUST_ENV") == Ok("production".to_string()) {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("migrations")
    } else {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join("migrations")
    };
    debug!("{}", &migrations.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Pool failed");

    Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    let api_routes = Router::new()
        .nest("/lists", list_router())
        .with_state(Arc::new(AppState { pool }))
        .nest("/health", health_router());

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .fallback_service(ServeDir::new("static").fallback(ServeFile::new("static/index.html")));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Server started successfully");
    axum::serve(listener, app).await?;
    Ok(())
}

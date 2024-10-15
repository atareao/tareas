mod http;
mod models;

use sqlx::{
    sqlite::SqlitePoolOptions,
    migrate::{
        Migrator,
        MigrateDatabase
    },
};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use std::{
    str::FromStr,
    env::var,
    path::Path,
};
use tracing::info;


#[tokio::main]
async fn main(){
    let log_level = var("RUST_LOG").unwrap_or("debug".to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Log level: {log_level}");

    let db_url = var("DB_URL").unwrap_or("podmixer.db".to_string());
    info!("DB url: {}", db_url);

    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap(){
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let migrations = if var("RUST_ENV") == Ok("production".to_string()){
        std::env::current_exe().unwrap().parent().unwrap().join("migrations")
    }else{
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join("migrations")
    };
    info!("{}", &migrations.display());

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


    tracing::info!("🚀 Server started successfully");
    http::serve(&pool)
        .await
        .unwrap();
}

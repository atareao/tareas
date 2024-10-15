use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: u64,
}

impl AppState{
    pub fn new(pool: SqlitePool, jwt_secret: String, jwt_expires_in: String, jwt_max_age: u64) -> Self{
        Self{
            pool,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
        }
    }
}


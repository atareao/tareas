use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row, Error};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
    pub position: i64,
    pub active: bool,
    pub admin: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            username: row.get("username"),
            hashed_password: row.get("hashed_password"),
            position: row.get("position"),
            active: row.get("active"),
            admin: row.get("admin"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    // CRUD

    pub async fn create(pool: &SqlitePool, username: &str, hashed_password: &str) -> Result<Self, Error>{
        let sql = "INSERT INTO users (username, hashed_password, position, active, admin, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(username)
            .bind(hashed_password)
            .bind(0)
            .bind(true)
            .bind(false)
            .bind(now)
            .bind(now)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_name(pool: &SqlitePool, username: &str) -> Result<Self, Error>{
        let sql = "SELECT * FROM users WHERE username = $1";
        query(sql)
            .bind(username)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_id(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM users";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(&self, pool: &SqlitePool) -> Result<Self, Error>{
        let sql = "UPDATE users SET hashed_password = $1, position = $2, active = $3, admon = $4, updated_at = $5 WHERE id = $6 RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(self.hashed_password.clone())
            .bind(self.position)
            .bind(self.active)
            .bind(self.admin)
            .bind(now)
            .bind(self.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<Self, Error>{
        let sql = "DELETE FROM users WHERE id = $1 RETURNING *";
        query(sql)
            .bind(self.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

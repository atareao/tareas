use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row, Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub active: bool,
    pub user_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl List {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            position: row.get("position"),
            active: row.get("active"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn create(pool: &SqlitePool, name: &str, user_id: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO lists (name, position, active, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(name)
            .bind(0)
            .bind(true)
            .bind(user_id)
            .bind(now)
            .bind(now)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_name(pool: &SqlitePool, name: &str) -> Result<Self, Error>{
        let sql = "SELECT * FROM lists WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_id(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "SELECT * FROM lists WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM lists";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(&self, pool: &SqlitePool) -> Result<Self, Error>{
        let sql = "UPDATE lists SET position = $1, active = $2, user_id =$3, updated_at = $4 WHERE id = $5 RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(self.position)
            .bind(self.active)
            .bind(self.user_id)
            .bind(now)
            .bind(self.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<Self, Error>{
        let sql = "DELETE FROM lists WHERE id = $1 RETURNING *";
        query(sql)
            .bind(self.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

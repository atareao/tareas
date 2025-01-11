use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteRow, SqlitePool}, Row, error::Error, query};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub list_id: i64,
    pub name: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            list_id: row.get("list_id"),
            name: row.get("name"),
            done: row.get("done"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn to_json(&self) -> Value{
        serde_json::json!({
            "id": self.id,
            "list_id": self.list_id,
            "name": self.name,
            "done": self.done,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
        })
    }

    pub async fn create(pool: &SqlitePool, name: &str, list_id: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO tasks (list_id, name, done, created_at, updated_at) VALUES ($1, $2, $3, $4, $4) RETURNING *";
        query(sql)
            .bind(list_id)
            .bind(name)
            .bind(false)
            .bind(Utc::now())
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i32) -> Result<Self, Error>{
        let sql = "SELECT * FROM tasks WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool, list_id: i64) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks WHERE list_id = $1";
        query(sql)
            .bind(list_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, id: i32, name: &str, done: bool) -> Result<Self, Error>{
        let sql = "UPDATE tasks SET name = $1, done = $2, updated_at = $3 WHERE id = $4 RETURNING *";
        query(sql)
            .bind(name)
            .bind(done)
            .bind(Utc::now())
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i32) -> Result<Self, Error>{
        let sql = "DELETE FROM tasks WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn search(pool: &SqlitePool, list_id: i64, name: &str) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks WHERE list_id = $1 AND LOWER(name) LIKE $2";
        query(sql)
            .bind(list_id)
            .bind(format!("%{}%", name.to_lowercase()))
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }
}

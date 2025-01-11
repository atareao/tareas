use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteRow, SqlitePool}, Row, error::Error, query};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskTag {
    pub id: i32,
    pub task_id: i32,
    pub tag_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TaskTag {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            task_id: row.get("task_id"),
            tag_id: row.get("task_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn to_json(&self) -> Value{
        serde_json::json!({
            "id": self.id,
            "task_id": self.task_id,
            "tag_id": self.tag_id,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
        })
    }

    pub async fn create(pool: &SqlitePool, task_id: i64, tag_id: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO tasks_tags (task_id, tag_id, created_at, updated_at) VALUES ($1, $2, $3, $3) RETURNING *";
        query(sql)
            .bind(task_id)
            .bind(tag_id)
            .bind(Utc::now())
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i32) -> Result<Self, Error>{
        let sql = "SELECT * FROM tasks_tags WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool, list_id: i64) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks_tags WHERE list_id = $1";
        query(sql)
            .bind(list_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, id: i32, name: &str, done: bool) -> Result<Self, Error>{
        let sql = "UPDATE tasks_tags SET name = $1, done = $2, updated_at = $3 WHERE id = $4 RETURNING *";
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
        let sql = "DELETE FROM tasks_tags WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn search(pool: &SqlitePool, list_id: i64, name: &str) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks_tags WHERE list_id = $1 AND LOWER(name) LIKE $2";
        query(sql)
            .bind(list_id)
            .bind(format!("%{}%", name.to_lowercase()))
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }
}

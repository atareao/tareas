use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row, Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub completed: bool,
    pub list_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Task {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            position: row.get("position"),
            completed: row.get("completed"),
            list_id: row.get("list_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn create(pool: &SqlitePool, name: &str, list_id: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO tasks (name, position, completed, list_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(name)
            .bind(0)
            .bind(false)
            .bind(list_id)
            .bind(now)
            .bind(now)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_name(pool: &SqlitePool, name: &str) -> Result<Self, Error>{
        let sql = "SELECT * FROM tasks WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_id(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "SELECT * FROM tasks WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(&self, pool: &SqlitePool) -> Result<Self, Error>{
        let sql = "UPDATE tasks SET position = $1, completed = $2, list_id =$3, updated_at = $4 WHERE id = $5 RETURNING *";
        let now = Utc::now();
        query(sql)
            .bind(self.position)
            .bind(self.completed)
            .bind(self.list_id)
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


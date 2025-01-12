use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteRow, SqlitePool}, Row, error::Error, query};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct List {
    pub id: i64,
    pub name: String,
    pub position : i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


impl List {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn to_json(&self) -> Value{
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "position": self.position,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
        })
    }

    pub async fn create(pool: &SqlitePool, name: &str, position: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO lists (name, position, created_at, updated_at) VALUES ($1, $2, $3, $3) RETURNING *";
        query(sql)
            .bind(name)
            .bind(position)
            .bind(Utc::now())
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "SELECT * FROM lists WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM lists ORDER BY position";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, id: i64, name: &str, position: i64) -> Result<Self, Error>{
        let sql = "UPDATE lists SET name = $1, position = $2, updated_at = $3 WHERE id = $4 RETURNING *";
        query(sql)
            .bind(name)
            .bind(position)
            .bind(Utc::now())
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "DELETE FROM lists WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn search(pool: &SqlitePool, name: &str) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM lists WHERE LOWER(name) LIKE $1 ORDER BY position";
        query(sql)
            .bind(format!("%{}%", name.to_lowercase()))
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

}

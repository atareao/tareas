use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteRow, SqlitePool}, Row, error::Error, query};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub list_id: i64,
    pub name: String,
    pub position: i64,
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
            position: row.get("position"),
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
            "position": self.position,
            "done": self.done,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
        })
    }

    pub async fn create(pool: &SqlitePool, name: &str, position: i64, list_id: i64) -> Result<Self, Error>{
        let sql = "INSERT INTO tasks (list_id, name, done, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *";
        query(sql)
            .bind(list_id)
            .bind(name)
            .bind(position)
            .bind(false)
            .bind(Utc::now())
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "SELECT * FROM tasks WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool, list_id: i64) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks WHERE list_id = $1 ORDER BY position";
        query(sql)
            .bind(list_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn read_all_filtered(pool: &SqlitePool, list_id: i64, tag_ids: &Vec<i64>) -> Result<Vec<Self>, Error>{
        let ids = (2..tag_ids.len() + 1)
            .collect::<Vec<usize>>()
            .iter()
            .map(|number| format!("${}", number))
            .collect::<Vec<String>>()
            .join(",");
        let sql_string = format!("SELECT * FROM tasks
                         INNER JOIN tasks_tags ON tasks.id = tasks_tags.task_id
                         WHERE list_id = $1 AND tasks_tags.tag_id IN ({}) ORDER BY position", &ids);
        let mut sql = query(&sql_string);
        sql = sql.bind(list_id);
        for tag_id in tag_ids{
            sql = sql.bind(tag_id);
        }
        sql.map(Self::from_row)
           .fetch_all(pool)
           .await
    }

    pub async fn update(pool: &SqlitePool, id: i64, name: &str, position: i64, done: bool) -> Result<Self, Error>{
        let sql = "UPDATE tasks SET name = $1, position =$2, done = $3, updated_at = $4 WHERE id = $5 RETURNING *";
        query(sql)
            .bind(name)
            .bind(position)
            .bind(done)
            .bind(Utc::now())
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        let sql = "DELETE FROM tasks WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn search(pool: &SqlitePool, list_id: i64, name: &str) -> Result<Vec<Self>, Error>{
        let sql = "SELECT * FROM tasks WHERE list_id = $1 AND LOWER(name) LIKE $2 ORDER BY position";
        query(sql)
            .bind(list_id)
            .bind(format!("%{}%", name.to_lowercase()))
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn search_filtered(pool: &SqlitePool, list_id: i64, name: &str, tag_ids: &Vec<i64>) -> Result<Vec<Self>, Error>{
        let ids = (3..tag_ids.len() + 2)
            .collect::<Vec<usize>>()
            .iter()
            .map(|number| format!("${}", number))
            .collect::<Vec<String>>()
            .join(",");
        let sql_string = format!("SELECT * FROM tasks
                         INNER JOIN tasks_tags ON tasks.id = tasks_tags.task_id
                         WHERE list_id = 1$ AND LOWER(name) LIKE  $2 AND tasks_tags.tag_id IN ({}) ORDER BY position", &ids);
        let mut sql = query(&sql_string);
        sql = sql
            .bind(list_id)
            .bind(format!("%{}%", name.to_lowercase()));
        for tag_id in tag_ids{
            sql = sql.bind(tag_id);
        }
        sql.map(Self::from_row)
           .fetch_all(pool)
           .await
    }

}

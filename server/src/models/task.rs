use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    error::Error,
    query,
    sqlite::{SqlitePool, SqliteRow},
    QueryBuilder, Row,
};
use tracing::debug;

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
    fn from_row(row: SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            list_id: row.get("list_id"),
            name: row.get("name"),
            position: row.get("position"),
            done: row.get("done"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn to_json(&self) -> Value {
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

    pub async fn create(
        pool: &SqlitePool,
        name: &str,
        position: i64,
        list_id: i64,
    ) -> Result<Self, Error> {
        debug!("create task name: {name}, position: {position}, list_id: {list_id}");
        let sql = "INSERT INTO tasks (list_id, name, position, done, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *";
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

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error> {
        let sql = "SELECT * FROM tasks WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool, list_id: i64) -> Result<Vec<Self>, Error> {
        let sql = "SELECT * FROM tasks WHERE list_id = $1 ORDER BY position";
        query(sql)
            .bind(list_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    /// Lectura filtrada por etiquetas.
    ///
    /// Conservado como parte del modelo previsto: el filtrado todavía no está
    /// expuesto en la API HTTP.
    #[allow(dead_code)]
    pub async fn read_all_filtered(
        pool: &SqlitePool,
        list_id: i64,
        tag_ids: &Vec<i64>,
    ) -> Result<Vec<Self>, Error> {
        // Sin etiquetas no hay intersección posible y generar `IN ()` produce
        // SQL inválido: se devuelve vacío sin llegar a consultar.
        if tag_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Los placeholders de la lista `IN` se generan con `QueryBuilder`, que
        // los emite como parámetros ligados ($2, $3, ...): nunca se interpola
        // entrada del usuario en el SQL.
        let mut qb = QueryBuilder::new(
            "SELECT * FROM tasks
                         INNER JOIN tasks_tags ON tasks.id = tasks_tags.task_id
                         WHERE list_id = ",
        );
        qb.push_bind(list_id);
        qb.push(" AND tasks_tags.tag_id IN (");
        let mut separated = qb.separated(", ");
        for tag_id in tag_ids {
            separated.push_bind(*tag_id);
        }
        separated.push_unseparated(") ORDER BY position");
        qb.build().map(Self::from_row).fetch_all(pool).await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        name: &str,
        position: i64,
        done: bool,
    ) -> Result<Self, Error> {
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

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Self, Error> {
        let sql = "DELETE FROM tasks WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    /// Búsqueda por nombre (case-insensitive).
    ///
    /// Conservado como parte del modelo previsto: la búsqueda todavía no está
    /// expuesta en la API HTTP.
    #[allow(dead_code)]
    pub async fn search(pool: &SqlitePool, list_id: i64, name: &str) -> Result<Vec<Self>, Error> {
        let sql =
            "SELECT * FROM tasks WHERE list_id = $1 AND LOWER(name) LIKE $2 ORDER BY position";
        query(sql)
            .bind(list_id)
            .bind(format!("%{}%", name.to_lowercase()))
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    /// Búsqueda filtrada por etiquetas.
    ///
    /// Conservado como parte del modelo previsto: el filtrado todavía no está
    /// expuesto en la API HTTP.
    #[allow(dead_code)]
    pub async fn search_filtered(
        pool: &SqlitePool,
        list_id: i64,
        name: &str,
        tag_ids: &Vec<i64>,
    ) -> Result<Vec<Self>, Error> {
        // Igual que en `read_all_filtered`: sin etiquetas se evita el `IN ()`
        // inválido devolviendo vacío.
        if tag_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Placeholders ligados vía `QueryBuilder`.
        let mut qb = QueryBuilder::new(
            "SELECT * FROM tasks
                         INNER JOIN tasks_tags ON tasks.id = tasks_tags.task_id
                         WHERE list_id = ",
        );
        qb.push_bind(list_id);
        qb.push(" AND LOWER(name) LIKE ");
        qb.push_bind(format!("%{}%", name.to_lowercase()));
        qb.push(" AND tasks_tags.tag_id IN (");
        let mut separated = qb.separated(", ");
        for tag_id in tag_ids {
            separated.push_bind(*tag_id);
        }
        separated.push_unseparated(") ORDER BY position");
        qb.build().map(Self::from_row).fetch_all(pool).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{tag::Tag, task_tag::TaskTag, test_utils::setup_pool, List};

    /// `Task::create` + `Task::read_all` devuelven lo insertado.
    #[tokio::test]
    async fn create_and_read_all_roundtrip() {
        let pool = setup_pool().await;
        let list = List::create(&pool, "inbox", 0).await.expect("list");

        let first = Task::create(&pool, "primera", 0, list.id)
            .await
            .expect("create first");
        let second = Task::create(&pool, "segunda", 1, list.id)
            .await
            .expect("create second");

        let all = Task::read_all(&pool, list.id).await.expect("read_all");
        let ids: Vec<i64> = all.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![first.id, second.id]);
        assert_eq!(all[0].name, "primera");
        assert_eq!(all[0].list_id, list.id);
        assert!(!all[0].done);

        let one = Task::read(&pool, second.id).await.expect("read");
        assert_eq!(one.name, "segunda");
    }

    /// `search_filtered` con **dos** etiquetas genera `IN (?, ?)` válido y
    /// devuelve las tareas etiquetadas (regresión de la generación de
    /// placeholders con `QueryBuilder`).
    #[tokio::test]
    async fn search_filtered_with_two_tags() {
        let pool = setup_pool().await;
        let list = List::create(&pool, "inbox", 0).await.expect("list");
        let trabajo = Tag::create(&pool, "trabajo", 0).await.expect("tag trabajo");
        let casa = Tag::create(&pool, "casa", 1).await.expect("tag casa");

        let a = Task::create(&pool, "comprar pan", 0, list.id)
            .await
            .expect("task a");
        let b = Task::create(&pool, "comprar leche", 1, list.id)
            .await
            .expect("task b");
        // Tarea sin etiquetas: no debe aparecer en el filtrado.
        let c = Task::create(&pool, "comprar tornillos", 2, list.id)
            .await
            .expect("task c");

        TaskTag::create(&pool, a.id, trabajo.id)
            .await
            .expect("a/trabajo");
        TaskTag::create(&pool, b.id, casa.id).await.expect("b/casa");

        let found = Task::search_filtered(&pool, list.id, "comprar", &vec![trabajo.id, casa.id])
            .await
            .expect("search_filtered");
        let ids: Vec<i64> = found.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![a.id, b.id]);
        assert!(!ids.contains(&c.id));
    }

    /// `search_filtered` sin etiquetas no debe generar `IN ()` (SQL inválido):
    /// devuelve vacío sin consultar.
    #[tokio::test]
    async fn search_filtered_with_empty_tags_returns_empty() {
        let pool = setup_pool().await;
        let list = List::create(&pool, "inbox", 0).await.expect("list");
        let _ = Task::create(&pool, "comprar pan", 0, list.id)
            .await
            .expect("task");

        let found = Task::search_filtered(&pool, list.id, "comprar", &vec![])
            .await
            .expect("search_filtered");
        assert!(found.is_empty());
    }
}

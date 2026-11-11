mod api;
mod error;
mod list;
mod state;
mod tag;
mod task;
mod task_tag;

pub use api::{Data, OptionalId, Response};
pub use error::Error;
pub use list::List;
pub use state::AppState;
pub use task::Task;
// `Tag` y `TaskTag` se conservan en sus módulos (ver `#[allow(dead_code)]`)
// pero no se re-exportan todavía: la feature de etiquetado no está expuesta
// en la API HTTP, así que ningún consumidor los referencia aún.

/// Utilidades compartidas por los tests de los modelos.
#[cfg(test)]
pub(crate) mod test_utils {
    use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};
    use std::path::Path;

    /// Crea un pool SQLite en memoria con las migraciones ya aplicadas.
    ///
    /// Se limita a una única conexión porque en SQLite cada conexión a
    /// `sqlite::memory:` abre una base de datos independiente: con más de una
    /// conexión los tests no verían el esquema ni los datos insertados.
    pub async fn setup_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("no se pudo crear el pool SQLite en memoria");

        let migrations = Migrator::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations"))
            .await
            .expect("no se pudieron cargar las migraciones");

        migrations
            .run(&pool)
            .await
            .expect("no se pudieron aplicar las migraciones");

        pool
    }
}

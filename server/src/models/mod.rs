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

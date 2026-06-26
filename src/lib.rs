pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

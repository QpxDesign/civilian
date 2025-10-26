use crate::structs::Incident::Incident;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub out: Arc<Mutex<Vec<Incident>>>,
}

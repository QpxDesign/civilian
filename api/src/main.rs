use axum::{response::Html, routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[path = "./structs/mod.rs"]
mod structs;

#[path = "./utils/mod.rs"]
mod utils;

#[path = "./routes/mod.rs"]
mod routes;

use crate::structs::AppState::AppState;

#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any); //
    let _ = dotenvy::dotenv();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::vars().find(|v| v.0 == "DATABASE_URL").unwrap().1)
        .await
        .unwrap();
    let state = AppState {
        pool,
        out: Arc::new(Mutex::new(Vec::new())),
    };
    let app: Router = Router::new()
        .route(
            "/",
            get(Html(
                format!(
                "<meta http-equiv=\"refresh\" content=\"0; url=https://civilian.lying.club/ \" />"
            )
                .to_string(),
            )),
        )
        .route(
            "/incidents",
            get(crate::routes::send_incidents::send_incidents),
        )
        .with_state(state)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1947").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::str::FromStr;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tuprfr::{adapters::inbound::http::routes::router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connect_opts = PgConnectOptions::from_str(&database_url)
        .expect("invalid DATABASE_URL")
        .ssl_mode(PgSslMode::Prefer);

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run migrations");

    let state = AppState { db };
    let session_layer = SessionManagerLayer::new(MemoryStore::default());
    let app = router(state).layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!(addr = %listener.local_addr().unwrap(), "listening");
    axum::serve(listener, app).await.unwrap();
}

use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::str::FromStr;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tuprfr::{adapters::inbound::http::routes::router, config::AppConfig, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load().expect("failed to load config");

    let ssl_mode = match config.database.ssl_mode.as_str() {
        "disable" => PgSslMode::Disable,
        "require" => PgSslMode::Require,
        _ => PgSslMode::Prefer,
    };

    let connect_opts = PgConnectOptions::from_str(&config.database.url)
        .expect("invalid database.url")
        .ssl_mode(ssl_mode);

    let db = PgPoolOptions::new()
        .max_connections(config.database.pool_max_connections)
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

    let listener = tokio::net::TcpListener::bind(&config.server.bind_address)
        .await
        .unwrap();
    tracing::info!(addr = %listener.local_addr().unwrap(), "listening");
    axum::serve(listener, app).await.unwrap();
}

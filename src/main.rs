mod config;
mod error;
mod models;
mod routes;
mod state;

use axum::{routing::{get, patch, post}, Router};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env()?;

    let connect_opts = PgConnectOptions::new()
        .host(&config.db_host)
        .port(config.db_port)
        .username(&config.db_user)
        .password(&config.db_password)
        .database(&config.db_name)
        .ssl_mode(PgSslMode::Require)
        .statement_cache_capacity(0);

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_opts)
        .await?;

    tracing::info!("Connected to database");

    let state = state::AppState { db, config: config.clone() };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(routes::health::handler))
        .route("/players", post(routes::players::create))
        .route("/players/:id", get(routes::players::get).patch(routes::players::update))
        .route("/players/:id/checkins", post(routes::checkins::create))
        .route("/players/:id/checkins/:date", get(routes::checkins::get_by_date))
        .route("/players/:id/ai/chat", post(routes::ai::chat))
        .with_state(state)
        .layer(cors);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("GameReady API running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

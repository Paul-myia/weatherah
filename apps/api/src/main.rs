mod handlers;
mod models;
mod services;
mod utils;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use handlers::{weather::*, websocket::websocket_handler};
use services::database::Database;
use std::{env, sync::Arc};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub weather_tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/weather".to_string());

    let db: Arc<Database> = Arc::new(Database::new(&database_url).await?);

    // Create broadcast channel for real-time weather updates
    let (weather_tx, _) = broadcast::channel(100);

    let state = AppState {
        db: db.clone(),
        weather_tx: weather_tx.clone(),
    };

    // Spawn background task for periodic weather updates
    let weather_service = services::weather_service::WeatherService::new(db.clone());
    tokio::spawn(async move {
        weather_service.start_periodic_updates(weather_tx).await;
    });

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/weather/current", get(get_current_weather))
        .route("/api/weather/forecast", post(get_weather_forecast))
        .route("/ws", get(websocket_handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
        .with_state(state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<u16>()?;

    info!("Starting server on port {}", port);
    info!("Using Open-Meteo API for weather data");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

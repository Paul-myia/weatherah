use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use tracing::error;

use crate::{
    models::weather::{ForecastRequest, WeatherRequest},
    services::{forecast_service::ForecastService, weather_service::WeatherService},
    AppState,
};

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "weather-backend" }))
}

pub async fn get_current_weather(
    Query(params): Query<WeatherRequest>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let weather_service = WeatherService::new(state.db.clone());

    match weather_service
        .get_current_weather(params.latitude, params.longitude)
        .await
    {
        Ok(weather) => Ok(Json(json!({ "current": weather }))),
        Err(e) => {
            error!("Failed to get current weather: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_weather_forecast(
    State(state): State<AppState>,
    Json(request): Json<ForecastRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let forecast_service = ForecastService::new();
    let weather_service = WeatherService::new(state.db.clone());

    // Get current weather first
    let current_weather = match weather_service
        .get_current_weather(request.latitude, request.longitude)
        .await
    {
        Ok(weather) => weather,
        Err(e) => {
            error!("Failed to get current weather for forecast: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Generate forecast using Chronos-T5
    match forecast_service
        .generate_forecast(&request.historical_data)
        .await
    {
        Ok(forecast) => Ok(Json(json!({
            "current": current_weather,
            "forecast": forecast
        }))),
        Err(e) => {
            error!("Failed to generate forecast: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct WeatherPoint {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub pressure: f64,
    pub description: String,
    pub icon: String,
    pub timestamp: DateTime<Utc>,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WeatherForecast {
    pub id: Uuid,
    pub weather_point_id: Uuid,
    pub forecasted_at: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub pressure: f64,
    pub description: String,
    pub icon: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub historical_data: Vec<HistoricalDataPoint>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct HistoricalDataPoint {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
}

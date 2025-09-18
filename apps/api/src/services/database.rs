use anyhow::Result;
use sqlx::PgPool;
use tracing::info;

use crate::models::weather::{WeatherForecast, WeatherPoint};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;

        // Run migrations if they exist
        // Note: You may need to run migrations manually or use sqlx migrate!
        // sqlx::migrate!("./migrations").run(&pool).await?;

        info!("Connected to database");
        Ok(Self { pool })
    }

    pub async fn insert_weather_point(&self, weather: &WeatherPoint) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO weather_points (id, latitude, longitude, temperature, humidity,
                                      wind_speed, wind_direction, pressure, description,
                                      icon, timestamp, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                temperature = EXCLUDED.temperature,
                humidity = EXCLUDED.humidity,
                wind_speed = EXCLUDED.wind_speed,
                wind_direction = EXCLUDED.wind_direction,
                pressure = EXCLUDED.pressure,
                description = EXCLUDED.description,
                icon = EXCLUDED.icon,
                timestamp = EXCLUDED.timestamp
            "#,
        )
        .bind(&weather.id)
        .bind(weather.latitude)
        .bind(weather.longitude)
        .bind(weather.temperature)
        .bind(weather.humidity)
        .bind(weather.wind_speed)
        .bind(weather.wind_direction)
        .bind(weather.pressure)
        .bind(&weather.description)
        .bind(&weather.icon)
        .bind(weather.timestamp)
        .bind(&weather.location)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_weather_by_location(
        &self,
        lat: f64,
        lon: f64,
    ) -> Result<Option<WeatherPoint>> {
        let weather = sqlx::query_as::<_, WeatherPoint>(
            r#"
            SELECT id, latitude, longitude, temperature, humidity, wind_speed,
                   wind_direction, pressure, description, icon, timestamp, location
            FROM weather_points
            WHERE ABS(latitude - $1) < 0.01 AND ABS(longitude - $2) < 0.01
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
        )
        .bind(lat)
        .bind(lon)
        .fetch_optional(&self.pool)
        .await?;

        Ok(weather)
    }

    pub async fn insert_forecast(&self, forecast: &WeatherForecast) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO weather_forecasts (id, weather_point_id, forecasted_at, temperature,
                                         humidity, wind_speed, pressure, description, icon, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(&forecast.id)
        .bind(&forecast.weather_point_id)
        .bind(forecast.forecasted_at)
        .bind(forecast.temperature)
        .bind(forecast.humidity)
        .bind(forecast.wind_speed)
        .bind(forecast.pressure)
        .bind(&forecast.description)
        .bind(&forecast.icon)
        .bind(forecast.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

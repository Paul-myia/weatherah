use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast, time::interval};
use tracing::{error, info};
use uuid::Uuid;

use crate::{models::weather::WeatherPoint, services::database::Database};

pub struct WeatherService {
    client: Client,
    db: Arc<Database>,
}

impl WeatherService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            client: Client::new(),
            db,
        }
    }

    pub async fn get_current_weather(&self, lat: f64, lon: f64) -> Result<WeatherPoint> {
        // First check database for recent data
        if let Ok(Some(weather)) = self.db.get_weather_by_location(lat, lon).await {
            let age = Utc::now().signed_duration_since(weather.timestamp);
            if age.num_minutes() < 30 {
                return Ok(weather);
            }
        }

        // Fetch from Open-Meteo API
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,wind_speed_10m,wind_direction_10m,surface_pressure,weather_code&timezone=auto",
            lat, lon
        );

        let response = self.client.get(&url).send().await?;
        let data: Value = response.json().await?;

        // Get location name from reverse geocoding (free service)
        let location = self
            .get_location_name(lat, lon)
            .await
            .unwrap_or_else(|_| format!("{:.2}, {:.2}", lat, lon));

        let current = &data["current"];
        let weather_code = current["weather_code"].as_u64().unwrap_or(0);
        let (description, icon) = self.get_weather_info(weather_code);

        let weather = WeatherPoint {
            id: Uuid::new_v4(),
            latitude: lat,
            longitude: lon,
            temperature: current["temperature_2m"].as_f64().unwrap_or(0.0),
            humidity: current["relative_humidity_2m"].as_f64().unwrap_or(0.0),
            wind_speed: current["wind_speed_10m"].as_f64().unwrap_or(0.0),
            wind_direction: current["wind_direction_10m"].as_f64().unwrap_or(0.0),
            pressure: current["surface_pressure"].as_f64().unwrap_or(1013.25),
            description,
            icon,
            timestamp: Utc::now(),
            location,
        };

        // Save to database
        if let Err(e) = self.db.insert_weather_point(&weather).await {
            error!("Failed to save weather data: {}", e);
        }

        Ok(weather)
    }

    async fn get_location_name(&self, lat: f64, lon: f64) -> Result<String> {
        // Use OpenStreetMap Nominatim for reverse geocoding
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json&addressdetails=1",
            lat, lon
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "WeatherApp/1.0")
            .send()
            .await?;

        let data: Value = response.json().await?;

        // Extract city, town, or village name
        let address = &data["address"];
        let location = address["city"]
            .as_str()
            .or_else(|| address["town"].as_str())
            .or_else(|| address["village"].as_str())
            .or_else(|| address["municipality"].as_str())
            .or_else(|| address["county"].as_str())
            .unwrap_or("Unknown Location")
            .to_string();

        Ok(location)
    }

    fn get_weather_info(&self, weather_code: u64) -> (String, String) {
        // WMO Weather interpretation codes to description and icon mapping
        match weather_code {
            0 => ("Clear sky".to_string(), "01d".to_string()),
            1 => ("Mainly clear".to_string(), "02d".to_string()),
            2 => ("Partly cloudy".to_string(), "03d".to_string()),
            3 => ("Overcast".to_string(), "04d".to_string()),
            45 | 48 => ("Foggy".to_string(), "50d".to_string()),
            51 | 53 | 55 => ("Drizzle".to_string(), "09d".to_string()),
            56 | 57 => ("Freezing drizzle".to_string(), "09d".to_string()),
            61 | 63 | 65 => ("Rain".to_string(), "10d".to_string()),
            66 | 67 => ("Freezing rain".to_string(), "13d".to_string()),
            71 | 73 | 75 => ("Snow".to_string(), "13d".to_string()),
            77 => ("Snow grains".to_string(), "13d".to_string()),
            80 | 81 | 82 => ("Rain showers".to_string(), "09d".to_string()),
            85 | 86 => ("Snow showers".to_string(), "13d".to_string()),
            95 => ("Thunderstorm".to_string(), "11d".to_string()),
            96 | 99 => ("Thunderstorm with hail".to_string(), "11d".to_string()),
            _ => ("Unknown".to_string(), "01d".to_string()),
        }
    }

    pub async fn start_periodic_updates(&self, tx: broadcast::Sender<String>) {
        let mut interval = interval(Duration::from_secs(60));

        // Default locations for periodic updates
        let locations = vec![
            (52.5200, 13.4050),  // Berlin
            (40.7128, -74.0060), // New York
            (51.5074, -0.1278),  // London
        ];

        loop {
            interval.tick().await;

            for (lat, lon) in &locations {
                match self.get_current_weather(*lat, *lon).await {
                    Ok(weather) => {
                        let message = serde_json::json!({
                            "type": "weather_update",
                            "data": weather,
                            "timestamp": Utc::now()
                        });

                        if let Err(e) = tx.send(message.to_string()) {
                            error!("Failed to broadcast weather update: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch weather for {}, {}: {}", lat, lon, e);
                    }
                }
            }

            info!(
                "Broadcasted weather updates for {} locations",
                locations.len()
            );
        }
    }
}

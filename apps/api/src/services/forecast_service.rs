use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use tracing::{error, info};

use crate::models::weather::HistoricalDataPoint;

pub struct ForecastService {
    client: Client,
    hf_api_key: String,
}

impl ForecastService {
    pub fn new() -> Self {
        let hf_api_key = env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY must be set");

        Self {
            client: Client::new(),
            hf_api_key,
        }
    }

    pub async fn generate_forecast(
        &self,
        historical_data: &[HistoricalDataPoint],
    ) -> Result<Vec<Value>> {
        if historical_data.is_empty() {
            return Ok(vec![]);
        }

        // Prepare time series data for Chronos-T5
        let temperatures: Vec<f64> = historical_data
            .iter()
            .map(|point| point.temperature)
            .collect();

        let payload = json!({
            "inputs": {
                "past_values": temperatures,
                "prediction_length": 24 // 24 hour forecast
            },
            "parameters": {
                "temperature": 0.7,
                "do_sample": true
            }
        });

        let response = self
            .client
            .post("https://api-inference.huggingface.co/models/amazon/chronos-t5-large")
            .header("Authorization", format!("Bearer {}", self.hf_api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Hugging Face API error: {}", error_text);
            return Err(anyhow::anyhow!("Forecast API error: {}", error_text));
        }

        let forecast_data: Value = response.json().await?;

        // Transform the forecast data into our format
        let predictions: Vec<Value> = forecast_data["prediction"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .enumerate()
            .map(|(i, temp)| {
                let forecast_time = Utc::now() + chrono::Duration::hours(i as i64 + 1);
                json!({
                    "timestamp": forecast_time,
                    "temperature": temp.as_f64().unwrap_or(20.0),
                    "humidity": 60.0 + (i as f64 * 2.0) % 40.0, // Mock data
                    "wind_speed": 5.0 + (i as f64 * 0.5) % 10.0, // Mock data
                    "pressure": 1013.25 + (i as f64 * 0.1) % 20.0, // Mock data
                    "description": "Forecasted",
                    "icon": "02d"
                })
            })
            .collect();

        info!("Generated forecast for {} hours", predictions.len());
        Ok(predictions)
    }
}

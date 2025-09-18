use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[allow(dead_code)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    WeatherApiError(reqwest::Error),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            AppError::WeatherApiError(e) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Weather API error: {}", e),
            ),
            AppError::ValidationError(e) => {
                (StatusCode::BAD_REQUEST, format!("Validation error: {}", e))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

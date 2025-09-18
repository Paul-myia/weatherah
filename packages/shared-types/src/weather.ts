export interface WeatherPoint {
  id: string;
  latitude: number;
  longitude: number;
  temperature: number;
  humidity: number;
  windSpeed: number;
  windDirection: number;
  pressure: number;
  description: string;
  icon: string;
  timestamp: string;
  location: string;
}

export interface WeatherForecast {
  id: string;
  weatherPointId: string;
  forecastedAt: string;
  temperature: number;
  humidity: number;
  windSpeed: number;
  pressure: number;
  description: string;
  icon: string;
  timestamp: string;
}

export interface GeolocationCoords {
  latitude: number;
  longitude: number;
  accuracy: number;
}

export interface WebSocketMessage {
  type: "weather_update" | "forecast_update" | "error" | "ping" | "pong";
  data?: WeatherPoint | WeatherForecast | string;
  timestamp: string;
}

export interface WeatherApiResponse {
  current: WeatherPoint;
  forecast: WeatherForecast[];
}

export interface ForecastRequest {
  latitude: number;
  longitude: number;
  historicalData: Array<{
    timestamp: string;
    temperature: number;
  }>;
}

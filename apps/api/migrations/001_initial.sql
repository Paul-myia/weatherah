CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE weather_points (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION NOT NULL,
    wind_speed DOUBLE PRECISION NOT NULL,
    wind_direction DOUBLE PRECISION NOT NULL,
    pressure DOUBLE PRECISION NOT NULL,
    description VARCHAR(255) NOT NULL,
    icon VARCHAR(10) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE weather_forecasts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    weather_point_id UUID NOT NULL REFERENCES weather_points(id) ON DELETE CASCADE,
    forecasted_at TIMESTAMPTZ NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION NOT NULL,
    wind_speed DOUBLE PRECISION NOT NULL,
    pressure DOUBLE PRECISION NOT NULL,
    description VARCHAR(255) NOT NULL,
    icon VARCHAR(10) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_weather_points_location ON weather_points(latitude, longitude);
CREATE INDEX idx_weather_points_timestamp ON weather_points(timestamp DESC);
CREATE INDEX idx_weather_forecasts_weather_point_id ON weather_forecasts(weather_point_id);
CREATE INDEX idx_weather_forecasts_timestamp ON weather_forecasts(timestamp);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for weather_points table
CREATE TRIGGER update_weather_points_updated_at BEFORE UPDATE
    ON weather_points FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

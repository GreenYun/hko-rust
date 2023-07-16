// Copyright (c) 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Rainfall in the past hour from automatic weather station
//!
//! This dataset provides rainfall amount measured at automatic weather station
//! during the 1-hour period ending at the observation time. Please note the
//! following in using this dataset:
//!
//! 1. The rainfall data in this dataset is originated from automatic weather
//!    stations. In particular, the source of rainfall data of automatic weather
//!    station “Hong Kong Observatory” in this dataset is different from the
//!    official record of Hong Kong Observatory rainfall data as given in the
//!    climatological database, other weather bulletins such as Current Weather
//!    Report, Yesterday’s Weather and Radiation Level, etc.
//! 2. The rainfall data in this dataset is provisional. Only limited data
//!    validation has been carried out. Users should take note of this
//!    limitation in using the data.
//!
//! - **HTTP Request Method**: GET
//! - **Return Type**: JSON

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::common::Lang;

/// Main response type.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Observation time
    pub obs_time: DateTime<FixedOffset>,

    /// Rainfall data
    pub hourly_rainfall: Vec<HourlyRainfall>,
}

/// Rainfall amount in the 1-hour period
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyRainfall {
    /// Name of automatic weather station
    pub automatic_weather_station: String,

    /// Automatic weather station ID for this dataset
    #[serde(rename = "automaticWeatherStationID")]
    pub automatic_weather_station_id: String,

    /// An integer value of the total rainfall amount during the 1-hour period
    /// measured by the automatic weather station ending at the observation
    /// time.
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_rainfall_value")]
    pub value: RainfallValue,

    /// Unit of the rainfall amount
    pub unit: String,
}

#[derive(Clone, Debug)]
pub enum RainfallValue {
    /// No rainfall
    UnderMaintenance,

    /// Rainfall amount in millimeter
    Rainfall(u32),
}

/// Generate API URL from specified date.
#[must_use]
pub fn url(lang: &Lang) -> String {
    format!("https://data.weather.gov.hk/weatherAPI/opendata/hourlyRainfall.php?lang={lang}")
}

#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(lang: &Lang) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(serde_json::from_str(&get(url(lang)).await?.text().await?)?)
}

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

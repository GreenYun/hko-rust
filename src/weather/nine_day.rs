// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! 9-day weather forecast of Hong Kong.

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::Deserialize;

use super::PSR;
use crate::{
    common::{PlaceValUnit, ValUnit},
    fetch::impl_api,
    weather::Name as WeatherName,
};

/// 9-day weather forecast of Hong Kong.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NineDay {
    /// General situation
    pub general_situation: String,

    /// A list of weather forecast
    pub weather_forecast: Vec<WeatherForcast>,

    /// Sea surface temperature
    pub sea_temp: SeaTemp,

    /// Soil temperature
    pub soil_temp: Vec<SoilTemp>,

    /// Update time
    pub update_time: DateTime<FixedOffset>,
}

/// The weather forecast on specified `date`.
///
/// The `week` field from the original response is ignored, since it can be
/// calculated from `date`.
#[derive(Clone, Debug, Deserialize)]
pub struct WeatherForcast {
    /// Forecast Date
    #[serde(rename = "forecastDate")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_yyyymmdd_to_date")]
    pub date: NaiveDate,

    /// Forecast Weather
    #[serde(rename = "forecastWeather")]
    pub weather: String,

    /// Forecast maximum temperature
    #[serde(rename = "forecastMaxtemp")]
    pub max_temp: ValUnit,

    /// Forecast minimum temperature
    #[serde(rename = "forecastMintemp")]
    pub min_temp: ValUnit,

    /// Forecast wind
    #[serde(rename = "forecastWind")]
    pub wind: String,

    /// Forecast maximum relative humidity
    #[serde(rename = "forecastMaxrh")]
    pub max_humidity: ValUnit,

    /// Forecast minimum relative humidity
    #[serde(rename = "forecastMinrh")]
    pub min_humidity: ValUnit,

    /// Forecast weather icon
    ///
    /// To retrieve icons, use [`icon_uri`](WeatherName::icon_uri).
    ///
    /// Visit
    /// [hko.gov.hk](https://www.hko.gov.hk/textonly/v2/explain/wxicon_e.htm)
    /// for more details.
    #[serde(rename = "ForecastIcon")]
    pub icon: WeatherName,

    /// Probability of significant rain
    #[serde(rename = "PSR")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_psr")]
    pub psr: PSR,
}

/// Sea temperature measured in `place` at `record_time`.
#[derive(Clone, Debug, Deserialize)]
pub struct SeaTemp {
    #[serde(flatten)]
    pub temp: PlaceValUnit,

    #[serde(rename = "recordTime")]
    pub record_time: DateTime<FixedOffset>,
}

/// Soil temperature measured in `place` at `record_time`.
#[derive(Clone, Debug, Deserialize)]
pub struct SoilTemp {
    #[serde(flatten)]
    pub temp: PlaceValUnit,
    pub depth: ValUnit,

    #[serde(rename = "recordTime")]
    pub record_time: DateTime<FixedOffset>,
}

impl_api!(NineDay, weather, fnd);

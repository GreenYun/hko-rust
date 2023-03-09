// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! 9-day weather forecast.
//!
//! Provides 9-day weather forecast of Hong Kong.

use chrono::{Date, DateTime, FixedOffset};
use serde::Deserialize;

use super::PSR;
use crate::{
    common::{PlaceValUnit, ValUnit},
    fetch::impl_api,
};

/// The weather forecast on specified `date`.
#[derive(Clone, Debug, Deserialize)]
pub struct WeatherForcast {
    #[serde(rename = "forecastDate")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_yyyymmdd_to_datetime")]
    pub date: Date<FixedOffset>,

    #[serde(rename = "forecastWeather")]
    pub weather: String,

    /// The weather icon
    ///
    /// To retrieve icons, use [`icon_uri`] macro to obtain the URI.
    ///
    /// Visit
    /// [hko.gov.hk](https://www.hko.gov.hk/textonly/v2/explain/wxicon_e.htm)
    /// for more details.
    #[serde(rename = "ForecastIcon")]
    pub icon: i32,

    #[serde(rename = "forecastWind")]
    pub wind: String,

    #[serde(rename = "forecastMaxtemp")]
    pub max_temp: ValUnit,

    #[serde(rename = "forecastMintemp")]
    pub min_temp: ValUnit,

    #[serde(rename = "forecastMaxrh")]
    pub max_humidity: ValUnit,

    #[serde(rename = "forecastMinrh")]
    pub min_humidity: ValUnit,

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

/// 9-day weather forecast of Hong Kong.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NineDay {
    pub general_situation: String,

    /// 9-day weather forecast
    pub weather_forecast: Vec<WeatherForcast>,
    pub sea_temp: SeaTemp,
    pub soil_temp: Vec<SoilTemp>,
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(NineDay, weather, fnd);

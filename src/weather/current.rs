// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Current weather report.
//!
//! Provides current weather report of Hong Kong.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::{
    common::{Message, PlaceValUnit},
    fetch::impl_api,
    weather::Name as WeatherName,
};

/// Current weather report of Hong Kong.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub lightning: Option<Lightning>,
    pub rainfall: Rainfall,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(rename = "uvindex")]
    pub uv_index: UVIndexOrEmpty,
    pub update_time: DateTime<FixedOffset>,

    /// Return a List. If no data for warning message, empty string will be returned.
    /// This should be fixed in the future version.
    pub warning_message: Message,
    pub rainstorm_reminder: Option<String>,

    /// Special weather tips
    #[serde(rename = "specialWxTips")]
    pub special_tips: Option<Message>,

    /// Message of tropical cyclone position
    pub tcmessage: Option<Message>,

    /// Minimum temperature from midnight to 9 am
    pub mintemp_from00_to09: Option<String>,

    /// Accumulated rainfall at HKO from midnight to noon
    pub rainfall_from00_to12: Option<String>,

    /// Rainfall in last month
    pub rainfall_last_month: Option<String>,

    /// Accumulated rainfall from January to last month
    pub rainfall_january_to_last_month: Option<String>,
    pub temperature: Temperature,
    pub humidity: Humidity,
}

/// Whether lightning `occur`s in `place`.
#[derive(Clone, Debug, Deserialize)]
pub struct LightningData {
    pub place: String,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_bool")]
    pub occur: bool,
}

/// Lightning occurring records from `start_time` to `end_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lightning {
    pub data: Vec<LightningData>,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

/// `max` and `min` rainfall measured in `place`.
///
/// Either `max` or `min` may be missing, without default value. Leave to
/// [`None`].
#[derive(Clone, Debug, Deserialize)]
pub struct RainfallData {
    pub place: String,
    pub max: Option<f32>,
    pub min: Option<f32>,
    pub unit: String,

    /// Maintenance flag
    #[serde(rename = "main")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_bool")]
    pub maintenance: bool,
}

/// Rainfall `data` measured between `start_time` and `end_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rainfall {
    pub data: Vec<RainfallData>,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

/// A List of weather icons.
///
/// Each `icon` of [`WeatherName`] for some weather description.
///
/// To retrieve icons, use [`icon_uri`](WeatherName::icon_uri) to obtain the
/// URI.
///
/// Visit
/// [hko.gov.hk](https://www.hko.gov.hk/textonly/v2/explain/wxicon_e.htm) for
/// more details.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub icon: Vec<WeatherName>,

    #[serde(rename = "iconUpdateTime")]
    pub update_time: DateTime<FixedOffset>,
}

/// UV index `value` observed from specified `place`, with additional
/// `desc`ription and `message` (optional).
#[derive(Clone, Debug, Deserialize)]
pub struct UVIndexData {
    pub place: String,
    pub value: f32,
    pub desc: String,
    pub message: Option<String>,
}

/// UV index `data` collected at specified `record_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    pub data: Vec<UVIndexData>,
    pub record_desc: String,
}

/// See [`UVIndex`].
///
/// [`Empty`](UVIndexOrEmpty::Empty) means that UV index is not applicable
/// currently, and should contain an empty string.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum UVIndexOrEmpty {
    UVIndex(UVIndex),
    Empty(String),
}

impl UVIndexOrEmpty {
    /// Returns true if the result is [`Empty`](UVIndexOrEmpty::Empty).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    /// Converts from `UVIndexOrEmpty` to [`Option`]`<`[`UVIndex`]`>`.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn uv_index(self) -> Option<UVIndex> {
        match self {
            #[allow(unused_variables)]
            Self::Empty(x) => None,
            Self::UVIndex(x) => Some(x),
        }
    }
}

/// The temperature, observed from specified `place`s, at specified
/// `recode_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub data: Vec<PlaceValUnit>,
    pub record_time: DateTime<FixedOffset>,
}

/// The relative humidity, observed from specified `place`s, at specified
/// `recode_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub data: Vec<PlaceValUnit>,
    pub record_time: DateTime<FixedOffset>,
}

impl_api!(Current, weather, rhrread);

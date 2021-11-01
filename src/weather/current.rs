// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Current weather report.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::common::{Message, PlaceValUnit};

/// Whether lightning `occur`s in `place`.
#[derive(Debug, Deserialize)]
pub struct LightningData {
    pub place: String,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_bool")]
    pub occur: bool,
}

/// Lightning occurring records from `start_time` to `end_time`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lightning {
    pub data: Vec<LightningData>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub start_time: DateTime<FixedOffset>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub end_time: DateTime<FixedOffset>,
}

/// `max` and `min` rainfall measured in `place`.
///
/// Either `max` or `min` may be missing, without default value. Leave to [`None`].
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rainfall {
    pub data: Vec<RainfallData>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub start_time: DateTime<FixedOffset>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub end_time: DateTime<FixedOffset>,
}

/// UV index `value` observed from specified `place`, with additional
/// `desc`ription and `message` (optional).
#[derive(Debug, Deserialize)]
pub struct UVIndexData {
    pub place: String,
    pub value: f64,
    pub desc: String,
    pub message: Option<String>,
}

/// UV index `data` collected at specified `record_time`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    pub data: Vec<UVIndexData>,
    pub record_desc: String,
}

/// See [`UVIndex`].
///
/// [`Empty`](UVIndexOrEmpty::Empty) means that UV index is not applicable
/// currently, and should contain an empty string.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UVIndexOrEmpty {
    UVIndex(UVIndex),
    Empty(String),
}

/// The temperature, observed from specified `place`s, at specified `recode_time`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub data: Vec<PlaceValUnit>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub record_time: DateTime<FixedOffset>,
}

/// The relative humidity, observed from specified `place`s, at specified `recode_time`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub data: Vec<PlaceValUnit>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub record_time: DateTime<FixedOffset>,
}

/// A List of weather icons.
///
/// Each `icon` (as primitive type [`i32`]) can be converted to
/// [`Name`](crate::weather::Name) for some weather description.
///
/// To retrieve icons, use [`icon_uri`] macro to obtain the URI.
///
/// Visit
/// [hko.gov.hk](https://www.hko.gov.hk/textonly/v2/explain/wxicon_e.htm) for
/// more details.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub icon: Vec<i32>,

    #[serde(rename = "iconUpdateTime")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub update_time: DateTime<FixedOffset>,
}

/// Current weather report.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub lightning: Option<Lightning>,
    pub rainfall: Rainfall,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(rename = "uvindex")]
    pub uv_index: UVIndexOrEmpty,
    pub warning_message: Option<Message>,
    pub rainstorm_reminder: Option<String>,

    /// Special weather tips
    #[serde(rename = "specialWxTips")]
    pub special_tips: Option<Message>,

    /// Message of tropical cyclone position
    pub tcmessage: Option<Message>,

    /// Minimum temperature from midnight to 9 am
    pub mintemp_from00_to09: String,

    /// Accumulated rainfall at HKO from midnight to noon
    pub rainfall_from00_to12: String,

    /// Rainfall in last month
    pub rainfall_last_month: String,

    /// Accumulated rainfall from January to last month
    pub rainfall_january_to_last_month: String,
    pub temperature: Temperature,
    pub humidity: Humidity,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Current, rhrread);

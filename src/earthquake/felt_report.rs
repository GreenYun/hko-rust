// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

/// Locally Felt Earth Tremor Report
///
/// __Note__: This API has never been tested. Use [`fetch`](crate::fetch()) at your risk.
#[derive(Debug, Deserialize)]
pub struct FeltReport {
    #[serde(rename = "lat")]
    pub latitude: f64,

    #[serde(rename = "lon")]
    pub longitude: f64,

    /// Richter magnitude scale
    #[serde(rename = "mag")]
    pub magnitude: f64,
    pub intensity: i64,
    pub region: String,
    pub details: String,

    #[serde(rename = "ptime")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub occur_time: DateTime<FixedOffset>,

    #[serde(rename = "updateTime")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(FeltReport, feltearthquake);

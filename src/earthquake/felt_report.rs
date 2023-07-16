// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Locally Felt Earth Tremor Report
//!
//! Provides reports on earth tremors felt in Hong Kong.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Reports on earth tremors felt in Hong Kong.
///
/// Note: The response from HKO may be just an empty JSON object.
#[derive(Clone, Debug, Deserialize)]
pub struct FeltReport {
    #[serde(rename = "lat")]
    pub latitude: Option<f64>,

    #[serde(rename = "lon")]
    pub longitude: Option<f64>,

    /// Richter magnitude scale
    #[serde(rename = "mag")]
    pub magnitude: Option<f64>,
    pub intensity: Option<i64>,
    pub region: Option<String>,
    pub details: Option<Vec<String>>,

    #[serde(rename = "ptime")]
    pub occur_time: Option<DateTime<FixedOffset>>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<DateTime<FixedOffset>>,
}

impl_api!(FeltReport, earthquake, feltearthquake);

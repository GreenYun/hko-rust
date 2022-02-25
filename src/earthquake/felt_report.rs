// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Locally Felt Earth Tremor Report

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Locally Felt Earth Tremor Report
///
/// __Note__: This API has never been tested. Use `fetch` at your risk.
#[derive(Clone, Debug, Deserialize)]
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
    pub occur_time: DateTime<FixedOffset>,

    #[serde(rename = "updateTime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(FeltReport, earthquake, feltearthquake);

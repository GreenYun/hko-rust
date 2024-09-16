// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Quick Earthquake Messages
//!
//! Provides information on earthquakes of magnitude 6.0 or above worldwide, as
//! analyzed by the Hong Kong Observatory.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Information on earthquakes of magnitude 6.0 or above worldwide, as analyzed
/// by the Hong Kong Observatory.
#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "lat")]
    pub latitude: f64,

    #[serde(rename = "lon")]
    pub longitude: f64,

    /// Richter magnitude scale
    #[serde(rename = "mag")]
    pub magnitude: f64,
    pub region: String,

    #[serde(rename = "ptime")]
    pub occur_time: DateTime<FixedOffset>,

    #[serde(rename = "updateTime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Message, earthquake, qem);

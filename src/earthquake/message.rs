// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Quick Earthquake Messages
#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "lat")]
    pub latitude: f64,

    #[serde(rename = "lon")]
    pub longitude: f64,

    // Richter magnitude scale
    #[serde(rename = "mag")]
    pub magnitude: f64,
    pub region: String,

    #[serde(rename = "ptime")]
    pub occur_time: DateTime<FixedOffset>,

    #[serde(rename = "updateTime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Message, earthquake, qem);

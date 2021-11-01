// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Local weather forecast.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

/// Local weather forecast.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    pub general_situation: String,
    pub tc_info: String,
    pub fire_danger_warning: String,
    pub forecast_period: String,
    pub forecast_desc: String,
    pub outlook: String,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Local, flw);

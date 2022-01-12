// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Local weather forecast.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Local weather forecast.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    pub general_situation: String,
    pub tc_info: String,
    pub fire_danger_warning: String,
    pub forecast_period: String,
    pub forecast_desc: String,
    pub outlook: String,
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Local, weather, flw);

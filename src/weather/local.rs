// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Local weather forecast.
//!
//! Provides local weather forecast for today and/or tomorrow.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// Local weather forecast for today and/or tomorrow.
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

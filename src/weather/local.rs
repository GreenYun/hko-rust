// Copyright (c) 2021 - 2024 GreenYun Organization
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
    /// General situation
    pub general_situation: String,

    /// Tropical cyclone information
    pub tc_info: String,

    /// Fire danger warning message
    pub fire_danger_warning: String,

    /// Forecast period
    pub forecast_period: String,

    /// Forecast description
    pub forecast_desc: String,

    /// Outlook
    pub outlook: String,

    /// Update time
    pub update_time: DateTime<FixedOffset>,
}

impl_api!(Local, weather, flw);

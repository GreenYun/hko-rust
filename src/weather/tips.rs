// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Special weather tips.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::fetch::impl_api;

/// A tip with `desc`ription and `update_time`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tip {
    /// May be missing if the value is null or not available.
    pub desc: Option<String>,

    /// May be missing if the value is null or not available.
    #[serde(default)]
    pub update_time: Option<DateTime<FixedOffset>>,
}

/// Special weather tips type contains none or more tips.
#[derive(Clone, Debug, Deserialize)]
pub struct Tips {
    #[serde(rename = "swt")]
    pub tips: Vec<Tip>,
}

impl_api!(Tips, weather, swt);

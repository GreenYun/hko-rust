// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Special weather tips.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

/// A tip with `desc`ription and `update_time`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tip {
    /// May be missing if the value is null or not available.
    pub desc: Option<String>,

    /// May be missing if the value is null or not available.
    #[serde(default)]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_option_datetime")]
    pub update_time: Option<DateTime<FixedOffset>>,
}

/// None or more tips.
#[derive(Debug, Deserialize)]
pub struct Tips {
    #[serde(rename = "swt")]
    pub tips: Vec<Tip>,
}

impl_api!(Tips, swt);

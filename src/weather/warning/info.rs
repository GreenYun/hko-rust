// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Weather warning information
//!
//! Provides detailed information of weather warning(s) in force.

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::{WarningStatementCode, WarningSubtypeCode};
use crate::{common::Message, fetch::impl_api};

/// Detailed information of weather warning(s) in force.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize)]
pub struct InfoDetail {
    pub contents: Option<Message>,

    /// Warning statement code
    #[serde(rename = "warningStatementCode")]
    pub code: WarningStatementCode,

    /// Warning sub-type code
    pub subtype: Option<WarningSubtypeCode>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<DateTime<FixedOffset>>,
}

/// A list of `InfoDetail` that describes every warning now in force.
#[derive(Clone, Debug, Deserialize)]
pub struct Info {
    pub details: Option<Vec<InfoDetail>>,
}

impl_api!(Info, weather, warningInfo);

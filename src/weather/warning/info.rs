// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Weather warning information

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::{WarningStatementCode, WarningSubtypeCode};
use crate::{common::Message, fetch::impl_api};

/// The details of weather warning information.
#[derive(Clone, Debug, Deserialize)]
pub struct InfoDetail {
    pub contents: Option<Message>,

    #[serde(rename = "warningStatementCode")]
    pub code: WarningStatementCode,
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

// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Weather warning information

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::{WarningStatementCode, WarningSubtypeCode};
use crate::common::Message;

/// The details of weather warning information.
#[derive(Debug, Deserialize)]
pub struct InfoDetail {
    pub contents: Option<Message>,

    #[serde(rename = "warningStatementCode")]
    pub code: WarningStatementCode,
    pub subtype: Option<WarningSubtypeCode>,

    #[serde(rename = "updateTime")]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_option_datetime")]
    pub update_time: Option<DateTime<FixedOffset>>,
}

/// A list of `InfoDetail` that describes every warning now in force.
#[derive(Debug, Deserialize)]
pub struct Info {
    pub details: Option<Vec<InfoDetail>>,
}

impl_api!(Info, warningInfo);

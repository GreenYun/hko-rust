// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Weather warning summary.
//!
//! Provides a summary of the weather warning(s) in force in Hong Kong.

use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::{Action, Code};
use crate::fetch::impl_api;

// #[allow(non_snake_case)]
// #[derive(Serialize, Deserialize)]
// struct SummaryItemRaw {
//     name: String,
//     code: String,
//     actionCode: String,
//     issueTime: String,
//     expireTime: Option<String>,
//     updateTime: String,
// }

/// Single item describes only one weather warning signal now in force.
///
/// The `type` field of original response is omitted since the type matches the
/// warning statement code and its subtype.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryItem {
    pub name: String,

    /// Warning code
    ///
    /// When the warning statement code meets one of `WFIRE`, `WRAIN`, and,
    /// `WTCSGNL`, `code` will be a
    /// [`WarningSubtypeCode`](super::WarningSubtypeCode).
    pub code: Code,

    /// Action code
    #[serde(rename = "actionCode")]
    pub action: Action,
    pub issue_time: DateTime<FixedOffset>,

    #[serde(default)]
    pub expire_time: Option<DateTime<FixedOffset>>,
    pub update_time: DateTime<FixedOffset>,
}

/// A summary of the weather warning(s) in force in Hong Kong.
///
/// Note: Each field in `fields` contains one `SummaryItem` and the key of that
/// is the warning statement code in [`String`] type, which can be converted
/// into [`WarningStatementCode`](super::WarningStatementCode).
#[derive(Clone, Debug, Deserialize)]
pub struct Summary {
    #[serde(flatten)]
    pub fields: HashMap<String, SummaryItem>,
}

impl_api!(Summary, weather, warnsum);

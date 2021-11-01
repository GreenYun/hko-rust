// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::{Action, Code};

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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryItem {
    pub name: String,

    /// Warning code
    ///
    /// When the warning statement code meets one of `WFIRE`, `WRAIN`, and,
    /// `WTCSGNL`, `code` will be a [`WarningSubtypeCode`](super::WarningSubtypeCode).
    pub code: Code,

    /// Action code
    #[serde(rename = "actionCode")]
    pub action: Action,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub issue_time: DateTime<FixedOffset>,

    #[serde(default)]
    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_option_datetime")]
    pub expire_time: Option<DateTime<FixedOffset>>,

    #[serde(deserialize_with = "crate::internal::deserialize::deserialize_to_datetime")]
    pub update_time: DateTime<FixedOffset>,
}

/// Weather warning summary.
///
/// Each field in `fields` contains one `SummaryItem` and the key of that is
/// the warning statement code in [`String`] type, which can be converted into
/// [`WarningStatementCode`](super::WarningStatementCode).
#[derive(Debug, Deserialize)]
pub struct Summary {
    #[serde(flatten)]
    pub fields: HashMap<String, SummaryItem>,
}

impl_api!(Summary, warnsum);

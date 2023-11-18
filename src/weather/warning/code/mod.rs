// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use strum::EnumString;

use crate::impl_display_traits;

/// Warning statement code.
///
/// In particular circumstance, the code may store as [`String`] type, which is
/// safe to convert to [`WarningStatementCode`].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, EnumString, Eq, PartialEq)]
pub enum WarningStatementCode {
    WFIRE,   // Fire Danger Warning
    WFROST,  // Frost Warning
    WHOT,    // Hot Weather Warning
    WCOLD,   // Cold Weather Warning
    WMSGNL,  // Strong Monsoon Signal
    WTCPRE8, // Pre-no.8 Special Announcement
    WRAIN,   // Rainstorm Warning Signal
    WFNTSA,  // Special Announcement on Flooding in the northern New Territories
    WL,      // Landslip Warning
    WTCSGNL, // Tropical Cyclone Warning Signal
    WTMW,    // Tsunami Warning
    WTS,     // Thunderstorm Warning
}

/// Warning subtype code.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, EnumString, Eq, PartialEq)]
pub enum WarningSubtypeCode {
    WFIREY, // Yellow Fire Danger Warning
    WFIRER, // Red Fire Danger Warning
    WRAINA, // Amber Rainstorm Warning
    WRAINR, // Red Rainstorm Warning
    WRAINB, // Black Rainstorm Warning
    TC1,    // No. 1 Tropical Cyclone Warning
    TC3,    // No. 3 Tropical Cyclone Warning
    TC8NE,  // No. 8 North East Tropical Cyclone Warning
    TC8SE,  // No. 8 South East Tropical Cyclone Warning
    TC8SW,  // No. 8 South West Tropical Cyclone Warning
    TC8NW,  // No. 8 North West Tropical Cyclone Warning
    TC9,    // No. 9 Tropical Cyclone Warning
    TC10,   // No. 10 Tropical Cyclone Warning
    CANCEL, // Cancel All Signals
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Code {
    WarningStatement(WarningStatementCode),
    WarningSubType(WarningSubtypeCode),
}

#[derive(Clone, Debug, Deserialize, EnumString, Eq, PartialEq)]
pub enum Action {
    ISSUE,
    REISSUE,
    CANCEL,
    EXTEND,
    UPDATE,
}

mod string;

impl_display_traits!(WarningStatementCode);
impl_display_traits!(WarningSubtypeCode);
impl_display_traits!(Code);

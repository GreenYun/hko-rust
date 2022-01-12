// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::Deserialize;
use strum_macros::EnumString;

use crate::impl_display_traits;

/// Warning statement code.
///
/// In particular circumstance, the code may store as [`String`] type, which is
/// safe to convert to [`WarningStatementCode`].
#[derive(Clone, Debug, Deserialize, EnumString, PartialEq)]
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
#[derive(Clone, Debug, Deserialize, EnumString, PartialEq)]
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
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Code {
    WarningStatement(WarningStatementCode),
    WarningSubType(WarningSubtypeCode),
}

#[derive(Clone, Debug, Deserialize, EnumString, PartialEq)]
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

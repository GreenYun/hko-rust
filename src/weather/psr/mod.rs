// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{error::InvalidPSRError, impl_display_traits};

/// Probability of significant rain.
///
/// More information about PSR, please refer
/// [hko.gov.hk](https://www.hko.gov.hk/en/wxinfo/currwx/fnd.htm?tablenote=true).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PSR {
    High,
    MediumHigh,
    Medium,
    MediumLow,
    Low,
}

impl FromStr for PSR {
    type Err = InvalidPSRError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "High" | "高" => Ok(Self::High),
            "Medium High" | "中高" => Ok(Self::MediumHigh),
            "Medium" | "中" => Ok(Self::Medium),
            "Medium Low" | "中低" => Ok(Self::MediumLow),
            "Low" | "低" => Ok(Self::Low),
            _ => Err(InvalidPSRError),
        }
    }
}

mod string;

impl_display_traits!(PSR);

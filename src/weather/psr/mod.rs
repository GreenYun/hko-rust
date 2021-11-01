// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error, fmt, str::FromStr};

use crate::impl_display_traits;

/// This error occurs when trying to convert invalid string to PSR type.
#[derive(Debug, Clone, Copy)]
pub struct InvalidPSRError;

impl fmt::Display for InvalidPSRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "PSR value is not valid".fmt(f)
    }
}

impl error::Error for InvalidPSRError {}

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
            "High" | "高" => Ok(PSR::High),
            "Medium High" | "中高" => Ok(PSR::MediumHigh),
            "Medium" | "中" => Ok(PSR::Medium),
            "Medium Low" | "中低" => Ok(PSR::MediumLow),
            "Low" | "低" => Ok(PSR::Low),
            _ => Err(InvalidPSRError),
        }
    }
}

mod string;

impl_display_traits!(PSR);

mod test;

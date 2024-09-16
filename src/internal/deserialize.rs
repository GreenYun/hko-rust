// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(clippy::module_name_repetitions, unused_imports)]

use std::{fmt, str::FromStr};

use chrono::NaiveDate;
use serde::{
    de::{Error as DeError, Expected, Unexpected, Visitor},
    Deserializer,
};

#[cfg(feature = "hourly_rainfall")]
use crate::hourly_rainfall::RainfallValue;
#[cfg(feature = "weather")]
use crate::weather::PSR;

#[cfg(feature = "weather")]
pub fn deserialize_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "`TRUE` or `FALSE`";

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            match value.to_uppercase().as_str() {
                "TRUE" => Ok(true),
                "FALSE" | "" => Ok(false),
                _ => Err(DeError::invalid_value(Unexpected::Str(value), &EXPECTING)),
            }
        }
    }

    deserializer.deserialize_identifier(BoolVisitor)
}

#[cfg(feature = "weather")]
pub fn deserialize_yyyymmdd_to_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "string of date formatted in YYYYMMDD";

    struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = NaiveDate;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            NaiveDate::parse_from_str(value, "%Y%m%d")
                .map_err(|_| DeError::invalid_value(Unexpected::Str(value), &EXPECTING))
        }
    }

    deserializer.deserialize_identifier(DateVisitor)
}

#[cfg(feature = "weather")]
pub fn deserialize_to_psr<'de, D>(deserializer: D) -> Result<PSR, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "string of the probability of significant rain";

    struct PSRVisitor;

    impl<'de> Visitor<'de> for PSRVisitor {
        type Value = PSR;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            PSR::from_str(value).map_err(|_| DeError::invalid_value(Unexpected::Str(value), &EXPECTING))
        }
    }

    deserializer.deserialize_identifier(PSRVisitor)
}

#[cfg(feature = "hourly_rainfall")]
pub fn deserialize_to_rainfall_value<'de, D>(deserializer: D) -> Result<RainfallValue, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "`M` or an integer";

    struct RainfallValueVisitor;

    impl<'de> Visitor<'de> for RainfallValueVisitor {
        type Value = RainfallValue;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            match value.to_uppercase().as_str() {
                "M" => Ok(RainfallValue::UnderMaintenance),
                s => s
                    .parse()
                    .map(Self::Value::Rainfall)
                    .map_err(|_| DeError::invalid_value(Unexpected::Str(value), &EXPECTING)),
            }
        }
    }

    deserializer.deserialize_identifier(RainfallValueVisitor)
}

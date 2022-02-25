// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt, str::FromStr};

use chrono::{Date, FixedOffset, NaiveDate, TimeZone};
use serde::{
    de::{Error as DeError, Expected, Unexpected, Visitor},
    Deserializer,
};

use crate::weather::PSR;

pub(crate) fn deserialize_yyyymmdd_to_datetime<'de, D>(
    deserializer: D,
) -> Result<Date<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "string of date formatted in YYYYMMDD";

    struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = Date<FixedOffset>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            let naive_date = NaiveDate::parse_from_str(value, "%Y%m%d")
                .map_err(|_| DeError::invalid_value(Unexpected::Str(value), &EXPECTING))?;

            FixedOffset::east(8 * 60 * 60)
                .from_local_date(&naive_date)
                .single()
                .ok_or(DeError::invalid_value(Unexpected::Str(value), &EXPECTING))
        }
    }

    deserializer.deserialize_identifier(DateVisitor)
}

pub(crate) fn deserialize_to_psr<'de, D>(deserializer: D) -> Result<PSR, D::Error>
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
            PSR::from_str(value)
                .map_err(|_| DeError::invalid_value(Unexpected::Str(value), &EXPECTING))
        }
    }

    deserializer.deserialize_identifier(PSRVisitor)
}

pub(crate) fn deserialize_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

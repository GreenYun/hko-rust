// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{fmt, str::FromStr};

use chrono::{Date, DateTime, FixedOffset, NaiveDate};
use serde::{
    de::{self, Expected},
    Deserializer,
};

pub(crate) fn deserialize_yyyymmdd_to_datetime<'de, D>(
    deserializer: D,
) -> Result<Date<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "string of date formatted in YYYYMMDD";

    struct DateVisitor;

    impl<'de> de::Visitor<'de> for DateVisitor {
        type Value = Date<FixedOffset>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let naive_date = match NaiveDate::parse_from_str(value, "%Y%m%d") {
                Ok(d) => d,
                _ => {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(value),
                        &EXPECTING,
                    ))
                }
            };

            Ok(Date::from_utc(naive_date, FixedOffset::east(8 * 60 * 60)))
        }
    }

    deserializer.deserialize_identifier(DateVisitor)
}

const EXPECTING: &str = "string of date and time format described in RFC3339";

struct DateTimeVisitor;

impl<'de> de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        EXPECTING.fmt(formatter)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DateTime::parse_from_rfc3339(value) {
            Ok(date_time) => Ok(date_time),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Str(value),
                &EXPECTING,
            )),
        }
    }
}

pub(crate) fn deserialize_to_datetime<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_identifier(DateTimeVisitor)
}

pub(crate) fn deserialize_to_option_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    match deserializer.deserialize_identifier(DateTimeVisitor) {
        Ok(d) => Ok(Some(d)),
        Err(e) => Err(e),
    }
}

use crate::weather::PSR;

pub(crate) fn deserialize_to_psr<'de, D>(deserializer: D) -> Result<PSR, D::Error>
where
    D: Deserializer<'de>,
{
    const EXPECTING: &str = "string of the probability of significant rain";

    struct PSRVisitor;

    impl<'de> de::Visitor<'de> for PSRVisitor {
        type Value = PSR;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match PSR::from_str(value) {
                Ok(psr) => Ok(psr),
                Err(_) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(value),
                    &EXPECTING,
                )),
            }
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

    impl<'de> de::Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            EXPECTING.fmt(formatter)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value.to_uppercase().as_str() {
                "TRUE" => Ok(true),
                "FALSE" | "" => Ok(false),
                _ => Err(de::Error::invalid_value(
                    de::Unexpected::Str(value),
                    &EXPECTING,
                )),
            }
        }
    }

    deserializer.deserialize_identifier(BoolVisitor)
}

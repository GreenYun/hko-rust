// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Open Data (Climate and Weather Information)
//!
//! Open data contains such datasets:
//! - Hourly heights of astronomical tides ([`hhot`])
//! - Times and heights of astronomical high and low tides ([`hlt`])
//! - Times of sunrise, sun transit and sunset ([`srs`])
//! - Times of moonrise, moon transit and moonset ([`mrs`])
//! - Cloud-to-ground and cloud-to-cloud lightning count ([`lhl`])
//! - Latest 10-minute mean visibility ([`ltmv`])
//! - Daily Mean Temperature ([`clmtemp`])
//! - Daily Maximum Temperature ([`clmmaxt`])
//! - Daily Minimum Temperature ([`clmmint`])
//! - Weather and Radiation Level Report ([`ryes`])
//!
//! **The main data type in each module is `Response`.**
//!
//! Each `Response` has implementation of
//! [`FromStr`](std::str::FromStr). Parse the string with
//! [`from_str`](std::str::FromStr::from_str) and get the data type if
//! succeeded.
//!
//! Public funtions `url` and `fetch` are provided in each module to fetch data
//! with the API.

macro_rules! concat_url {
    ($datatype:ident, $tail:literal) => {
        concat!(
            "https://data.weather.gov.hk/weatherAPI/opendata/opendata.php?dataType=",
            stringify!($datatype),
            $tail
        )
    };
}

use concat_url;
use strum_macros::Display;

pub use self::{clm::*, rs::*, station::*};

#[derive(Clone, Debug, Display, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ResponseFormat {
    JSON,
    CSV,
}

mod clm;
pub mod hhot;
pub mod hlt;
pub mod lhl;
pub mod ltmv;
mod rs;
pub mod ryes;
mod station;

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

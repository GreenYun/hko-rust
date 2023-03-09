// Copyright (c) 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Gregorian-Lunar calendar conversion table

use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

use crate::error::APIRequestError;

/// Main response type.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    /// Lunar year, Gan-Zhi and Zodiac, in traditional Chinese.
    pub lunar_year: String,

    /// Lunar date, in traditional Chinese.
    pub lunar_date: String,
}

/// Generate API URL from specified date.
///
/// # Errors
///
/// Returns [`APIRequestError`] if specified date is not illegal or out of
/// range.
pub fn url(date: NaiveDate) -> Result<String, APIRequestError> {
    if date.year() != 2023 {
        return Err(APIRequestError("year must be 2021-2024".into()));
    }

    Ok(format!(
        "https://data.weather.gov.hk/weatherAPI/opendata/lunardate.php?date={}",
        date.format("%Y-%m-%d")
    ))
}

#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(date: NaiveDate) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(serde_json::from_str(&get(url(date).unwrap()).await?.text().await?)?)
}

#[cfg(test)]
mod test;

// Copyright (c) 2023 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Gregorian-Lunar calendar conversion table
//!
//! - **HTTP Request Method**: GET
//! - **Return Type**: JSON

use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

use crate::error::APIRequestError;

/// Main response type.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    /// Lunar year, Gan-Zhi and Zodiac (in traditional Chinese)
    pub lunar_year: String,

    /// Lunar date (in traditional Chinese)
    pub lunar_date: String,
}

/// Generate API URL from specified date.
///
/// # Errors
///
/// Returns [`APIRequestError`] if specified date is not illegal or out of
/// range.
pub fn url(date: NaiveDate) -> Result<String, APIRequestError> {
    if !matches!(date.year(), 2023 | 2024) {
        return Err(APIRequestError("date must be between 2023-01-01 and 2024-12-31".into()));
    }

    Ok(format!(
        "https://data.weather.gov.hk/weatherAPI/opendata/lunardate.php?date={}",
        date.format("%Y-%m-%d")
    ))
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
pub async fn fetch(date: NaiveDate) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(serde_json::from_str(&get(url(date)?).await?.text().await?)?)
}

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

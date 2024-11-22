// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Provides predicted tidal information. (Times and heights of astronomical
//! high and low tides)

use std::str::FromStr;

use serde::Deserialize;

use crate::{
    error::{APIRequestError, DataError},
    opendata::{ResponseFormat, SeaStation},
};

#[derive(Clone, Debug)]
pub struct ResponseUnit {
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub height: f32,
}

/// Main response type.
#[derive(Clone, Debug)]
pub struct Response(pub Vec<ResponseUnit>);

impl FromStr for Response {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().as_bytes();

        macro_rules! response_unit {
            ($month:expr, $day:expr, $hour:expr, $minute:expr, $height:expr) => {
                ResponseUnit {
                    month: $month,
                    day: $day,
                    hour: $hour,
                    minute: $minute,
                    height: $height,
                }
            };
        }

        Ok(Self(if raw.first().ok_or(DataError::EarlyEOF)? == &b'{' {
            // JSON
            #[derive(Deserialize)]
            struct JsonResponse {
                data: Vec<Vec<String>>,
            }

            let JsonResponse { data } = serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

            data.into_iter()
                .filter_map(|v| {
                    let month = v.first()?.parse::<u32>().ok()?;
                    let day = v.get(1)?.parse::<u32>().ok()?;

                    Some(
                        v.windows(2)
                            .skip(2)
                            .step_by(2)
                            .filter_map(|s| {
                                let time = s[0].parse::<u32>().ok()?;
                                let hour = time / 100;
                                let minute = time % 100;

                                let height = s[1].parse::<f32>().ok()?;

                                Some(response_unit!(month, day, hour, minute, height))
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .collect()
        } else {
            // CSV
            #[derive(Deserialize)]
            struct CsvResponse {
                month: u32,
                day: u32,
                data: Vec<String>,
            }

            let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(raw);

            rdr.records()
                .filter_map(|r| {
                    let CsvResponse { month, day, data } = r.ok()?.deserialize(None).ok()?;

                    Some(
                        data.windows(2)
                            .step_by(2)
                            .filter_map(|s| {
                                let time = s[0].parse::<u32>().ok()?;
                                let hour = time / 100;
                                let minute = time % 100;

                                let height = s[1].parse::<f32>().ok()?;

                                Some(response_unit! {month, day, hour, minute, height})
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .collect()
        }))
    }
}

/// The API also accepts `month`, `day`, and `hour`, but they are not used, and
/// we omit them.
///
/// # Errors
///
/// Returns [`APIRequestError`] if year out of historical range.
pub fn url(station: SeaStation, year: i32, response_format: Option<ResponseFormat>) -> Result<String, APIRequestError> {
    if !matches!(year, 2021..=2024) {
        return Err(APIRequestError(format!("year must be 2021-2024, got {year}")));
    }

    Ok(format!(
        concat_url!(HLT, "&station={}&year={}{}"),
        station,
        year,
        response_format.map(|f| format!("&rformat={f}")).unwrap_or_default(),
    ))
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[cfg_attr(docsrs, doc(cfg(feature = "fetch")))]
pub async fn fetch(
    station: SeaStation,
    year: i32,
    response_format: Option<ResponseFormat>,
) -> anyhow::Result<Response> {
    let client = reqwest::Client::builder().build()?;

    fetch_with_client(station, year, response_format, client).await
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[cfg_attr(docsrs, doc(cfg(feature = "fetch")))]
pub async fn fetch_with_client(
    station: SeaStation,
    year: i32,
    response_format: Option<ResponseFormat>,
    client: reqwest::Client,
) -> anyhow::Result<Response> {
    let resp = client
        .get(url(station, year, response_format)?)
        .send()
        .await?
        .text()
        .await?;

    Ok(Response::from_str(&resp)?)
}

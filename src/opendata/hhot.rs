// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Provides predicted tidal information. (Hourly heights of astronomical tides)

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
    pub height: f32,
}

/// Main response type.
#[derive(Clone, Debug)]
pub struct Response(pub Vec<ResponseUnit>);

impl FromStr for Response {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().as_bytes();

        Ok(Self(if raw.first().ok_or(DataError::EarlyEOF)? == &b'{' {
            // JSON
            #[derive(Deserialize)]
            struct JsonResponse {
                fields: Vec<String>,
                data: Vec<Vec<String>>,
            }

            let JsonResponse { fields, data } =
                serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

            let hours = fields
                .into_iter()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>();

            data.into_iter()
                .filter_map(|v| {
                    let month = v.first()?.parse().ok()?;
                    let day = v.get(1)?.parse().ok()?;

                    Some(
                        v.get(2..)?
                            .iter()
                            .enumerate()
                            .filter_map(|(i, s)| {
                                let height = s.parse().ok()?;

                                Some(ResponseUnit {
                                    month,
                                    day,
                                    hour: hours.get(i).copied().unwrap_or(i.try_into().unwrap_or(0) + 1),
                                    height,
                                })
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
                mm: u32,
                dd: u32,
                data: Vec<f32>,
            }

            let has_header = s
                .chars()
                .any(|c| !c.is_ascii_digit() && !c.is_whitespace() && !matches!(c, ',' | '.'));

            let mut rdr = csv::ReaderBuilder::new().has_headers(has_header).from_reader(raw);

            let hours = if has_header {
                let header = rdr.headers().map_err(|e| DataError::SourceFormat(e.to_string()))?;

                header.into_iter().skip(2).filter_map(|x| x.parse().ok()).collect()
            } else {
                Vec::new()
            };

            rdr.records()
                .filter_map(|r| {
                    let CsvResponse {
                        mm: month,
                        dd: day,
                        data,
                    } = r.ok()?.deserialize(None).ok()?;

                    Some(
                        data.into_iter()
                            .enumerate()
                            .map(|(h, height)| ResponseUnit {
                                month,
                                day,
                                hour: if has_header {
                                    hours.get(h).copied().unwrap_or(h.try_into().unwrap_or(0) + 1)
                                } else {
                                    h.try_into().unwrap_or(0) + 1
                                },
                                height,
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .collect()
        }))
    }
}

/// Generate API URL from specified date.
///
/// # Errors
///
/// Returns [`APIRequestError`] if specified date is not illegal or out of
/// historical range.
pub fn url(
    station: &SeaStation,
    year: i32,
    month: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
    response_format: Option<ResponseFormat>,
) -> Result<String, APIRequestError> {
    use std::fmt::Write;

    if !matches!(year, 2021..=2024) {
        return Err(APIRequestError("year must be 2021-2024".to_owned()));
    }

    let mut s = String::new();

    if let Some(month) = month {
        if !(1..=12).contains(&month) {
            return Err(APIRequestError("month must be 1-12".to_owned()));
        }

        let _: Result<_, _> = write!(s, "&month={month}");
    }

    if let Some(day) = day {
        if !(1..=31).contains(&day) && month.is_none() {
            return Err(APIRequestError(
                "day must be 1-31 and month must be specified".to_owned(),
            ));
        }

        let _: Result<_, _> = write!(s, "&day={day}");
    }

    if let Some(hour) = hour {
        if !(1..=24).contains(&hour) && day.is_none() {
            return Err(APIRequestError(
                "hour must be 1-24 and day must be specified".to_owned(),
            ));
        }

        let _: Result<_, _> = write!(s, "&hour={hour}");
    }

    Ok(format!(
        concat_url!(HHOT, "&station={}&year={}{}{}"),
        station,
        year,
        response_format.map(|f| format!("&rformat={f}")).unwrap_or_default(),
        s,
    ))
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(
    station: SeaStation,
    year: i32,
    month: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
    response_format: Option<ResponseFormat>,
) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(Response::from_str(
        &get(url(&station, year, month, day, hour, response_format)?)
            .await?
            .text()
            .await?,
    )?)
}

// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Latest 10-minute mean visibility.

use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::{
    common::{Lang, ValUnit},
    error::DataError,
    opendata::{concat_url, ResponseFormat},
};

#[derive(Clone, Debug)]
pub struct ResponseUnit {
    pub time: DateTime<FixedOffset>,
    pub station: String,
    pub visibility: ValUnit,
}

/// Main response type.
#[derive(Clone, Debug)]
pub struct Response(pub Vec<ResponseUnit>);

impl FromStr for Response {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().as_bytes();

        Ok(Self(match raw.get(0).ok_or(DataError::EarlyEOF)? {
            // JSON
            b'{' => {
                #[derive(Deserialize)]
                struct JsonResponse {
                    data: Vec<Vec<String>>,
                }

                let JsonResponse { data } =
                    serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

                data.into_iter()
                    .filter_map(|v| {
                        let time = NaiveDateTime::parse_from_str(v.get(0)?, "%Y%m%d%H%M").ok()?;
                        let time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&time)
                            .single()?;

                        let station = v.get(1)?.to_string();

                        let visibility = v.get(2)?;

                        let visibility = {
                            use nom::{error, number::complete};

                            let (unit, value) =
                                complete::float::<_, error::Error<_>>(visibility.as_str()).ok()?;

                            ValUnit {
                                value,
                                unit: unit.trim().to_string(),
                            }
                        };

                        Some(ResponseUnit {
                            time,
                            station,
                            visibility,
                        })
                    })
                    .collect()
            }

            // CSV
            _ => {
                #[derive(Deserialize)]
                struct CsvResponse {
                    time: String,
                    station: String,
                    visibility: String,
                }

                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(s.as_bytes());

                rdr.records()
                    .filter_map(|r| {
                        let CsvResponse {
                            time,
                            station,
                            visibility,
                        } = r.ok()?.deserialize(None).ok()?;

                        let time =
                            NaiveDateTime::parse_from_str(time.as_str(), "%Y%m%d%H%M").ok()?;
                        let time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&time)
                            .single()?;

                        let visibility = {
                            use nom::{error, number::complete};

                            let (unit, value) =
                                complete::float::<_, error::Error<_>>(visibility.as_str()).ok()?;

                            ValUnit {
                                value,
                                unit: unit.trim().to_string(),
                            }
                        };

                        Some(ResponseUnit {
                            time,
                            station,
                            visibility,
                        })
                    })
                    .collect()
            }
        }))
    }
}

pub fn url(lang: Lang, response_format: Option<ResponseFormat>) -> String {
    format!(
        concat_url!(LTMV, "&lang={}{}"),
        lang.to_string(),
        response_format
            .map(|f| format!("&rformat={}", f))
            .unwrap_or(String::new()),
    )
}

#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(
    lang: Lang,
    response_format: Option<ResponseFormat>,
) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(Response::from_str(
        &get(url(lang, response_format)).await?.text().await?,
    )?)
}

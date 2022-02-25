// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Cloud-to-ground and cloud-to-cloud lightning count.

use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::{
    common::Lang,
    error::DataError,
    opendata::{concat_url, ResponseFormat},
};

#[derive(Clone, Debug)]
pub struct ResponseUnit {
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub r#type: String,
    pub region: String,
    pub count: u32,
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
                        let time = v.get(0)?.split('-').collect::<Vec<_>>();
                        let start_time =
                            NaiveDateTime::parse_from_str(time.get(0)?, "%Y%m%d%H%M").ok()?;
                        let start_time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&start_time)
                            .single()?;

                        let end_time =
                            NaiveDateTime::parse_from_str(time.get(1)?, "%Y%m%d%H%M").ok()?;
                        let end_time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&end_time)
                            .single()?;

                        let r#type = v.get(1)?.clone();
                        let region = v.get(2)?.clone();
                        let count = (*v.get(3)?).parse::<u32>().ok()?;

                        Some(ResponseUnit {
                            start_time,
                            end_time,
                            r#type,
                            region,
                            count,
                        })
                    })
                    .collect()
            }

            // CSV
            _ => {
                #[derive(Deserialize)]
                struct CsvResponse {
                    time: String,
                    r#type: String,
                    region: String,
                    count: u32,
                }

                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(raw);

                rdr.records()
                    .filter_map(|r| {
                        let CsvResponse {
                            time,
                            r#type,
                            region,
                            count,
                        } = r.ok()?.deserialize(None).ok()?;

                        let time = time.split('-').collect::<Vec<_>>();

                        let start_time =
                            NaiveDateTime::parse_from_str(time.get(0)?, "%Y%m%d%H%M").ok()?;
                        let start_time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&start_time)
                            .single()?;

                        let end_time =
                            NaiveDateTime::parse_from_str(time.get(1)?, "%Y%m%d%H%M").ok()?;
                        let end_time = FixedOffset::east(8 * 60 * 60)
                            .from_local_datetime(&end_time)
                            .single()?;

                        Some(ResponseUnit {
                            start_time,
                            end_time,
                            r#type,
                            region,
                            count,
                        })
                    })
                    .collect()
            }
        }))
    }
}

pub fn url(lang: Lang, response_format: Option<ResponseFormat>) -> String {
    format!(
        concat_url!(LHL, "&lang={}{}"),
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

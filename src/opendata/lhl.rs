// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Provides cloud-to-ground and cloud-to-cloud lightning count over Hong Kong
//! territory in the past hour. (the data provided is provisional)

use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::{
    common::Lang,
    error::DataError,
    internal::hkt,
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

        Ok(Self(if raw.first().ok_or(DataError::EarlyEOF)? == &b'{' {
            // JSON
            #[derive(Deserialize)]
            struct JsonResponse {
                data: Vec<Vec<String>>,
            }

            let JsonResponse { data } = serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

            data.into_iter()
                .filter_map(|v| {
                    let time = v.first()?.split('-').collect::<Vec<_>>();
                    let start_time = NaiveDateTime::parse_from_str(time.first()?, "%Y%m%d%H%M").ok()?;
                    let start_time = hkt().from_local_datetime(&start_time).single()?;

                    let end_time = NaiveDateTime::parse_from_str(time.get(1)?, "%Y%m%d%H%M").ok()?;
                    let end_time = hkt().from_local_datetime(&end_time).single()?;

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
        } else {
            // CSV
            #[derive(Deserialize)]
            struct CsvResponse {
                time: String,
                r#type: String,
                region: String,
                count: u32,
            }

            let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(raw);

            rdr.records()
                .filter_map(|r| {
                    let CsvResponse {
                        time,
                        r#type,
                        region,
                        count,
                    } = r.ok()?.deserialize(None).ok()?;

                    let time = time.split('-').collect::<Vec<_>>();

                    let start_time = NaiveDateTime::parse_from_str(time.first()?, "%Y%m%d%H%M").ok()?;
                    let start_time = hkt().from_local_datetime(&start_time).single()?;

                    let end_time = NaiveDateTime::parse_from_str(time.get(1)?, "%Y%m%d%H%M").ok()?;
                    let end_time = hkt().from_local_datetime(&end_time).single()?;

                    Some(ResponseUnit {
                        start_time,
                        end_time,
                        r#type,
                        region,
                        count,
                    })
                })
                .collect()
        }))
    }
}

#[must_use]
pub fn url(lang: &Lang, response_format: Option<ResponseFormat>) -> String {
    format!(
        concat_url!(LHL, "&lang={}{}"),
        lang,
        response_format.map(|f| format!("&rformat={f}")).unwrap_or_default(),
    )
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(lang: Lang, response_format: Option<ResponseFormat>) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(Response::from_str(
        &get(url(&lang, response_format)).await?.text().await?,
    )?)
}

// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Provides regional weather in Hong Kong - latest 10-minute mean visibility.
//! (the data provided is provisional)

use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use chrono_tz::Hongkong;
use serde::Deserialize;

use crate::{
    common::{Lang, ValUnit},
    error::DataError,
    opendata::ResponseFormat,
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

        Ok(Self(if raw.first().ok_or(DataError::EarlyEOF)? == &b'{' {
            // JSON
            #[derive(Deserialize)]
            struct JsonResponse {
                data: Vec<Vec<String>>,
            }

            let JsonResponse { data } = serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

            data.into_iter()
                .filter_map(|v| {
                    let time = NaiveDateTime::parse_from_str(v.first()?, "%Y%m%d%H%M").ok()?;
                    let time = Hongkong.from_local_datetime(&time).single()?.fixed_offset();

                    let station = v.get(1)?.to_string();

                    let visibility = v.get(2)?;

                    let visibility = {
                        use nom::{error, number::complete};

                        let (unit, value) = complete::float::<_, error::Error<_>>(visibility.as_str()).ok()?;

                        ValUnit {
                            value,
                            unit: unit.trim().to_owned(),
                        }
                    };

                    Some(ResponseUnit {
                        time,
                        station,
                        visibility,
                    })
                })
                .collect()
        } else {
            // CSV
            #[derive(Deserialize)]
            struct CsvResponse {
                time: String,
                station: String,
                visibility: String,
            }

            let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(s.as_bytes());

            rdr.records()
                .filter_map(|r| {
                    let CsvResponse {
                        time,
                        station,
                        visibility,
                    } = r.ok()?.deserialize(None).ok()?;

                    let time = NaiveDateTime::parse_from_str(time.as_str(), "%Y%m%d%H%M").ok()?;
                    let time = Hongkong.from_local_datetime(&time).single()?.fixed_offset();

                    let visibility = {
                        use nom::{error, number::complete};

                        let (unit, value) = complete::float::<_, error::Error<_>>(visibility.as_str()).ok()?;

                        ValUnit {
                            value,
                            unit: unit.trim().to_owned(),
                        }
                    };

                    Some(ResponseUnit {
                        time,
                        station,
                        visibility,
                    })
                })
                .collect()
        }))
    }
}

#[must_use]
pub fn url(lang: &Lang, response_format: Option<ResponseFormat>) -> String {
    format!(
        concat_url!(LTMV, "&lang={}{}"),
        lang,
        response_format.map(|f| format!("&rformat={f}")).unwrap_or_default(),
    )
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[cfg_attr(docsrs, doc(cfg(feature = "fetch")))]
pub async fn fetch(lang: Lang, response_format: Option<ResponseFormat>) -> anyhow::Result<Response> {
    let client = reqwest::Client::builder().build()?;

    fetch_with_client(lang, response_format, client).await
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[cfg_attr(docsrs, doc(cfg(feature = "fetch")))]
pub async fn fetch_with_client(
    lang: Lang,
    response_format: Option<ResponseFormat>,
    client: reqwest::Client,
) -> anyhow::Result<Response> {
    let resp = client.get(url(&lang, response_format)).send().await?.text().await?;

    Ok(Response::from_str(&resp)?)
}

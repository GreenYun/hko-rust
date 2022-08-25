// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Times and heights of astronomical high and low tides.

use std::str::FromStr;

use serde::Deserialize;

use crate::{
    error::{APIRequestError, DataError},
    opendata::{concat_url, ResponseFormat, SeaStation},
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

            let JsonResponse { data } =
                serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

            data.into_iter()
                .filter_map(|v| {
                    let month = v.get(0)?.parse::<u32>().ok()?;
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

            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(raw);

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
pub fn url(
    station: &SeaStation,
    year: i32,
    response_format: Option<ResponseFormat>,
) -> Result<String, APIRequestError> {
    if !matches!(year, 2021..=2024) {
        return Err(APIRequestError(format!(
            "year must be 2021-2024, got {}",
            year
        )));
    }

    Ok(format!(
        concat_url!(HLT, "&station={}&year={}{}"),
        station,
        year,
        response_format
            .map(|f| format!("&rformat={}", f))
            .unwrap_or_default(),
    ))
}

#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(
    station: SeaStation,
    year: i32,
    response_format: Option<ResponseFormat>,
) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(Response::from_str(
        &get(url(&station, year, response_format)?)
            .await?
            .text()
            .await?,
    )?)
}

#[cfg(feature = "test")]
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Response;

    macro_rules! response_from_str {
        ($s:expr $(,)?) => {{
            super::Response::from_str($s).unwrap()
        }};
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn test() {
        // CSV with header
        let Response(r1) = response_from_str!(
            r#"Month,Date,Time,Height(m),Time,Height(m),Time,Height(m),Time,Height(m)
01,01,0219,0.53,0930,1.55,1308,1.26,1934,2.60
01,02,0313,0.31,1030,1.55,,,,
01,03,0406,0.18,1121,1.54,1443,1.30,,"#,
        );

        // CSV without header
        let Response(r2) = response_from_str!(
            r#"01,01,0219,0.53,0930,1.55,1308,1.26,1934,2.60
01,02,0313,0.31,1030,1.55,,,,
01,03,0406,0.18,1121,1.54,1443,1.30,,"#,
        );

        // JSON
        let Response(r3) = response_from_str!(
            r#"{
    "fields": ["Month", "Date", "Time", "Height(m)", "Time", "Height(m)", "Time", "Height(m)", "Time", "Height(m)"],
    "data": [
        ["01", "01", "0219", "0.53", "0930", "1.55", "1308", "1.26", "1934", "2.60"],
        ["01", "02", "0313", "0.31", "1030", "1.55", "", "", "", ""],
        ["01", "03", "0406", "0.18", "1121", "1.54", "1443", "1.30", "", ""]]}"#,
        );

        assert!(r1.len() == r2.len() && r2.len() == r3.len());
        assert!(r1[0].month == r2[0].month && r2[0].month == r3[0].month);
        assert!(r1[0].day == r2[0].day && r2[0].day == r3[0].day);
        assert!(r1[0].hour == r2[0].hour && r2[0].hour == r3[0].hour);
        assert!(r1[0].minute == r2[0].minute && r2[0].minute == r3[0].minute);
        assert!(r1[0].height == r2[0].height && r2[0].height == r3[0].height);
    }
}

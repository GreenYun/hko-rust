// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

macro_rules! time_format {
    () => {
        "{}T{}:00+08:00"
    };
}

macro_rules! response_unit {
    ($rise:expr, $transit:expr, $set:expr $(,)?) => {
        ResponseUnit {
            rise: DateTime::parse_from_rfc3339($rise).ok()?,
            transit: DateTime::parse_from_rfc3339($transit).ok()?,
            set: DateTime::parse_from_rfc3339($set).ok()?,
        }
    };
}

#[allow(unused_macros)]
macro_rules! response_from_str {
    ($s:expr $(,)?) => {{
        super::Response::from_str($s).unwrap()
    }};
}

macro_rules! impl_rs {
    ($i:ident) => {
        use std::str::FromStr;

        use chrono::{DateTime, FixedOffset};
        use serde::Deserialize;

        use crate::{
            error::{APIRequestError, DataError},
            opendata::ResponseFormat,
        };

        #[derive(Clone, Debug)]
        pub struct ResponseUnit {
            pub rise: DateTime<FixedOffset>,
            pub transit: DateTime<FixedOffset>,
            pub set: DateTime<FixedOffset>,
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
                            .filter_map(|row| {
                                if row.len() != 4 {
                                    return None;
                                };

                                let rise = format!(time_format!(), row[0], row[1]);
                                let transit = format!(time_format!(), row[0], row[2]);
                                let set = format!(time_format!(), row[0], row[3]);

                                Some(response_unit! {&rise, &transit, &set})
                            })
                            .collect()
                    }

                    //CSV
                    _ => {
                        #[derive(Deserialize)]
                        struct CsvResponse {
                            date: String,
                            rise: String,
                            transit: String,
                            set: String,
                        }

                        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(raw);

                        rdr.records()
                            .filter_map(|r| {
                                let CsvResponse {
                                    date,
                                    rise,
                                    transit,
                                    set,
                                } = r.ok()?.deserialize(None).ok()?;

                                let rise = format!(time_format!(), date, rise);
                                let transit = format!(time_format!(), date, transit);
                                let set = format!(time_format!(), date, set);

                                Some(response_unit! {&rise, &transit, &set})
                            })
                            .collect()
                    }
                }))
            }
        }

        /// Generate API URL from specified date.
        ///
        /// # Errors
        ///
        /// Returns [`APIRequestError`] if specified date is not illegal or out
        /// of historical range.
        pub fn url(
            year: i32,
            month: Option<u32>,
            day: Option<u32>,
            response_format: Option<ResponseFormat>,
        ) -> Result<String, APIRequestError> {
            use std::fmt::Write;

            if !matches!(year, 2018..=2024) {
                return Err(APIRequestError("year must be 2018-2024".to_owned()));
            }

            let mut s = String::new();

            if let Some(month) = month {
                if !(1..=12).contains(&month) {
                    return Err(APIRequestError("month must be 1-12".to_owned()));
                }

                let _: Result<_, _> = write!(s, "&month={month}");
            }

            if let Some(day) = day {
                if !(1..=31).contains(&day) && month == None {
                    return Err(APIRequestError(
                        "day must be 1-31 and month must be specified".to_owned(),
                    ));
                }

                let _: Result<_, _> = write!(s, "&day={day}");
            }

            Ok(format!(
                crate::opendata::concat_url!($i, "&year={}{}{}"),
                year,
                response_format
                    .map(|f| format!("&rformat={}", f))
                    .unwrap_or_default(),
                s,
            ))
        }

        #[cfg(feature = "fetch")]
        #[doc(cfg(feature = "fetch"))]
        pub async fn fetch(
            year: i32,
            month: Option<u32>,
            day: Option<u32>,
            response_format: Option<ResponseFormat>,
        ) -> anyhow::Result<Response> {
            use reqwest::get;

            Ok(Response::from_str(
                &get(url(year, month, day, response_format)?).await?.text().await?,
            )?)
        }

        #[cfg(feature = "test")]
        #[cfg(test)]
        mod test {
            use std::str::FromStr;

            use super::Response;

            #[tokio::test]
            async fn test() {
                let Response(r1) = response_from_str!(
                    r#"YYYY-MM-DD,RISE,TRAN.,SET
2022-01-01,07:03,12:27,17:51
2022-01-02,07:03,12:27,17:51
2022-01-03,07:03,12:28,17:52"#,
                );

                let Response(r2) = response_from_str!(
                    r#"2022-01-01,07:03,12:27,17:51
2022-01-02,07:03,12:27,17:51
2022-01-03,07:03,12:28,17:52"#,
                );

                let Response(r3) = response_from_str!(
                    r#"{
    "fields": ["YYYY-MM-DD", "RISE", "TRAN.", "SET"],
    "data":[
        ["2022-01-01", "07:03", "12:27", "17:51"],
        ["2022-01-02", "07:03", "12:27", "17:51"],
        ["2022-01-03", "07:03", "12:28", "17:52"]]}"#,
                );

                assert!(r1.len() == r2.len() && r2.len() == r3.len());
                assert!(r1[0].rise == r2[0].rise && r2[0].rise == r3[0].rise);
                assert!(r1[0].transit == r2[0].transit && r2[0].transit == r3[0].transit);
                assert!(r1[0].set == r2[0].set && r2[0].set == r3[0].set);

                #[cfg(feature = "fetch")]
                {
                    use super::fetch;

                    let Response(_) = fetch(2022, None, None, None).await.unwrap();
                }
            }
        }
    };
}

pub mod mrs;
pub mod srs;

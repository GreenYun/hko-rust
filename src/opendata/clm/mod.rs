// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

#[allow(unused_macros)]
macro_rules! response_from_str {
    ($s:expr $(,)?) => {{
        super::Response::from_str($s).unwrap()
    }};
}

macro_rules! impl_clm {
    ($i:ident) => {
        use std::str::FromStr;

        use serde::Deserialize;

        use crate::{
            error::{APIRequestError, DataError},
            opendata::{ResponseFormat, TempStation},
        };

        #[derive(Clone, Debug)]
        pub struct ResponseUnit {
            pub year: u32,
            pub month: u32,
            pub day: u32,
            pub temp: Option<f32>,

            /// Data is complete if completeness is `true`.
            pub completeness: bool,
        }

        /// Main response type.
        #[derive(Clone, Debug)]
        pub struct Response(pub Vec<ResponseUnit>);

        impl FromStr for Response {
            type Err = DataError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let raw = s.trim().as_bytes();

                Ok(Self(match raw.first().ok_or(DataError::EarlyEOF)? {
                    // JSON
                    b'{' => {
                        #[derive(Deserialize)]
                        struct JsonResponse {
                            data: Vec<Vec<String>>,
                        }

                        let JsonResponse { data } = serde_json::from_str(s)
                            .map_err(|e| DataError::SourceFormat(e.to_string()))?;

                        data.into_iter()
                            .filter_map(|v| {
                                let year = v.get(0)?.parse().ok()?;
                                let month = v.get(1)?.parse().ok()?;
                                let day = v.get(2)?.parse().ok()?;
                                let temp = v.get(3)?.parse().ok();
                                let completeness = match v.get(4)?.as_str() {
                                    "C" => true,
                                    _ => false,
                                };

                                Some(ResponseUnit {
                                    year,
                                    month,
                                    day,
                                    temp,
                                    completeness,
                                })
                            })
                            .collect()
                    }

                    // CSV
                    _ => {
                        #[derive(Deserialize)]
                        struct CsvResponse {
                            year: u32,
                            month: u32,
                            day: u32,
                            temp: String,
                            completeness: String,
                        }

                        let mut rdr = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .flexible(true)
                            .from_reader(raw);

                        rdr.records()
                            .filter_map(|r| {
                                let CsvResponse {
                                    year,
                                    month,
                                    day,
                                    temp,
                                    completeness,
                                } = r.ok()?.deserialize(None).ok()?;

                                Some(ResponseUnit {
                                    year,
                                    month,
                                    day,
                                    temp: temp.parse().ok(),
                                    completeness: completeness == "C",
                                })
                            })
                            .collect()
                    }
                }))
            }
        }

        const fn check_year(year: u32, station: TempStation) -> bool {
            match station {
                TempStation::CCH => year >= 1992,
                TempStation::CWB => year >= 2018,
                TempStation::HKA => year >= 1997,
                TempStation::HKO => year >= 1884 && !matches!(year, 1940..=1946),
                TempStation::HKP => year >= 2007,
                TempStation::HKS => year >= 1989,
                TempStation::HPV => year >= 2008,
                TempStation::JKB => year >= 1991,
                TempStation::KLT => year >= 2008,
                TempStation::KP => year >= 1992,
                TempStation::KSC => year >= 2008,
                TempStation::KTG => year >= 2009,
                TempStation::LFS => year >= 1985,
                TempStation::NGP => year >= 2003,
                TempStation::PEN => year >= 2004,
                TempStation::PLC => year >= 1993,
                TempStation::SE1 => year >= 2014,
                TempStation::SEK => year >= 1996,
                TempStation::SHA => year >= 1984,
                TempStation::SKG => year >= 1993,
                TempStation::SKW => year >= 2007,
                TempStation::SSH => year >= 2004,
                TempStation::SSP => year >= 2010,
                TempStation::STY => year >= 2009,
                TempStation::TC => year >= 1997,
                TempStation::TKL => year >= 1988,
                TempStation::TMS => year >= 1997,
                TempStation::TPO => year >= 1999,
                TempStation::TU1 => year >= 2007,
                TempStation::TW => year >= 2010,
                TempStation::TWN => year >= 2006,
                TempStation::TY1 => year >= 2010,
                TempStation::TYW => year >= 1995,
                TempStation::VP1 => year >= 2003,
                TempStation::WGL => year >= 1989,
                TempStation::WLP => year >= 2005,
                TempStation::WTS => year >= 2009,
                TempStation::YCT => year >= 2022,
                TempStation::YLP => year >= 2015,
            }
        }

        #[allow(clippy::missing_errors_doc)]
        pub fn url(
            station: TempStation,
            year: Option<u32>,
            month: Option<u32>,
            response_format: Option<ResponseFormat>,
        ) -> Result<String, APIRequestError> {
            Ok(format!(
                concat_url!($i, "&station={}{}{}{}"),
                station.clone(),
                if let Some(year) = year {
                    if !check_year(year, station.clone()) {
                        return Err(APIRequestError(format!(
                            "Year {} is not available for {}",
                            year, station
                        )));
                    }

                    format!("&year={}", year)
                } else {
                    String::new()
                },
                if let Some(month) = month {
                    format!("&month={}", month)
                } else {
                    String::new()
                },
                response_format
                    .map(|f| format!("&rformat={}", f))
                    .unwrap_or(String::new()),
            ))
        }

        #[cfg(feature = "fetch")]
        #[doc(cfg(feature = "fetch"))]
        pub async fn fetch(
            station: TempStation,
            year: Option<u32>,
            month: Option<u32>,
            response_format: Option<ResponseFormat>,
        ) -> anyhow::Result<Response> {
            use reqwest::get;

            Ok(Response::from_str(
                &get(url(station, year, month, response_format)?)
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

            #[tokio::test]
            async fn test() {
                // CSV with header
                let Response(r1) = response_from_str!(concat!(
                    "\u{FEFF}\u{22}\u{FEFF}",
                    r##"氣溫(攝氏度) - 天文台"
"Temperature (°C) at the Hong Kong Observatory"
年/Year,月/Month,日/Day,數值/Value,"數據完整性/data Completeness"
1884,3,1,***,
2021,11,29,23.9,#
2021,11,30,21.3,C
2021,12,1,,
"*** 沒有數據/unavailable"
"# 數據不完整/data incomplete"
"C 數據完整/data Complete""##,
                ));

                // JSON
                let Response(r2) = response_from_str!(
                    r##"{
    "type": [
        "氣溫(攝氏度) - 天文台",
        "Temperature (°C) at the Hong Kong Observatory"
    ],
    "fields": [
        "年\/Year", "月\/Month", "日\/Day", "數值\/Value", "數據完整性\/data Completeness"
    ],
    "data": [
        ["1884", "3", "1", "***", ""],
        ["2021", "11", "29", "23.9", "#"],
        ["2021", "11", "30", "21.3", "C"],
        ["2021", "12", "1", "", ""]
    ],
    "legend": [
        "*** 沒有數據\/unavailable",
        "# 數據不完整\/data incomplete",
        "C 數據完整\/data Complete"
    ]
}"##
                );

                assert!(r1.len() == r2.len() && r1.len() == 4);
                assert_eq!(r1[0].year, r2[0].year);
                assert_eq!(r1[1].month, r2[1].month);
                assert_eq!(r1[2].day, r2[2].day);
                assert_eq!(r1[2].temp, r2[2].temp);
                assert_eq!(r1[3].completeness, r2[3].completeness);

                #[cfg(feature = "fetch")]
                {
                    use super::fetch;
                    use crate::opendata::TempStation::CCH;

                    let Response(_) = fetch(CCH, None, None, None).await.unwrap();
                }
            }
        }
    };
}

pub mod clmmaxt;
pub mod clmmint;
pub mod clmtemp;

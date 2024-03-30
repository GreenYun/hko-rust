// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Provides weather and radiation level report.

use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use serde::Deserialize;
use serde_json::Value;

use super::WeatherStation;
use crate::{common::Lang, error::DataError, internal::hkt};

/// Data retrieved from a station.
#[derive(Clone, Debug)]
pub struct AreaData {
    /// Station name
    pub name: String,

    /// Average ambient gamma radiation dose rate (microsievert per hour)
    pub microsieverts: Option<f32>,

    /// Maximum air temperature (degree Celsius)
    pub max_temp: Option<f32>,

    /// Minimum air temperature (degree Celsius)
    pub min_temp: Option<f32>,

    /// Maximum air temperature (degree Celsius)
    pub readings_max_temp: Option<f32>,

    /// Minimum air temperature (degree Celsius)
    pub readings_min_temp: Option<f32>,

    /// Minimum grass temperature (degree Celsius)
    pub readings_min_grass_temp: Option<f32>,

    /// Maximun relative humidity (percentage)
    pub readings_max_rh: Option<f32>,

    /// Minimum relative humidity (percentage)
    pub readings_min_rh: Option<f32>,

    /// Rainfall (millimetre)
    pub readings_rainfall: Option<f32>,

    /// Average rainfall (millimetre)
    pub readings_average_rainfall: Option<f32>,

    /// Accumulated rainfall (millimetre)
    pub readings_accumulated_rainfall: Option<f32>,

    /// Maximum UV index
    pub readings_max_uv_index: Option<f32>,

    /// Mean UV index
    pub readings_mean_uv_index: Option<f32>,

    /// Duration of sunshine (hour)
    pub readings_sunshine: Option<f32>,
}

impl AreaData {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            microsieverts: None,
            max_temp: None,
            min_temp: None,
            readings_max_temp: None,
            readings_min_temp: None,
            readings_min_grass_temp: None,
            readings_max_rh: None,
            readings_min_rh: None,
            readings_rainfall: None,
            readings_average_rainfall: None,
            readings_accumulated_rainfall: None,
            readings_max_uv_index: None,
            readings_mean_uv_index: None,
            readings_sunshine: None,
        }
    }
}

impl Default for AreaData {
    fn default() -> Self {
        Self::new()
    }
}

/// Main response type.
#[derive(Clone, Debug)]
pub struct Response {
    /// Description of average ambient gamma radiation dose rate taken outdoors
    /// in Hong Kong
    pub hong_kong_desc: String,

    /// Notes
    pub note_desc: Vec<String>,

    /// Information Date
    pub report_time_info_date: NaiveDate,

    /// Bulletin date and time
    pub bulletin_date_time: DateTime<FixedOffset>,

    /// Area data
    pub area_data: Vec<AreaData>,
}

impl FromStr for Response {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct JsonResponse {
            hong_kong_desc: String,
            note_desc: String,
            note_desc1: String,
            note_desc2: String,
            note_desc3: String,
            report_time_info_date: String,
            bulletin_date: String,
            bulletin_time: String,

            #[serde(flatten)]
            extra_data: HashMap<String, Value>,
        }

        let JsonResponse {
            hong_kong_desc,
            note_desc,
            note_desc1,
            note_desc2,
            note_desc3,
            report_time_info_date,
            bulletin_date,
            bulletin_time,
            extra_data,
        } = serde_json::from_str(s).map_err(|e| DataError::SourceFormat(e.to_string()))?;

        let report_time_info_date = NaiveDate::parse_from_str(&report_time_info_date, "%Y%m%d")
            .map_err(|e| DataError::SourceFormat(e.to_string()))?;

        let bulletin_date_time = NaiveDateTime::parse_from_str(&(bulletin_date + &bulletin_time), "%Y%m%d%H%M")
            .map_err(|e| DataError::SourceFormat(e.to_string()))?;
        let bulletin_date_time = hkt()
            .from_local_datetime(&bulletin_date_time)
            .single()
            .ok_or_else(|| DataError::SourceFormat("Invalid time".to_owned()))?;

        let note_desc = vec![note_desc, note_desc1, note_desc2, note_desc3]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();

        let mut area_data = HashMap::new();

        for (key, val) in extra_data {
            const ATTRIBUTES: [&str; 15] = [
                "LocationName",
                "Microsieverts",
                "MaxTemp",
                "MinTemp",
                "ReadingsMaxTemp",
                "ReadingsMinTemp",
                "ReadingsMinGrassTemp",
                "ReadingsMaxRH",
                "ReadingsMinRH",
                "ReadingsRainfall",
                "ReadingsAvgRainfall",
                "ReadingsAccumRainfall",
                "ReadingsMaxUVIndex",
                "ReadingsMeanUVIndex",
                "ReadingsSunShine",
            ];

            if let Some(index) = ATTRIBUTES.iter().position(|&s| key.ends_with(s)) {
                let area_key = key.strip_suffix(ATTRIBUTES[index]).unwrap().to_owned();

                let data = area_data.entry(area_key).or_insert_with(AreaData::new);

                let val_to_f32 = || val.as_str().and_then(|s| s.parse().ok());

                match index {
                    0 => val.as_str().unwrap().clone_into(&mut data.name),
                    1 => data.microsieverts = val_to_f32(),
                    2 => data.max_temp = val_to_f32(),
                    3 => data.min_temp = val_to_f32(),
                    4 => data.readings_max_temp = val_to_f32(),
                    5 => data.readings_min_temp = val_to_f32(),
                    6 => data.readings_min_grass_temp = val_to_f32(),
                    7 => data.readings_max_rh = val_to_f32(),
                    8 => data.readings_min_rh = val_to_f32(),
                    9 => data.readings_rainfall = val_to_f32(),
                    10 => data.readings_average_rainfall = val_to_f32(),
                    11 => data.readings_accumulated_rainfall = val_to_f32(),
                    12 => data.readings_max_uv_index = val_to_f32(),
                    13 => data.readings_mean_uv_index = val_to_f32(),
                    14 => data.readings_sunshine = val_to_f32(),
                    _ => unreachable!(),
                };
            }
        }

        Ok(Self {
            hong_kong_desc,
            note_desc,
            report_time_info_date,
            bulletin_date_time,
            area_data: area_data.values().cloned().collect(),
        })
    }
}

#[must_use]
pub fn url(date: NaiveDate, lang: Option<Lang>, station: Option<WeatherStation>) -> String {
    format!(
        concat_url!(RYES, "&date={}{}{}"),
        date.format("%Y%m%d"),
        lang.map(|l| format!("&lang={l}")).unwrap_or_default(),
        station.map(|s| format!("&station={s}")).unwrap_or_default()
    )
}

#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch(date: NaiveDate, lang: Option<Lang>, station: Option<WeatherStation>) -> anyhow::Result<Response> {
    use reqwest::get;

    Ok(Response::from_str(&get(url(date, lang, station)).await?.text().await?)?)
}

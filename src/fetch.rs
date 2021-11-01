// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::Result;
use reqwest::get;
use serde::de::DeserializeOwned;

use crate::common::Lang;

pub trait API {
    const BASE: &'static str;
    const DATATYPE: &'static str;

    fn url(&self, lang: Lang) -> String {
        format!(
            "https://data.weather.gov.hk/weatherAPI/opendata/{}.php?dataType={}&lang={:?}",
            Self::BASE,
            Self::DATATYPE,
            lang
        )
    }
}

pub async fn fetch<T>(lang: Lang) -> Result<T>
where
    T: API + DeserializeOwned,
{
    Ok(serde_json::from_str(
        &get(&(format!(
            "https://data.weather.gov.hk/weatherAPI/opendata/{}.php?dataType={}&lang={:?}",
            T::BASE,
            T::DATATYPE,
            lang
        )))
        .await?
        .text()
        .await?,
    )?)
}

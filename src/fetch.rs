// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use crate::common::Lang;

/// API trait for all data types.
pub trait API {
    const BASE: &'static str;
    const DATATYPE: &'static str;

    /// Generates the URL for retrieving the data, in specified language.
    #[must_use]
    fn url(lang: Lang) -> String {
        format!(
            "https://data.weather.gov.hk/weatherAPI/opendata/{}.php?dataType={}&lang={}",
            Self::BASE,
            Self::DATATYPE,
            lang
        )
    }
}

macro_rules! impl_api {
    ($t:ty, $b:ident, $d:ident) => {
        impl crate::fetch::API for $t {
            const BASE: &'static str = stringify!($b);
            const DATATYPE: &'static str = stringify!($d);
        }
    };
}

pub(crate) use impl_api;

/// Helper function to fetch data from API.
#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch<T>(lang: Lang) -> anyhow::Result<T>
where
    T: API + serde::de::DeserializeOwned,
{
    use reqwest::get;

    Ok(serde_json::from_str(&get(T::url(lang)).await?.text().await?)?)
}

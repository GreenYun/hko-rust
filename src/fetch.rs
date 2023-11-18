// Copyright (c) 2021 - 2023 GreenYun Organization
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
            "https://data.weather.gov.hk/weatherAPI/opendata/{}.php?dataType={}&lang={lang}",
            Self::BASE,
            Self::DATATYPE,
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

#[allow(unused_imports)]
pub(crate) use impl_api;

/// Helper function to fetch data from API.
///
/// You may found connection error from [`reqwest`], because this crate has not
/// import any dependency feature providing TLS/HTTPS connection function.
/// Add your favorite TLS implementation to your `Cargo.toml` to fix this
/// problem. See the documentation of [`reqwest`] for more information.
#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
#[doc(cfg(feature = "fetch"))]
pub async fn fetch<T>(lang: Lang) -> anyhow::Result<T>
where
    T: API + serde::de::DeserializeOwned,
{
    use reqwest::get;

    Ok(serde_json::from_str(&get(T::url(lang)).await?.text().await?)?)
}

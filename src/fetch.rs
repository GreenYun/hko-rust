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

/// Helper trait to fetch data from API.
#[cfg(feature = "fetch")]
pub trait Fetch: Sized {
    /// Fetch function for API.
    #[allow(clippy::missing_errors_doc)]
    fn fetch(lang: Lang) -> impl std::future::Future<Output = anyhow::Result<Self>> + Send;

    /// Fetch function for API, with custom client.
    #[allow(clippy::missing_errors_doc)]
    fn fetch_with_client(
        lang: Lang,
        client: reqwest::Client,
    ) -> impl std::future::Future<Output = anyhow::Result<Self>> + Send;
}

#[cfg(feature = "fetch")]
impl<T> Fetch for T
where
    T: API + serde::de::DeserializeOwned,
{
    async fn fetch(lang: Lang) -> anyhow::Result<Self> {
        use reqwest::get;

        Ok(get(Self::url(lang)).await?.json().await?)
    }

    async fn fetch_with_client(lang: Lang, client: reqwest::Client) -> anyhow::Result<Self> {
        Ok(client.get(Self::url(lang)).send().await?.json().await?)
    }
}

/// Helper function to fetch data from API.
///
/// You may found connection error from [`reqwest`], because this crate has not
/// import any dependency feature providing TLS/HTTPS connection function.
/// Add your favorite TLS implementation to your `Cargo.toml` to fix this
/// problem. See the documentation of [`reqwest`] for more information.
#[allow(clippy::missing_errors_doc)]
#[cfg(feature = "fetch")]
pub async fn fetch<T>(lang: Lang) -> anyhow::Result<T>
where
    T: Fetch,
{
    T::fetch(lang).await
}

/// Helper function to fetch data from API with custom client.
///
/// You may found connection error from [`reqwest`], because this crate has not
/// import any dependency feature providing TLS/HTTPS connection function.
/// Add your favorite TLS implementation to your `Cargo.toml` to fix this
/// problem. See the documentation of [`reqwest`] for more information.
#[allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#[cfg(feature = "fetch")]
pub async fn fetch_with_client<T>(lang: Lang, client: reqwest::Client) -> anyhow::Result<T>
where
    T: Fetch,
{
    T::fetch_with_client(lang, client).await
}

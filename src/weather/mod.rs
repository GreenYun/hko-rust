// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Weather information
//!
//! Weather information contains such datasets:
//! - 9-day Weather Forecast ([`NineDay`])
//! - Current Weather Report ([`Current`])
//! - Local Weather Forecast  ([`Local`])
//! - Weather Warning Information ([`Info`])
//! - Weather Warning Summary ([`Summary`])
//! - Special Weather Tips ([`Tips`])
//!
//! Each of these data types have implementation of [`API`](crate::API).
//! `fetch` function fetches and parses response from the API and return the
//! data type if succeeded. `fetch` will not resolve any network problems, so
//! apps running under various network should fetch the JSON-based response by
//! itself. All these data types have implementations of
//! [`Deserialize`](serde::de::Deserialize) for parsing JSON data.
//!
//! To generate the API URL, simply call [`Type::url(lang)`](crate::API::url()).
//!
//! ## Example
//!
//! Parse the JSON data (already stored in `s` of `&str`) and get the dataset:
//! ```
//! use hko::weather::Local;
//!
//! let l : Local = serde_json::from_str(s)?;
//! ```
//!
//! Simply `fetch` and wait for the dataset written in English:
//! ```
//! use hko::weather::Current;
//!
//! let c : Current = fetch(Lang::en).await?;
//! ```

pub use self::{
    current::Current,
    local::Local,
    name::*,
    nine_day::NineDay,
    psr::*,
    tips::Tips,
    warning::{info::Info, summary::Summary},
};

pub mod current;
pub mod local;
mod name;
pub mod nine_day;
mod psr;
pub mod tips;
pub mod warning;

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

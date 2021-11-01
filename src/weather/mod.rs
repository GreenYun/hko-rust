// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
//! [`fetch`](crate::fetch()) function fetches and parses response from the API
//! and return the data type if succeeded. [`fetch`](crate::fetch()) will not
//! resolve any network problems, so apps running under various network should
//! fetch the JSON-based response by itself. All these data types have
//! implementations of [`Deserialize`](serde::de::Deserialize) for parsing JSON
//! data.
//!
//! To generate the API URL, simply call [`url(lang)`](crate::API::url()).
//!
//! ## Example
//!
//! Simply `fetch` and wait for the dataset written in English:
//! ```
//! use hko::weather::Current;
//!
//! let c : Current = fetch(Lang::en).await?;
//! ```
//!
//! Parse the JSON data (already stored in `s` of `&str`) and get the dataset:
//! ```
//! use hko::weather::Local;
//!
//! let l : Local = serde_json::from_str(s)?;
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

#[macro_use]
mod impl_api;

pub mod current;
pub mod local;
mod name;
pub mod nine_day;
mod psr;
pub mod tips;
pub mod warning;

mod test;

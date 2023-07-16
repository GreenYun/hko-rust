// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Earthquake information
//!
//! Earthquake information contains such datasets:
//! - Quick Earthquake Messages ([`Message`])
//! - Locally Felt Earth Tremor Report ([`FeltReport`])
//!
//! Each of these data types have implementation of [`API`](crate::API).
//! `fetch` function fetches and parses response from the API and return the
//! data type if succeeded. `fetch` will not resolve any network problems, so
//! apps running under various network should fetch the JSON-based response by
//! itself. All these data types have implementations of
//! [`Deserialize`](serde::de::Deserialize) for parsing JSON data.
//!
//! To generate the API URL, simply call [`url(lang)`](crate::API::url()).
//!
//! - **HTTP Request Method**: GET
//! - **Return Type**: JSON
//!
//! ## Example
//!
//! Parse the JSON data (already stored in `s` of `&str`) and get the dataset:
//!
//! ```
//! use hko::earthquake::FeltReport;
//!
//! # fn f(s: &str) -> anyhow::Result<FeltReport> {
//! let r : FeltReport = serde_json::from_str(s)?;
//! # Ok(r)
//! # }
//! ```
//!
//! Simply `fetch` and wait for the dataset written in English:
//!
//! ```
//! use hko::common::Lang;
//! use hko::earthquake::Message;
//! use hko::fetch;
//!
//! # async fn f() -> anyhow::Result<Message> {
//! let m : Message = fetch(Lang::EN).await?;
//! # Ok(m)
//! # }
//! ```

pub use self::{felt_report::FeltReport, message::Message};

pub mod felt_report;
pub mod message;

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

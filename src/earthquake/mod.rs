// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
//! ## Example
//!
//! Parse the JSON data (already stored in `s` of `&str`) and get the dataset:
//! ```
//! use hko::earthquake::FeltReport;
//!
//! let l : FeltReport = serde_json::from_str(s)?;
//! ```
//!
//! Simply `fetch` and wait for the dataset written in English:
//! ```
//! use hko::earthquake::Message;
//!
//! let c : Message = fetch(Lang::en).await?;
//! ```

pub use self::{felt_report::FeltReport, message::Message};

pub mod felt_report;
pub mod message;

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

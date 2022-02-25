// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

#![feature(doc_cfg)]

//! This is a library for users to access Hong Kong Observatory Open Data API.
//!
//! Hong Kong Observatory Open Data API provides weather, earthquake
//! information, and other open data including climate information.
//!
//! This library provides interfaces to parse JSON-based response from the API,
//! returning Rust data types. To use them, refer the documentation of Mods,
//! Structs, Enums, Functions, as well as Macros.
//!
//! More details of the API, refer
//! [hko.gov.hk](https://www.weather.gov.hk/en/abouthko/opendata_intro.htm) (or
//! [中文](https://www.weather.gov.hk/tc/abouthko/opendata_intro.htm)).

#[cfg(feature = "fetch")]
pub use self::fetch::fetch;
pub use self::fetch::API;

pub mod common;
pub mod earthquake;
pub mod error;
mod fetch;
mod internal;
#[macro_use]
mod r#macro;
#[cfg(feature = "opendata")]
#[doc(cfg(feature = "opendata"))]
pub mod opendata;
pub mod weather;

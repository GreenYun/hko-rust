// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(unused_macros)]
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

#[macro_use]
mod r#macro;
mod fetch;
mod internal;

pub mod common;

#[doc(hidden)]
pub mod error;

#[cfg(feature = "earthquake")]
#[doc(cfg(feature = "earthquake"))]
pub mod earthquake;

#[cfg(feature = "hourly_rainfall")]
#[doc(cfg(feature = "hourly_rainfall"))]
pub mod hourly_rainfall;

#[cfg(feature = "lunardate")]
#[doc(cfg(feature = "lunardate"))]
pub mod lunardate;

#[cfg(feature = "opendata")]
#[doc(cfg(feature = "opendata"))]
pub mod opendata;

#[cfg(feature = "weather")]
#[doc(cfg(feature = "weather"))]
pub mod weather;

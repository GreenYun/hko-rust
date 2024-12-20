// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Weather warnings.
//!
//! Weather warnings contains two types of datasets: informational data and
//! summary.

pub use self::code::*;

mod code;
pub mod info;
pub mod summary;

#[cfg(feature = "test")]
#[cfg(test)]
mod test;

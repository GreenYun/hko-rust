// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Weather warnings.
//!
//! Weather warnings contains two types of datasets: informational data and
//! summary.

pub use self::code::*;

mod code;
pub mod info;
pub mod summary;

mod test;

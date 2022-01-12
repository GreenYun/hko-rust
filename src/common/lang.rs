// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! This module provides [`Lang`] for enumerating language codes.

use strum_macros::Display;

/// The `Lang` type has three values:
/// `EN` for English,
/// `SC` for Simplified Chinese, and
/// `TC` for Traditional Chinese.
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Lang {
    EN,
    SC,
    TC,
}

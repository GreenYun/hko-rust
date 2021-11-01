// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! This module provides [`Lang`] for enumerating language codes.

/// The `Lang` type has three values:
/// `en` for English,
/// `sc` for Simplified Chinese, and
/// `tc` for Traditional Chinese.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Lang {
    en,
    sc,
    tc,
}

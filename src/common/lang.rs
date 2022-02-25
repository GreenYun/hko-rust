// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

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

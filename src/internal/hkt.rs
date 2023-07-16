// Copyright (c) 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use chrono::FixedOffset;

pub const HKT: Option<FixedOffset> = FixedOffset::east_opt(8 * 3600);

pub fn hkt() -> FixedOffset {
    HKT.unwrap()
}

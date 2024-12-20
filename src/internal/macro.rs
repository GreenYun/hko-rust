// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

macro_rules! enum_lang_matches {
    ($case:expr, $lang:expr,
        $($val:path => $en:expr, $tc:expr, $sc:expr),+ $(,)?) => {
        match $case {
            $($val => match $lang {
                crate::common::Lang::EN => $en,
                crate::common::Lang::TC => $tc,
                crate::common::Lang::SC => $sc,
            }),
            +
        }
    };
}

pub(crate) use enum_lang_matches;

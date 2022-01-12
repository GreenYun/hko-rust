// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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

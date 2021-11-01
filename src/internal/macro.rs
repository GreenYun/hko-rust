// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

macro_rules! enum_lang_matches {
    ($case:expr, $lang:expr,
        $($val:path => $en:expr, $tc:expr, $sc:expr),+ $(,)?) => {
        match $case {
            $($val => match $lang {
                crate::common::Lang::en => $en,
                crate::common::Lang::tc => $tc,
                crate::common::Lang::sc => $sc,
            }),
            +
        }
    };
}

pub(crate) use enum_lang_matches;

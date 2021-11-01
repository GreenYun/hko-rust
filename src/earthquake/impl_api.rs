// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

macro_rules! impl_api {
    ($t:ty, $d:ident) => {
        impl crate::fetch::API for $t {
            const BASE: &'static str = "earthquake";
            const DATATYPE: &'static str = stringify!($d);
        }
    };
}

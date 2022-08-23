// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

/// Generates the URI of specified weather icon, usually an HTTPS link.
#[macro_export]
macro_rules! icon_uri {
    ($i:expr) => {
        format!(
            "https://www.hko.gov.hk/images/HKOWxIconOutline/pic{:?}.png",
            $i
        )
    };
}

/// Generates the URI of specified PSR icon, usually an HTTPS link.
#[macro_export]
macro_rules! psr_icon_uri {
    ($psr:expr) => {
        format!(
            "https://www.hko.gov.hk/common/images/PSR{:?}_50_light.png",
            $psr
        )
    };
}

macro_rules! impl_display_traits_internal {
    ($t:ty; $($tr:ident, $lang:ident, $f:ident, $s:ident, $($doc_lang:ident)+);+ $(;)?) => {
        $(
            #[doc = concat!("Formats the ", stringify!($($doc_lang)+), ".")]
            ///
            /// See [formatting rules](impl_display_traits).
            ///
            /// ## Example
            ///
            /// ```
            #[doc = concat!("let ", stringify!($s), r#" = format!("{:"#, stringify!($f), r#"}", val);"#)]
            /// ```
            impl ::std::fmt::$tr for $t {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    use crate::common::{EnumNameDesc, Lang};

                    use ::std::fmt::Display;

                    self.$s(Lang::$lang).fmt(f)
                }
            }
        )
        +
    };
}

/// Implements formatters for multi-language outputs.
///
/// `$t` must be a type that implements [`EnumNameDesc`](crate::common::EnumNameDesc)
/// trait.
///
/// Several formatter [types](std::fmt#formatting-traits) are redefined for
/// multi-language outputs:
///
/// | Formatter | Content     | Language            |
/// | :-------: | ----------- | ------------------- |
/// | `b`       | name        | simplified Chinese  |
/// | `p`       | description | simplified Chinese  |
/// | `o`       | name        | traditional Chinese |
/// | `x`       | description | traditional Chinese |
/// | `e`       | name        | English             |
/// | `E`       | description | English             |
#[macro_export]
macro_rules! impl_display_traits {
    ($t:ty) => {
        impl_display_traits_internal! {
            $t;
            Binary, SC, b, name, name in simplified Chinese;
            Pointer, SC, p, desc, description in simplified Chinese;
            Octal, TC, o, name, name in traditional Chinese;
            LowerHex, TC, x, desc, description in traditional Chinese;
            LowerExp, EN, e, name, name in English;
            UpperExp, EN, E, desc, description in English;
        }
    };
}

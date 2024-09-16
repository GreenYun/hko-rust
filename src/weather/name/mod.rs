// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use serde_repr::Deserialize_repr;

/// Weather status names.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum Name {
    Sunny                       = 50,
    SunnyPeriods                = 51,
    SunnyIntervals              = 52,
    SunnyPeriodsWithAFewShowers = 53,
    SunnyIntervalsWithShowers   = 54,

    Cloudy                      = 60,
    Overcast                    = 61,
    LightRain                   = 62,
    Rain                        = 63,
    HeavyRain                   = 64,
    Thunderstorms               = 65,

    Fine0                       = 70,
    Fine1                       = 71,
    Fine2                       = 72,
    Fine3                       = 73,
    Fine4                       = 74,
    Fine5                       = 75,
    MainlyCloudy                = 76,
    MainlyFine                  = 77,

    Windy                       = 80,
    Dry                         = 81,
    Humid                       = 82,
    Fog                         = 83,
    Mist                        = 84,
    Haze                        = 85,

    Hot                         = 90,
    Warm                        = 91,
    Cool                        = 92,
    Cold                        = 93,
}

mod string;

impl_display_traits!(Name);

impl Name {
    /// Generates the URI of specified weather icon, usually an HTTPS link.
    #[inline]
    #[must_use]
    pub fn icon_uri(&self) -> String {
        format!(
            "https://www.hko.gov.hk/images/HKOWxIconOutline/pic{:?}.png",
            (*self as u32)
        )
    }
}

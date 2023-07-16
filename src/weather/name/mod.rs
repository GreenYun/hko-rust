// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use num_derive::{FromPrimitive, ToPrimitive};

/// Weather status names.
#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, PartialEq, ToPrimitive)]
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

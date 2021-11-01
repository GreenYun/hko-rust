// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error, fmt};

/// Weather status names.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Name {
    Sunny = 50,
    SunnyPeriods = 51,
    SunnyIntervals = 52,
    SunnyPeriodsWithAFewShowers = 53,
    SunnyIntervalsWithShowers = 54,

    Cloudy = 60,
    Overcast = 61,
    LightRain = 62,
    Rain = 63,
    HeavyRain = 64,
    Thunderstorms = 65,

    Fine0 = 70,
    Fine1 = 71,
    Fine2 = 72,
    Fine3 = 73,
    Fine4 = 74,
    Fine5 = 75,
    MainlyCloudy = 76,
    MainlyFine = 77,

    Windy = 80,
    Dry = 81,
    Humid = 82,
    Fog = 83,
    Mist = 84,
    Haze = 85,

    Hot = 90,
    Warm = 91,
    Cool = 92,
    Cold = 93,
}

mod string;

impl_display_traits!(Name);

/// The error type returned when a checked integral type conversion fails.
#[derive(Debug, Clone, Copy)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "out of range integral type conversion attempted".fmt(f)
    }
}

impl error::Error for TryFromIntError {}

impl TryFrom<i32> for Name {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Name::*;
        match value {
            50 => Ok(Sunny),
            51 => Ok(SunnyPeriods),
            52 => Ok(SunnyIntervals),
            53 => Ok(SunnyPeriodsWithAFewShowers),
            54 => Ok(SunnyIntervalsWithShowers),

            60 => Ok(Cloudy),
            61 => Ok(Overcast),
            62 => Ok(LightRain),
            63 => Ok(Rain),
            64 => Ok(HeavyRain),
            65 => Ok(Thunderstorms),

            70 => Ok(Fine0),
            71 => Ok(Fine1),
            72 => Ok(Fine2),
            73 => Ok(Fine3),
            74 => Ok(Fine4),
            75 => Ok(Fine5),
            76 => Ok(MainlyCloudy),
            77 => Ok(MainlyFine),

            80 => Ok(Windy),
            81 => Ok(Dry),
            82 => Ok(Humid),
            83 => Ok(Fog),
            84 => Ok(Mist),
            85 => Ok(Haze),

            90 => Ok(Hot),
            91 => Ok(Warm),
            92 => Ok(Cool),
            93 => Ok(Cold),
            _ => Err(TryFromIntError),
        }
    }
}

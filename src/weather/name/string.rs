// Copyright (c) 2021 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use super::Name;
use crate::{
    common::{EnumNameDesc, Lang},
    internal::enum_lang_matches,
};

impl EnumNameDesc for Name {
    #[allow(clippy::cognitive_complexity)]
    fn name(&self, lang: Lang) -> String {
        enum_lang_matches! {
                self, lang,
                Self::Sunny                       => "Sunny", "陽光充沛", "阳光充沛",
                Self::SunnyPeriods                => "Sunny Periods", "間有陽光", "间有阳光",
                Self::SunnyIntervals              => "Sunny Intervals", "短暫陽光", "短暂阳光",
                Self::SunnyPeriodsWithAFewShowers => "Sunny Periods with A Few Showers", "間有陽光幾陣驟雨", "间有阳光几阵骤雨",
                Self::SunnyIntervalsWithShowers   => "Sunny Intervals with Showers", "短暫陽光有驟雨", "短暂阳光有骤雨",

                Self::Cloudy        => "Cloudy", "多雲", "多云",
                Self::Overcast      => "Overcast", "密雲", "密云",
                Self::LightRain     => "Light Rain", "微雨", "微雨",
                Self::Rain          => "Rain", "雨", "雨",
                Self::HeavyRain     => "Heavy Rain", "大雨", "大雨",
                Self::Thunderstorms => "Thunderstorms", "雷暴", "雷暴",

                Self::Fine0        => "Fine", "天色良好", "天色良好",
                Self::Fine1        => "Fine", "天色良好", "天色良好",
                Self::Fine2        => "Fine", "天色良好", "天色良好",
                Self::Fine3        => "Fine", "天色良好", "天色良好",
                Self::Fine4        => "Fine", "天色良好", "天色良好",
                Self::Fine5        => "Fine", "天色良好", "天色良好",
                Self::MainlyCloudy => "Mainly Cloudy", "大致多雲", "大致多云",
                Self::MainlyFine   => "Mainly Fine", "天色大致良好", "天色大致良好",

                Self::Windy => "Windy", "大風", "大风",
                Self::Dry   => "Dry", "乾燥", "干燥",
                Self::Humid => "Humid", "潮濕", "潮湿",
                Self::Fog   => "Fog", "霧", "雾",
                Self::Mist  => "Mist", "薄霧", "薄雾",
                Self::Haze  => "Haze", "煙霞", "烟霞",

                Self::Hot   => "Hot", "熱", "热",
                Self::Warm  => "Warm", "暖", "暖",
                Self::Cool  => "Cool", "涼", "凉",
                Self::Cold  => "Cold", "冷", "冷",
            }.to_owned()
    }

    fn desc(&self, lang: Lang) -> String {
        match self {
            Self::Fine0 => match lang {
                Lang::EN => "Fine ( use only in night-time on 1st of the Lunar Month )",
                Lang::TC => "天色良好(只在農曆第一日晚間使用)",
                Lang::SC => "天色良好(只在农曆第一日晚间使用)",
            }
            .to_owned(),
            Self::Fine1 => match lang {
                Lang::EN => "Fine ( use only in night-time on 2nd to 6th of the Lunar Month )",
                Lang::TC => "天色良好(只在農曆第二日至第六日晚間使用)",
                Lang::SC => "天色良好(只在农曆第二日至第六日晚间使用)",
            }
            .to_owned(),
            Self::Fine2 => match lang {
                Lang::EN => "Fine ( use only in night-time during 7th to 13th of Lunar Month )",
                Lang::TC => "天色良好(只在農曆第七日至第十三日晚間使用)",
                Lang::SC => "天色良好(只在农曆第七日至第十三日晚间使用)",
            }
            .to_owned(),
            Self::Fine3 => match lang {
                Lang::EN => "Fine ( use only in night-time during 14th to 17th of Lunar Month )",
                Lang::TC => "天色良好(只在農曆第十四日至第十七日晚間使用)",
                Lang::SC => "天色良好(只在农曆第十四日至第十七日晚间使用)",
            }
            .to_owned(),
            Self::Fine4 => match lang {
                Lang::EN => "Fine ( use only in night-time during 18th to 24th of Lunar Month )",
                Lang::TC => "天色良好(只在農曆第十八日至第二十四日晚間使用)",
                Lang::SC => "天色良好(只在农曆第十八日至第二十四日晚间使用)",
            }
            .to_owned(),
            Self::Fine5 => match lang {
                Lang::EN => "Mainly Cloudy ( use only in night-time during 25th to 30th of Lunar Month )",
                Lang::TC => "天色良好(只在農曆第二十五日至第三十日晚間使用)",
                Lang::SC => "天色良好(只在农曆第二十五日至第三十日晚间使用)",
            }
            .to_owned(),
            Self::MainlyCloudy => match lang {
                Lang::EN => "Mainly Cloudy ( use only in night-time )",
                Lang::TC => "大致多雲(只在晚間使用)",
                Lang::SC => "大致多云(只在晚间使用)",
            }
            .to_owned(),
            Self::MainlyFine => match lang {
                Lang::EN => "Mainly Fine ( use only in night-time )",
                Lang::TC => "天色大致良好(只在晚間使用)",
                Lang::SC => "天色大致良好(只在晚间使用)",
            }
            .to_owned(),
            _ => self.name(lang),
        }
    }
}

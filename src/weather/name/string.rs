// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use super::Name;
use crate::{
    common::{EnumNameDesc, Lang},
    internal::enum_lang_matches,
};

impl EnumNameDesc for Name {
    fn name(&self, lang: Lang) -> String {
        format!(
            "{}",
            enum_lang_matches! {
                self, lang,
                Name::Sunny                       => "Sunny", "陽光充沛", "阳光充沛",
                Name::SunnyPeriods                => "Sunny Periods", "間有陽光", "间有阳光",
                Name::SunnyIntervals              => "Sunny Intervals", "短暫陽光", "短暂阳光",
                Name::SunnyPeriodsWithAFewShowers => "Sunny Periods with A Few Showers", "間有陽光幾陣驟雨", "间有阳光几阵骤雨",
                Name::SunnyIntervalsWithShowers   => "Sunny Intervals with Showers", "短暫陽光有驟雨", "短暂阳光有骤雨",

                Name::Cloudy        => "Cloudy", "多雲", "多云",
                Name::Overcast      => "Overcast", "密雲", "密云",
                Name::LightRain     => "Light Rain", "微雨", "微雨",
                Name::Rain          => "Rain", "雨", "雨",
                Name::HeavyRain     => "Heavy Rain", "大雨", "大雨",
                Name::Thunderstorms => "Thunderstorms", "雷暴", "雷暴",

                Name::Fine0        => "Fine", "天色良好", "天色良好",
                Name::Fine1        => "Fine", "天色良好", "天色良好",
                Name::Fine2        => "Fine", "天色良好", "天色良好",
                Name::Fine3        => "Fine", "天色良好", "天色良好",
                Name::Fine4        => "Fine", "天色良好", "天色良好",
                Name::Fine5        => "Fine", "天色良好", "天色良好",
                Name::MainlyCloudy => "Mainly Cloudy", "大致多雲", "大致多云",
                Name::MainlyFine   => "Mainly Fine", "天色大致良好", "天色大致良好",

                Name::Windy => "Windy", "大風", "大风",
                Name::Dry   => "Dry", "乾燥", "干燥",
                Name::Humid => "Humid", "潮濕", "潮湿",
                Name::Fog   => "Fog", "霧", "雾",
                Name::Mist  => "Mist", "薄霧", "薄雾",
                Name::Haze  => "Haze", "煙霞", "烟霞",

                Name::Hot   => "Hot", "熱", "热",
                Name::Warm  => "Warm", "暖", "暖",
                Name::Cool  => "Cool", "涼", "凉",
                Name::Cold  => "Cold", "冷", "冷",
            }
        )
    }
    fn desc(&self, lang: Lang) -> String {
        match self {
            Name::Fine0 => format!(
                "{}",
                match lang {
                    Lang::EN => "Fine ( use only in night-time on 1st of the Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第一日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第一日晚间使用)",
                },
            ),
            Name::Fine1 => format!(
                "{}",
                match lang {
                    Lang::EN => "Fine ( use only in night-time on 2nd to 6th of the Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第二日至第六日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第二日至第六日晚间使用)",
                },
            ),
            Name::Fine2 => format!(
                "{}",
                match lang {
                    Lang::EN => "Fine ( use only in night-time during 7th to 13th of Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第七日至第十三日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第七日至第十三日晚间使用)",
                },
            ),
            Name::Fine3 => format!(
                "{}",
                match lang {
                    Lang::EN => "Fine ( use only in night-time during 14th to 17th of Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第十四日至第十七日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第十四日至第十七日晚间使用)",
                },
            ),
            Name::Fine4 => format!(
                "{}",
                match lang {
                    Lang::EN => "Fine ( use only in night-time during 18th to 24th of Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第十八日至第二十四日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第十八日至第二十四日晚间使用)",
                },
            ),
            Name::Fine5 => format!(
                "{}",
                match lang {
                    Lang::EN => "Mainly Cloudy ( use only in night-time during 25th to 30th of Lunar Month )",
                    Lang::TC => "天色良好(只在農曆第二十五日至第三十日晚間使用)",
                    Lang::SC => "天色良好(只在农曆第二十五日至第三十日晚间使用)",
                },
            ),
            Name::MainlyCloudy => format!(
                "{}",
                match lang {
                    Lang::EN => "Mainly Cloudy ( use only in night-time )",
                    Lang::TC => "大致多雲(只在晚間使用)",
                    Lang::SC => "大致多云(只在晚间使用)",
                },
            ),
            Name::MainlyFine => format!(
                "{}",
                match lang {
                    Lang::EN => "Mainly Fine ( use only in night-time )",
                    Lang::TC => "天色大致良好(只在晚間使用)",
                    Lang::SC => "天色大致良好(只在晚间使用)",
                },
            ),
            _ => self.name(lang),
        }
    }
}

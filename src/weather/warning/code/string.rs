// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::{Code, WarningStatementCode, WarningSubtypeCode};
use crate::{
    common::{EnumNameDesc, Lang},
    internal::enum_lang_matches,
};

impl EnumNameDesc for WarningStatementCode {
    fn name(&self, lang: Lang) -> String {
        format!(
            "{}",
            enum_lang_matches! {
                self, lang,
                WarningStatementCode::WFIRE =>
                    "Fire Danger Warning",
                    "火災危險警告",
                    "火灾危险警告",
                WarningStatementCode::WFROST =>
                    "Frost Warning",
                    "霜凍警告",
                    "霜冻警告",
                WarningStatementCode::WHOT =>
                    "Hot Weather Warning",
                    "酷熱天氣警告",
                    "酷热天气警告",
                WarningStatementCode::WCOLD =>
                    "Cold Weather Warning",
                    "寒冷天氣警告",
                    "寒冷天气警告",
                WarningStatementCode::WMSGNL =>
                    "Strong Monsoon Signal",
                    "強烈季候風信號",
                    "强烈季候风信号",
                WarningStatementCode::WTCPRE8 =>
                    "Pre-no.8 Special Announcement",
                    "預警八號熱帶氣旋警告信號之特別報告",
                    "预警八号热带气旋警告信号之特别报告",
                WarningStatementCode::WRAIN =>
                    "Rainstorm Warning Signal",
                    "暴雨警告信號",
                    "暴雨警告信号",
                WarningStatementCode::WFNTSA =>
                    "Special Announcement on Flooding in the northern New Territories",
                    "新界北部水浸特別報告",
                    "新界北部水浸特别报告",
                WarningStatementCode::WL =>
                    "Landslip Warning",
                    "山泥傾瀉警告",
                    "山泥倾泻警告",
                WarningStatementCode::WTCSGNL =>
                    "Tropical Cyclone Warning Signal",
                    "熱帶氣旋警告信號",
                    "热带气旋警告信号",
                WarningStatementCode::WTMW =>
                    "Tsunami Warning",
                    "海嘯警告",
                    "海啸警告",
                WarningStatementCode::WTS =>
                    "Thunderstorm Warning",
                    "雷暴警告",
                    "雷暴警告",
            }
        )
    }

    fn desc(&self, lang: Lang) -> String {
        self.name(lang)
    }
}

impl EnumNameDesc for WarningSubtypeCode {
    fn name(&self, lang: Lang) -> String {
        format!(
            "{}",
            enum_lang_matches! {
                self, lang,
                WarningSubtypeCode::WFIREY =>
                    "Yellow Fire Danger Warning",
                    "黃色火災危險警告",
                    "黄色火灾危险警告",
                WarningSubtypeCode::WFIRER =>
                    "Red Fire Danger Warning",
                    "紅色火災危險警告",
                    "红色火灾危险警告",
                WarningSubtypeCode::WRAINA =>
                    "Amber Rainstorm Warning Signal",
                    "黃色暴雨警告信號",
                    "黄色暴雨警告信号",
                WarningSubtypeCode::WRAINR =>
                    "Red Rainstorm Warning Signal",
                    "紅色暴雨警告信號",
                    "红色暴雨警告信号",
                WarningSubtypeCode::WRAINB =>
                    "Black Rainstorm Warning Signal",
                    "黑色暴雨警告信號",
                    "黑色暴雨警告信号",
                WarningSubtypeCode::TC1 =>
                    "Standby Signal No. 1",
                    "一號戒備信號",
                    "一号戒备信号",
                WarningSubtypeCode::TC3 =>
                    "Strong Wind Signal No. 3",
                    "三號強風信號",
                    "三号强风信号",
                WarningSubtypeCode::TC8NE =>
                    "No. 8 Northeast Gale or Storm Signal",
                    "八號東北烈風或暴風信號",
                    "八号东北烈风或暴风信号",
                WarningSubtypeCode::TC8SE =>
                    "No. 8 Southeast Gale or Storm Signal",
                    "八號東南烈風或暴風信號",
                    "八号东南烈风或暴风信号",
                WarningSubtypeCode::TC8SW =>
                    "No. 8 Southwest Gale or Storm Signal",
                    "八號西南烈風或暴風信號",
                    "八号西南烈风或暴风信号",
                WarningSubtypeCode::TC8NW =>
                    "No. 8 Northwest Gale or Storm Signal",
                    "八號西北烈風或暴風信號",
                    "八号西北烈风或暴风信号",
                WarningSubtypeCode::TC9 =>
                    "Increasing Gale or Storm Signal No. 9",
                    "九號烈風或暴風風力增強信號",
                    "九号烈风或暴风风力增强信号",
                WarningSubtypeCode::TC10 =>
                    "Hurricane Signal No. 10",
                    "十號颶風信號",
                    "十号飓风信号",
            }
        )
    }

    fn desc(&self, lang: Lang) -> String {
        self.name(lang)
    }
}

impl EnumNameDesc for Code {
    fn name(&self, lang: Lang) -> String {
        match self {
            Code::WarningStatement(w) => w.name(lang),
            Code::WarningSubType(w) => w.name(lang),
        }
    }

    fn desc(&self, lang: Lang) -> String {
        match self {
            Code::WarningStatement(w) => w.desc(lang),
            Code::WarningSubType(w) => w.desc(lang),
        }
    }
}

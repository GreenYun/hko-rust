// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use super::PSR;
use crate::{
    common::{EnumNameDesc, Lang},
    internal::enum_lang_matches,
};

impl EnumNameDesc for PSR {
    fn name(&self, lang: Lang) -> String {
        enum_lang_matches! {
            self, lang,
            PSR::High       => "High",        "高",   "高",
            PSR::MediumHigh => "Medium High", "中高", "中高",
            PSR::Medium     => "Medium",      "中",   "中",
            PSR::MediumLow  => "Medium Low",  "中低", "中低",
            PSR::Low        => "Low",         "低",   "低",
        }
        .to_owned()
    }

    fn desc(&self, lang: Lang) -> String {
        enum_lang_matches! {
                self, lang,
                PSR::High =>
                    r#"For every 100 forecasts with a "high" probability, there are about 70 times or more with an average accumulated rainfall of 10 mm or above in actual observation."#,
                    r"每100次概率為「高」的預測中，實際上約有70次或以上平均累積雨量達到10毫米或以上。",
                    r"每100次概率为「高」的预测中，实际上约有70次或以上平均累积雨量达到10毫米或以上。",
                PSR::MediumHigh =>
                    r#"For every 100 forecasts with a "medium high" probability, there are about 55 to 69 times with an average accumulated rainfall of 10 mm or above in actual observation."#,
                    r"每100次概率為「中高」的預測中，實際上約有55至69次平均累積雨量達到10毫米或以上。",
                    r"每100次概率为「中高」的预测中，实际上约有55至69次平均累积雨量达到10毫米或以上。",
                PSR::Medium =>
                    r#"For every 100 forecasts with a "medium" probability, there are about 45 to 54 times with an average accumulated rainfall of 10 mm or above in actual observation."#,
                    r"每100次概率為「中」的預測中，實際上約有45至54次平均累積雨量達到10毫米或以上。",
                    r"每100次概率为「中」的预测中，实际上约有45至54次平均累积雨量达到10毫米或以上。",
                PSR::MediumLow =>
                    r#"For every 100 forecasts with a "medium low" probability, there are about 30 to 44 times with an average accumulated rainfall of 10 mm or above in actual observation."#,
                    r"每100次概率為「中低」的預測中，實際上約有30至44次平均累積雨量達到10毫米或以上。",
                    r"每100次概率为「中低」的预测中，实际上约有30至44次平均累积雨量达到10毫米或以上。",
                PSR::Low =>
                    r#"For every 100 forecasts with a "low" probability, there are about less than 30 times with an average accumulated rainfall of 10 mm or above in actual observation."#,
                    r"每100次概率為「低」的預測中，實際上約有少於30次平均累積雨量達到10毫米或以上。",
                    r"每100次概率为「低」的预测中，实际上约有少于30次平均累积雨量达到10毫米或以上。",
            }
        .to_owned()
    }
}

// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[tokio::test]
async fn current_test() {
    use num_traits::FromPrimitive;

    use crate::weather::{current::Current, Name};

    let test_input = {
        r#"{
    "lightning": {
        "data": [
            {
                "place": "大嶼山",
                "occur": "true"
            }
        ],
        "startTime": "2021-10-07T18:45:00+08:00",
        "endTime": "2021-10-07T19:45:00+08:00"
    },
    "rainfall": {
        "data": [
            {
                "unit": "mm",
                "place": "中西區",
                "max": 0,
                "main": "FALSE"
            },
            {
                "unit": "mm",
                "place": "東區",
                "max": 0,
                "main": "FALSE"
            }
        ],
        "startTime": "2021-09-28T14:45:00+08:00",
        "endTime": "2021-09-28T15:45:00+08:00"
    },
    "warningMessage": [
        "酷熱天氣警告現正生效，市民應慎防中暑，多補充水分。"
    ],
    "icon": [
        51
    ],
    "iconUpdateTime": "2021-09-28T06:00:00+08:00",
    "uvindex": {
        "data": [
            {
                "place": "京士柏",
                "value": 2,
                "desc": "低"
            }
        ],
        "recordDesc": "過去一小時"
    },
    "updateTime": "2021-09-28T16:02:00+08:00",
    "temperature": {
        "data": [
            {
                "place": "京士柏",
                "value": 31,
                "unit": "C"
            },
            {
                "place": "香港天文台",
                "value": 32,
                "unit": "C"
            }
        ],
        "recordTime": "2021-09-28T16:00:00+08:00"
    },
    "tcmessage": "",
    "mintempFrom00To09": "",
    "rainfallFrom00To12": "",
    "rainfallLastMonth": "",
    "rainfallJanuaryToLastMonth": "",
    "humidity": {
        "recordTime": "2021-09-28T16:00:00+08:00",
        "data": [
            {
                "unit": "percent",
                "value": 71,
                "place": "香港天文台"
            }
        ]
    }
}"#
    };

    let current: Current = serde_json::from_str(test_input).unwrap();
    println!("{:?}", current);
    assert_eq!(current.temperature.data[0].value, 31.);
    println!("{:o}", Name::from_i32(current.icon.icon[0]).unwrap());

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let current: Current = fetch(Lang::EN).await.unwrap();
        println!("{:?}", current);
    }
}

#[tokio::test]
async fn local_test() {
    use crate::weather::local::Local;

    let test_input = {
        r#"{
    "generalSituation": "廣東沿岸風勢微弱。此外，驟雨正影響該區。本港方面，今早部分地區錄得超過10毫米雨量，而西貢的雨量更超過30毫米。",
    "tcInfo": "在正午十二時，颱風燦都集結在上海以東約230公里，預料移動緩慢，在上海以東海域徘徊。",
    "fireDangerWarning": "",
    "forecastPeriod": "本港地區下午及今晚天氣預測",
    "forecastDesc": "大致多雲，間中有驟雨及雷暴，初時局部地區雨勢頗大。吹微風。",
    "outlook": "未來兩三日間中有驟雨。週末期間部分時間有陽光。",
    "updateTime": "2021-09-14T12:45:00+08:00"
}"#
    };

    let local: Local = serde_json::from_str(test_input).unwrap();
    println!("{:?}", local);

    assert!(local.general_situation.len() > 0);

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let local: Local = fetch(Lang::EN).await.unwrap();
        println!("{:?}", local);
    }
}

#[tokio::test]
async fn nineday_test() {
    use crate::weather::{nine_day::NineDay, psr::PSR};

    let test_input = {
        r#"{
        "generalSituation": "華南沿岸未來兩三日風勢微弱，而一道雨帶會持續影響該區。隨著高空反氣旋增強，週末期間華南地區天色好轉及酷熱。熱帶氣旋燦都會在未來一兩日在上海以東海域徘徊，隨後移向朝鮮半島至日本一帶並逐漸演變為溫帶氣旋。",
        "weatherForecast": [
            {
                "forecastDate": "20210915",
                "week": "星期三",
                "forecastWind": "西至西南風2至3級。",
                "forecastWeather": "大致多雲，有幾陣驟雨及雷暴。下午短暫時間有陽光。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 54,
                "PSR": "中低"
            },
            {
                "forecastDate": "20210916",
                "week": "星期四",
                "forecastWind": "西至西南風3級。",
                "forecastWeather": "大致多雲，間中有驟雨及雷暴。",
                "forecastMaxtemp": {
                    "value": 31,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 75,
                    "unit": "percent"
                },
                "ForecastIcon": 63,
                "PSR": "中"
            },
            {
                "forecastDate": "20210917",
                "week": "星期五",
                "forecastWind": "微風2級。",
                "forecastWeather": "短暫時間有陽光，有幾陣驟雨。初時局部地區有雷暴。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 54,
                "PSR": "中低"
            },
            {
                "forecastDate": "20210918",
                "week": "星期六",
                "forecastWind": "南至東南風2至3級。",
                "forecastWeather": "部分時間有陽光，局部地區有驟雨。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 28,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 90,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 53,
                "PSR": "低"
            },
            {
                "forecastDate": "20210919",
                "week": "星期日",
                "forecastWind": "東南風2至3級。",
                "forecastWeather": "大致天晴，日間酷熱，局部地區有驟雨。",
                "forecastMaxtemp": {
                    "value": 33,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 28,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 90,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 65,
                    "unit": "percent"
                },
                "ForecastIcon": 90,
                "PSR": "低"
            },
            {
                "forecastDate": "20210920",
                "week": "星期一",
                "forecastWind": "東風2至3級。",
                "forecastWeather": "部分時間有陽光，有幾陣驟雨。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 53,
                "PSR": "低"
            },
            {
                "forecastDate": "20210921",
                "week": "星期二",
                "forecastWind": "東風3至4級。",
                "forecastWeather": "短暫時間有陽光，有幾陣驟雨。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 54,
                "PSR": "低"
            },
            {
                "forecastDate": "20210922",
                "week": "星期三",
                "forecastWind": "東風3至4級。",
                "forecastWeather": "短暫時間有陽光，有幾陣驟雨。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 54,
                "PSR": "低"
            },
            {
                "forecastDate": "20210923",
                "week": "星期四",
                "forecastWind": "東風3至4級。",
                "forecastWeather": "短暫時間有陽光，有幾陣驟雨。",
                "forecastMaxtemp": {
                    "value": 32,
                    "unit": "C"
                },
                "forecastMintemp": {
                    "value": 27,
                    "unit": "C"
                },
                "forecastMaxrh": {
                    "value": 95,
                    "unit": "percent"
                },
                "forecastMinrh": {
                    "value": 70,
                    "unit": "percent"
                },
                "ForecastIcon": 54,
                "PSR": "低"
            }
        ],
        "updateTime": "2021-09-14T11:30:00+08:00",
        "seaTemp": {
            "place": "北角",
            "value": 28,
            "unit": "C",
            "recordTime": "2021-09-14T07:00:00+08:00"
        },
        "soilTemp": [
            {
                "place": "香港天文台",
                "value": 31.2,
                "unit": "C",
                "recordTime": "2021-09-14T07:00:00+08:00",
                "depth": {
                    "unit": "metre",
                    "value": 0.5
                }
            },
            {
                "place": "香港天文台",
                "value": 30.8,
                "unit": "C",
                "recordTime": "2021-09-14T07:00:00+08:00",
                "depth": {
                    "unit": "metre",
                    "value": 1
                }
            }
        ]
    }"#
    };

    let nine_day: NineDay = serde_json::from_str(test_input).unwrap();
    println!("{:?}", nine_day);
    assert_eq!(nine_day.weather_forecast[0].max_temp.value, 32.);
    assert_eq!(nine_day.weather_forecast[1].psr, PSR::Medium);
    assert_eq!(nine_day.weather_forecast[2].icon, 54);
    assert_eq!(nine_day.sea_temp.temp.value, 28.);
    assert_eq!(nine_day.soil_temp[0].temp.value, 31.2);

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let nine_day: NineDay = fetch(Lang::EN).await.unwrap();
        println!("{:?}", nine_day);
    }
}

#[tokio::test]
async fn tips_test() {
    use crate::weather::tips::Tips;

    let test_input = {
        r#"{
        "swt": [
            {
                "desc": "Tips 1",
                "updateTime": "2020-09-24T14:10:00+08:00"
            },
            {
                "desc": "Tips 2"
            }
        ]
    }"#
    };

    let tips: Tips = serde_json::from_str(test_input).unwrap();
    println!("{:?}", tips);
    assert_eq!(tips.tips[1].desc, Some(format!("Tips 2")));

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let tips: Tips = fetch(Lang::EN).await.unwrap();
        println!("{:?}", tips);
    }
}

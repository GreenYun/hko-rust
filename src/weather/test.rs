// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(clippy::nursery)]
#![allow(clippy::pedantic)]

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

    assert!(!local.general_situation.is_empty());

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let local: Local = fetch(Lang::EN).await.unwrap();
        println!("{:?}", local);
    }
}

#[tokio::test]
async fn nineday_test() {
    use crate::weather::nine_day::NineDay;

    let test_input = {
        r#"{
            "generalSituation": "熱帶氣旋泰利會在今明兩日繼續增強，大致移向雷州半島至海南島一帶，並為廣東沿岸帶來狂風驟雨及雷暴。預料泰利會在本週中期遠離廣東沿岸，但受南海中至北部的一道廣闊低壓槽影響，廣東沿岸仍有驟雨。而高空反氣旋會在接近週末增強，廣東地區天色稍為好轉。",
            "weatherForecast": [
                {
                    "forecastDate": "20230717",
                    "week": "星期一",
                    "forecastWind": "東至東北風7級，間中8級，漸轉東至東南風7至8級，離岸及高地達9級，稍後東至東南風6至7級。",
                    "forecastWeather": "密雲，有狂風驟雨及雷暴，雨勢有時頗大。海有大至非常大浪及有湧浪。",
                    "forecastMaxtemp": {
                        "value": 29,
                        "unit": "C"
                    },
                    "forecastMintemp": {
                        "value": 26,
                        "unit": "C"
                    },
                    "forecastMaxrh": {
                        "value": 95,
                        "unit": "percent"
                    },
                    "forecastMinrh": {
                        "value": 80,
                        "unit": "percent"
                    },
                    "ForecastIcon": 64,
                    "PSR": "高"
                },
                {
                    "forecastDate": "20230718",
                    "week": "星期二",
                    "forecastWind": "東至東南風5至6級，初時離岸及高地間中7級。",
                    "forecastWeather": "多雲，有狂風驟雨及雷暴，初時雨勢有時頗大。海有湧浪。",
                    "forecastMaxtemp": {
                        "value": 29,
                        "unit": "C"
                    },
                    "forecastMintemp": {
                        "value": 26,
                        "unit": "C"
                    },
                    "forecastMaxrh": {
                        "value": 95,
                        "unit": "percent"
                    },
                    "forecastMinrh": {
                        "value": 80,
                        "unit": "percent"
                    },
                    "ForecastIcon": 64,
                    "PSR": "高"
                },
                {
                    "forecastDate": "20230719",
                    "week": "星期三",
                    "forecastWind": "東至東南風4至5級，離岸及高地間中6級。",
                    "forecastWeather": "大致多雲，間中有驟雨及雷暴。初時驟雨較多。",
                    "forecastMaxtemp": {
                        "value": 30,
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
                    "PSR": "高"
                },
                {
                    "forecastDate": "20230720",
                    "week": "星期四",
                    "forecastWind": "東至東南風4級，間中5級。",
                    "forecastWeather": "大致多雲，有幾陣驟雨，局部地區有雷暴。",
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
                    "ForecastIcon": 62,
                    "PSR": "中低"
                },
                {
                    "forecastDate": "20230721",
                    "week": "星期五",
                    "forecastWind": "東至東南風4級，間中5級。",
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
                        "value": 75,
                        "unit": "percent"
                    },
                    "ForecastIcon": 54,
                    "PSR": "低"
                },
                {
                    "forecastDate": "20230722",
                    "week": "星期六",
                    "forecastWind": "東至東南風4級。",
                    "forecastWeather": "部分時間有陽光，有一兩陣驟雨。",
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
                    "forecastDate": "20230723",
                    "week": "星期日",
                    "forecastWind": "東風3級。",
                    "forecastWeather": "部分時間有陽光及酷熱。有幾陣驟雨，稍後局部地區有雷暴。",
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
                    "ForecastIcon": 53,
                    "PSR": "中低"
                },
                {
                    "forecastDate": "20230724",
                    "week": "星期一",
                    "forecastWind": "東至東北風3至4級。",
                    "forecastWeather": "短暫時間有陽光及酷熱。有幾陣驟雨，稍後局部地區有雷暴。",
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
                    "ForecastIcon": 54,
                    "PSR": "中"
                },
                {
                    "forecastDate": "20230725",
                    "week": "星期二",
                    "forecastWind": "東至東南風4至5級。",
                    "forecastWeather": "大致多雲，有幾陣驟雨。",
                    "forecastMaxtemp": {
                        "value": 32,
                        "unit": "C"
                    },
                    "forecastMintemp": {
                        "value": 28,
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
                    "ForecastIcon": 62,
                    "PSR": "中"
                }
            ],
            "updateTime": "2023-07-16T19:50:00+08:00",
            "seaTemp": {
                "place": "北角",
                "value": 29,
                "unit": "C",
                "recordTime": "2023-07-16T14:00:00+08:00"
            },
            "soilTemp": [
                {
                    "place": "香港天文台",
                    "value": 30.9,
                    "unit": "C",
                    "recordTime": "2023-07-16T07:00:00+08:00",
                    "depth": {
                        "unit": "metre",
                        "value": 0.5
                    }
                },
                {
                    "place": "香港天文台",
                    "value": 30.2,
                    "unit": "C",
                    "recordTime": "2023-07-16T07:00:00+08:00",
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
    assert_eq!(tips.tips[1].desc, Some("Tips 2".to_owned()));

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let tips: Tips = fetch(Lang::EN).await.unwrap();
        println!("{:?}", tips);
    }
}

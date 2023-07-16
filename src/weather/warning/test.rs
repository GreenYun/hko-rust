// Copyright (c) 2021 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

#[tokio::test]
async fn info_test() {
    use crate::weather::warning::info::Info;

    let test_input = {
        r#"{
    "details": [
        {
            "contents": [
                "雷暴警告",
                "天文台在9月24日上午11時40分發出之雷暴警告，有效時間延長至今日下午7時30分，預料香港有雷暴。",
                "雷暴發生時，請採取以下預防措施：",
                "1. 留在室內。在室外的人士應躲入建築物內。",
                "2. 切勿站立於高地或接近導電的物體、樹木或桅杆。"
            ],
            "warningStatementCode": "WTS",
            "updateTime": "2020-09-24T05:00:00+08:00"
        },
        {
            "contents": [
                "強烈季候風信號在11時15分發出。"
            ],
            "warningStatementCode": "WMSGNL",
            "updateTime": "2020-09-24T11:15:00+08:00"
        },
        {
            "contents": [
                "三號強風信號在上午11時15分發出。"
            ],
            "subtype": "TC3",
            "warningStatementCode": "WTCSGNL",
            "updateTime": "2020-09-24T11:15:00+08:00"
        },
        {
            "contents": [
                "香港天文台在07時00分發出酷熱天氣警告。",
                "天文台預料今日本港天氣酷熱，加上風勢輕微，市民應提高警惕，以防中暑。",
                "在戶外工作或活動的人士，應多喝水和不要過度勞累。於感覺不適時，應盡快到陰涼的地方休息。",
                "在沒有空調設備室內的人士，應盡量打開窗戶以保持空氣流通。",
                "避免長時間在陽光下曝曬，以免受太陽紫外線曬傷。應穿上鬆身衣服以及配戴適當帽子和能阻隔紫外線的太陽眼鏡。",
                "泳客或在戶外遊玩的人士應重複塗抹防曬系數15 或以上的太陽油。",
                "請關注長者或慢性病患者的健康狀況。如認識他們，請間中致電或探訪他們，看看是否需要提供幫助。"
            ],
            "warningStatementCode": "WHOT",
            "updateTime": "2020-09-24T07:00:00+08:00"
        }
    ]
}"#
    };

    let warning: Info = serde_json::from_str(test_input).unwrap();
    println!("{warning:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let warning: Info = fetch(Lang::EN).await.unwrap();
        println!("{warning:?}");
    }
}

#[tokio::test]
async fn summary_test() {
    use crate::weather::warning::summary::Summary;

    let test_input = {
        r#"{
    "WHOT": {
        "name": "酷熱天氣警告",
        "code": "WHOT",
        "actionCode": "ISSUE",
        "issueTime": "2020-09-24T07:00:00+08:00",
        "updateTime": "2020-09-24T07:00:00+08:00"
    },
    "WRAIN": {
        "name": "暴雨警告信號",
        "code": "WRAINR",
        "type": "紅色",
        "actionCode": "ISSUE",
        "issueTime": "2020-09-24T11:15:00+08:00",
        "updateTime": "2020-09-24T11:15:00+08:00"
    },
    "WTS": {
        "name": "雷暴警告",
        "code": "WTS",
        "actionCode": "EXTEND",
        "issueTime": "2020-09-24T11:40:00+08:00",
        "expireTime": "2020-09-24T19:30:00+08:00",
        "updateTime": "2020-09-24T05:00:00+08:00"
    },
    "WTCSGNL": {
        "name": "熱帶氣旋警告信號",
        "code": "TC3",
        "actionCode": "ISSUE",
        "type": "三號強風信號",
        "issueTime": "2020-09-24T11:15:00+08:00",
        "updateTime": "2020-09-24T11:15:00+08:00"
    }
}"#
    };

    let summary: Summary = serde_json::from_str(test_input).unwrap();
    println!("{summary:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let summary: Summary = fetch(Lang::EN).await.unwrap();
        println!("{summary:?}");
    }
}

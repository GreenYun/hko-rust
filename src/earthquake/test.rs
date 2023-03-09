// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

#[tokio::test]
async fn message_test() {
    use crate::earthquake::message::Message;

    let test_input = {
        r#"{
    "lat": 44.28,
    "lon": -129.14,
    "mag": 6.0,
    "region": "俄勒岡離岸海域",
    "ptime": "2021-12-08T09:21:00+08:00",
    "updateTime": "2021-12-08T09:31:00+08:00"
}"#
    };

    let message: Message = serde_json::from_str(test_input).unwrap();
    println!("{message:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let message: Message = fetch(Lang::EN).await.unwrap();
        println!("{message:?}");
    }
}

#[tokio::test]
async fn felt_report_test() {
    use crate::earthquake::felt_report::FeltReport;

    let test_input = {
        r#"{
    "updateTime": "2022-03-14T03:15:00+08:00",
    "mag": 4.1,
    "region": "中國東南部近岸",
    "intensity": 4,
    "lat": 22.51,
    "lon": 115.04,
    "details": [
        "（更新）據香港天文台的初步分析，2022年3月14日(星期一)上午2時29分中國東南部近岸發生一次 4.1級地震，震中位於北緯22.51度，東經115.04度附近，即香港之東北偏東約92公里。",
        "震中位置圖 ︰ https:\/\/www.hko.gov.hk\/tc\/gts\/equake\/map.htm",
        "香港天文台接獲超過八千名市民報告，表示感到輕微震動，震動維持數秒。初步分析顯示本港的地震烈度為修訂麥加利地震烈度表的第IV (四)度，即懸掛的物件擺動。門、窗、碗碟發出響聲。",
        "有關修訂麥加利地震烈度表的詳細解釋，請參閱以下網頁︰https:\/\/www.hko.gov.hk\/tc\/gts\/equake\/mms.htm"
    ],
    "ptime": "2022-03-14T02:29:00+08:00"
}"#
    };

    let felt_report: FeltReport = serde_json::from_str(test_input).unwrap();
    println!("{felt_report:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let felt_report: FeltReport = fetch(Lang::EN).await.unwrap();
        println!("{felt_report:?}");
    }
}

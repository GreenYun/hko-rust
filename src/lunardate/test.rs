// Copyright (c) 2023 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use chrono::NaiveDate;

#[tokio::test]
async fn test() {
    use crate::lunardate::Response;

    let test_input = r#"{"LunarYear":"癸卯年，兔","LunarDate":"正月初一"}"#;

    let response: Response = serde_json::from_str(test_input).unwrap();
    println!("{response:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::lunardate::fetch;

        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let response: Response = fetch(date).await.unwrap();
        println!("{response:?}");
    }
}

// Copyright (c) 2023 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use crate::common::Lang;

#[tokio::test]
async fn test() {
    use crate::hourly_rainfall::Response;

    let test_input = r#"{
    "obsTime": "2022-09-01T08:00:00+08:00",
    "hourlyRainfall": [
        {
        "automaticWeatherStation": "Lau Fau Shan",
        "automaticWeatherStationID": "RF001",
        "value": "0",
        "unit": "mm"
        },
        {
        "automaticWeatherStation": "Shui Pin Wai",
        "automaticWeatherStationID": "N12",
        "value": "M",
        "unit": "mm"
        }
    ]
}"#;

    let response: Response = serde_json::from_str(test_input).unwrap();
    println!("{response:?}");

    #[cfg(feature = "fetch")]
    {
        use crate::hourly_rainfall::fetch;

        let response: Response = fetch(&Lang::TC).await.unwrap();
        println!("{response:?}");
    }
}

// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
    println!("{:?}", message);

    #[cfg(feature = "fetch")]
    {
        use crate::{common::Lang, fetch::fetch};

        let message: Message = fetch(Lang::EN).await.unwrap();
        println!("{:?}", message);
    }
}

// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(clippy::nursery)]
#![allow(clippy::pedantic)]

use std::str::FromStr;

macro_rules! response_from_str {
    ($s:expr $(,)?) => {{
        Response::from_str($s).unwrap()
    }};
}

#[tokio::test]
async fn test_hhot() {
    use super::hhot::Response;

    // CSV with header
    let Response(r1) = response_from_str!(
        r#"MM,DD,01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24
01,01,0.70,0.54,0.56,0.70,0.91,1.09,1.27,1.45,1.54,1.54,1.43,1.32,1.26,1.32,1.57,1.91,2.26,2.50,2.59,2.59,2.47,2.22,1.83,1.34
01,02,0.87,0.48,0.32,0.36,0.55,0.80,1.01,1.24,1.44,1.54,1.54,1.42,1.33,1.29,1.40,1.70,2.07,2.45,2.66,2.72,2.70,2.53,2.24,1.78"#,
    );

    // CSV without header
    let Response(r2) = response_from_str!(
        r#"01,01,0.70,0.54,0.56,0.70,0.91,1.09,1.27,1.45,1.54,1.54,1.43,1.32,1.26,1.32,1.57,1.91,2.26,2.50,2.59,2.59,2.47,2.22,1.83,1.34
01,02,0.87,0.48,0.32,0.36,0.55,0.80,1.01,1.24,1.44,1.54,1.54,1.42,1.33,1.29,1.40,1.70,2.07,2.45,2.66,2.72,2.70,2.53,2.24,1.78"#,
    );

    // JSON
    let Response(r3) = response_from_str!(
        r#"{
    "fields": ["MM", "DD", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24"],
    "data": [
        ["01", "01", "0.70", "0.54", "0.56", "0.70", "0.91", "1.09", "1.27", "1.45", "1.54", "1.54", "1.43", "1.32", "1.26", "1.32", "1.57", "1.91", "2.26", "2.50", "2.59", "2.59", "2.47", "2.22", "1.83", "1.34"],
        ["01", "02", "0.87", "0.48", "0.32", "0.36", "0.55", "0.80", "1.01", "1.24", "1.44", "1.54", "1.54", "1.42", "1.33", "1.29", "1.40", "1.70", "2.07", "2.45", "2.66", "2.72", "2.70", "2.53", "2.24", "1.78"]
    ]}"#,
    );

    assert!(r1.len() == r2.len() && r2.len() == r3.len());
    assert!(r1[0].month == r2[0].month && r2[0].month == r3[0].month);
    assert!(r1[0].day == r2[0].day && r2[0].day == r3[0].day);
    assert!(r1[0].hour == r2[0].hour && r2[0].hour == r3[0].hour);
    assert!(r1[0].height == r2[0].height && r2[0].height == r3[0].height);

    // CSV specifying hour
    let Response(r1) = response_from_str!("MM,DD,02\n01,01,0.54");

    // JSON specifying hour
    let Response(r2) = response_from_str!(
        r#"{
    "fields": ["MM", "DD", "02"],
    "data": [["01", "01", "0.54"]]}"#,
    );

    assert!(r1.len() == r2.len() && r2.len() == 1);
    assert!(r1[0].month == r2[0].month);
    assert!(r1[0].day == r2[0].day);
    assert!(r1[0].hour == r2[0].hour && r2[0].hour == 2);
    assert!(r1[0].height == r2[0].height);

    #[cfg(feature = "fetch")]
    {
        use super::{hhot::fetch, SeaStation::CCH};

        let Response(_) = fetch(CCH, 2021, None, None, None, None).await.unwrap();
    }
}

#[tokio::test]
async fn test_hlt() {
    use super::hlt::Response;

    // CSV with header
    let Response(r1) = response_from_str!(
        r#"Month,Date,Time,Height(m),Time,Height(m),Time,Height(m),Time,Height(m)
01,01,0219,0.53,0930,1.55,1308,1.26,1934,2.60
01,02,0313,0.31,1030,1.55,,,,
01,03,0406,0.18,1121,1.54,1443,1.30,,"#,
    );

    // CSV without header
    let Response(r2) = response_from_str!(
        r#"01,01,0219,0.53,0930,1.55,1308,1.26,1934,2.60
01,02,0313,0.31,1030,1.55,,,,
01,03,0406,0.18,1121,1.54,1443,1.30,,"#,
    );

    // JSON
    let Response(r3) = response_from_str!(
        r#"{
    "fields": ["Month", "Date", "Time", "Height(m)", "Time", "Height(m)", "Time", "Height(m)", "Time", "Height(m)"],
    "data": [
        ["01", "01", "0219", "0.53", "0930", "1.55", "1308", "1.26", "1934", "2.60"],
        ["01", "02", "0313", "0.31", "1030", "1.55", "", "", "", ""],
        ["01", "03", "0406", "0.18", "1121", "1.54", "1443", "1.30", "", ""]]}"#,
    );

    assert!(r1.len() == r2.len() && r2.len() == r3.len());
    assert!(r1[0].month == r2[0].month && r2[0].month == r3[0].month);
    assert!(r1[0].day == r2[0].day && r2[0].day == r3[0].day);
    assert!(r1[0].hour == r2[0].hour && r2[0].hour == r3[0].hour);
    assert!(r1[0].minute == r2[0].minute && r2[0].minute == r3[0].minute);
    assert!(r1[0].height == r2[0].height && r2[0].height == r3[0].height);

    #[cfg(feature = "fetch")]
    {
        use super::{hlt::fetch, SeaStation::CCH};

        let Response(_) = fetch(CCH, 2021, None).await.unwrap();
    }
}

#[tokio::test]
async fn test_lhl() {
    use super::lhl::Response;

    // CSV with header
    let Response(r1) = response_from_str!(
        r#"DateTime,Type,Region,"lightning count"
202201010100-202201010159,Cloud-to-ground,"New Territories West",1
202201010100-202201010159,Cloud-to-ground,"New Territories East",0
202201010100-202201010159,Cloud-to-ground,"Hong Kong Island and Kowloon",0
202201010100-202201010159,Cloud-to-ground,Lantau,3
202201010100-202201010159,Cloud-to-ground,"Hong Kong territory",0
202201010100-202201010159,Cloud-to-cloud,"Hong Kong territory",0"#,
    );

    // CSV without header
    let Response(r2) = response_from_str!(
        r#"202201010100-202201010159,Cloud-to-ground,"New Territories West",1
202201010100-202201010159,Cloud-to-ground,"New Territories East",0
202201010100-202201010159,Cloud-to-ground,"Hong Kong Island and Kowloon",0
202201010100-202201010159,Cloud-to-ground,Lantau,3
202201010100-202201010159,Cloud-to-ground,"Hong Kong territory",0
202201010100-202201010159,Cloud-to-cloud,"Hong Kong territory",0"#,
    );

    // JSON
    let Response(r3) = response_from_str!(
        r#"{
    "fields":["DateTime", "Type", "Region", "lightning count"],
    "data":[
        ["202201010100-202201010159", "Cloud-to-ground", "New Territories West", "1"],
        ["202201010100-202201010159", "Cloud-to-ground", "New Territories East", "0"],
        ["202201010100-202201010159", "Cloud-to-ground", "Hong Kong Island and Kowloon", "0"],
        ["202201010100-202201010159", "Cloud-to-ground", "Lantau", "3"],
        ["202201010100-202201010159", "Cloud-to-ground", "Hong Kong territory", "0"],
        ["202201010100-202201010159", "Cloud-to-cloud", "Hong Kong territory", "0"]]}"#,
    );

    assert!(r1.len() == r2.len() && r2.len() == r3.len());
    assert!(r1[0].start_time == r2[0].start_time && r2[0].start_time == r3[0].start_time);
    assert!(r1[0].end_time == r2[0].end_time && r2[0].end_time == r3[0].end_time);
    assert!(r1[0].r#type == r2[0].r#type && r2[0].r#type == r3[0].r#type);
    assert!(r1[0].region == r2[0].region && r2[0].region == r3[0].region);
    assert!(r1[0].count == r2[0].count && r2[0].count == r3[0].count);

    #[cfg(feature = "fetch")]
    {
        use super::lhl::fetch;
        use crate::common::Lang::TC;

        let Response(_) = fetch(TC, None).await.unwrap();
    }
}

#[tokio::test]
async fn test_ltmv() {
    use super::ltmv::Response;

    let Response(r1) = response_from_str!(
        r#""Date time","Automatic Weather Station","10 minute mean visibility"
202201010110,Central,10km
202201010110,"Chek Lap Kok",9km
202201010110,"Sai Wan Ho",14km
202201010110,"Waglan Island",N/A"#
    );

    let Response(r2) = response_from_str!(
        r#"202201010110,Central,10km
202201010110,"Chek Lap Kok",9km
202201010110,"Sai Wan Ho",14km
202201010110,"Waglan Island",N/A"#
    );

    let Response(r3) = response_from_str!(
        r#"{
    "fields":["Date time", "Automatic Weather Station", "10 minute mean visibility"],
    "data":[
        ["202201010110", "Central", "10km"],
        ["202201010110", "Chek Lap Kok", "9km"],
        ["202201010110", "Sai Wan Ho", "14km"],
        ["202201010110", "Waglan Island", "N\/A"]]}"#
    );

    assert!(r1.len() == r2.len() && r2.len() == r3.len());
    assert!(r1[0].time == r2[0].time && r2[0].time == r3[0].time);
    assert!(r1[0].station == r2[0].station && r2[0].station == r3[0].station);
    assert!(r1[0].visibility == r2[0].visibility && r2[0].visibility == r3[0].visibility);

    #[cfg(feature = "fetch")]
    {
        use super::ltmv::fetch;
        use crate::common::Lang::TC;

        let Response(_) = fetch(TC, None).await.unwrap();
    }
}

#[tokio::test]
async fn test_ryes() {
    use super::ryes::Response;

    std::mem::drop(Response::from_str(
        r#"{
    "ChekLapKokLocationName": "Chek Lap Kok",
    "ChekLapKokMaxTemp": "19.4",
    "ChekLapKokMicrosieverts": "0.14",
    "ChekLapKokMinTemp": "16.2",
    "BulletinTime": "0015",
    "BulletinDate": "20220102",
    "ReportTimeInfoDate": "20220101",
    "HongKongDesc": "Average ambient gamma radiation dose rate taken outdoors in Hong Kong ranged from 0.09 to 0.14 microsievert per hour.  These are within the normal range of fluctuation of the background radiation level in Hong Kong.",
    "NoteDesc": "From readings taken at various locations in Hong Kong in the past, the hourly mean ambient gamma radiation dose rate may vary between 0.06 and 0.3 microsievert per hour. (1 microsievert = 0.000001 sievert = 0.001 millisievert)",
    "NoteDesc1": "Temporal variations are generally caused by changes in meteorological conditions such as rainfall, wind and barometric pressure.",
    "NoteDesc2": "Spatial variations are generally caused by differences in the radioactive content of local rock and soil.",
    "NoteDesc3": "The data displayed is provisional. Only limited data validation has been carried out.",
    "CheungChauLocationName": "Cheung Chau",
    "CheungChauMaxTemp": "18.3",
    "CheungChauMinTemp": "15.1",
    "HKOReadingsAccumRainfall": "0",
    "HKOReadingsAvgRainfall": "trace",
    "HKOReadingsMaxRH": "81",
    "HKOReadingsMaxTemp": "19.3",
    "HKOReadingsMinGrassTemp": "14.4",
    "HKOReadingsMinRH": "69",
    "HKOReadingsMinTemp": "16.4",
    "HKOReadingsRainfall": "0",
    "HappyValleyLocationName": "Happy Valley",
    "HappyValleyMaxTemp": "20.1",
    "HappyValleyMinTemp": "14.5",
    "HongKongParkLocationName": "Hong Kong Park",
    "HongKongParkMaxTemp": "18.8",
    "HongKongParkMinTemp": "15.5",
    "KaiTakRunwayParkLocationName": "Kai Tak Runway Park",
    "KaiTakRunwayParkMaxTemp": "18.8",
    "KaiTakRunwayParkMinTemp": "16.5",
    "KatOLocationName": "Kat O",
    "KatOMicrosieverts": "0.11",
    "KingsParkLocationName": "King's Park",
    "KingsParkMicrosieverts": "0.14",
    "KingsParkReadingsMaxTemp": "18.9",
    "KingsParkReadingsMaxUVIndex": "2",
    "KingsParkReadingsMeanUVIndex": "1",
    "KingsParkReadingsMinTemp": "15.1",
    "KingsParkReadingsSunShine": "1.6",
    "KowloonCityLocationName": "Kowloon City",
    "KowloonCityMaxTemp": "19.3",
    "KowloonCityMinTemp": "14.5",
    "KwunTongLocationName":secs "Kwun Tong",
    "KwunTongMaxTemp": "18.8",
    "KwunTongMicrosieverts": "0.12",
    "KwunTongMinTemp": "15.1",
    "LauFauShanLocationName": "Lau Fau Shan",
    "LauFauShanMaxTemp": "19.1",
    "LauFauShanMinTemp": "14.2",
    "PingChauLocationName": "Ping Chau",
    "PingChauMicrosieverts": "0.09",
    "SaiKungLocationName": "Sai Kung",
    "SaiKungMaxTemp": "19.2",
    "SaiKungMinTemp": "14.4",
    "SaiWanHoLocationName": "Sai Wan Ho",
    "SaiWanHoMicrosieverts": "0.09",
    "ShaTauKokLocationName": "Sha Tau Kok",
    "ShaTauKokMicrosieverts": "0.10",
    "ShaTinLocationName": "Sha Tin",
    "ShaTinMaxTemp": "19.6",
    "ShaTinMinTemp": "14.2",
    "ShamShuiPoLocationName": "Sham Shui Po",
    "ShamShuiPoMaxTemp": "20.1",
    "ShamShuiPoMinTemp": "15.5",
    "ShauKeiWanLocationName": "Shau Kei Wan",
    "ShauKeiWanMaxTemp": "18.4",
    "ShauKeiWanMinTemp": "15.5",
    "ShekKongLocationName": "Shek Kong",
    "ShekKongMaxTemp": "20.5",
    "ShekKongMinTemp": "12.9",
    "StanleyLocationName": "Stanley",
    "StanleyMaxTemp": "18.8",
    "StanleyMinTemp": "15.1",
    "TaKwuLingLocationName": "Ta Kwu Ling",
    "TaKwuLingMaxTemp": "20.3",
    "TaKwuLingMinTemp": "11.6",
    "TaiMeiTukLocationName": "Tai Mei Tuk",
    "TaiMeiTukMaxTemp": "19.7",
    "TaiMeiTukMicrosieverts": "0.12",
    "TaiMeiTukMinTemp": "14.9",
    "TaiPoLocationName": "Tai Po",
    "TaiPoMaxTemp": "18.3",
    "TaiPoMinTemp": "14.0",
    "TapMunLocationName": "Tap Mun",
    "TapMunMicrosieverts": "0.09",
    "TseungKwanOLocationName": "Tseung Kwan O",
    "TseungKwanOMaxTemp": "19.6",
    "TseungKwanOMinTemp": "14.0",
    "TsimBeiTsuiLocationName": "Tsim Bei Tsui",
    "TsimBeiTsuiMicrosieverts": "0.12",
    "TsingYiLocationName": "Tsing Yi",
    "TsingYiMaxTemp": "",
    "TsingYiMinTemp": "",
    "TsuenWanHoKoonLocationName": "Tsuen Wan Ho Koon",
    "TsuenWanHoKoonMaxTemp": "18.7",
    "TsuenWanHoKoonMinTemp": "12.9",
    "TsuenWanShingMunValleyLocationName": "Tsuen Wan Shing Mun Valley",
    "TsuenWanShingMunValleyMaxTemp": "20.2",
    "TsuenWanShingMunValleyMinTemp": "13.3",
    "TuenMunLocationName": "Tuen Mun",
    "TuenMunMaxTemp": "19.3",
    "TuenMunMinTemp": "14.9",
    "WongChukHangLocationName": "Wong Chuk Hang",
    "WongChukHangMaxTemp": "19.4",
    "WongChukHangMinTemp": "13.8",
    "WongTaiSinLocationName": "Wong Tai Sin",
    "WongTaiSinMaxTemp": "19.9",
    "WongTaiSinMinTemp": "14.6",
    "YuenLongParkLocationName": "Yuen Long Park",
    "YuenLongParkMaxTemp": "20.2",
    "YuenLongParkMinTemp": "13.3",
    "YuenNgFanLocationName": "Yuen Ng Fan",
    "YuenNgFanMicrosieverts": "0.12"
}"#,
    ));

    #[cfg(feature = "fetch")]
    {
        use chrono::NaiveDate;

        use super::ryes::fetch;

        std::mem::drop(
            fetch(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(), None, None)
                .await
                .unwrap(),
        );
    }
}

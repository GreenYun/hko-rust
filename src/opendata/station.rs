// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use strum_macros::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum SeaStation {
    /// Cheung Chau
    CCH,
    /// Chek Lap Kok
    CLK,
    /// Chi Ma Wan
    CMW,
    /// Kwai Chung
    KCT,
    /// Ko Lau Wan
    KLW,
    /// Lok On Pai
    LOP,
    /// Ma Wan
    MWC,
    /// Quarry Bay
    QUB,
    /// Shek Pik
    SPW,
    /// Tai O
    TAO,
    /// Tsim Bei Tsui
    TBT,
    /// Tai Miu Wan
    TMW,
    /// Tai Po Kau
    TPK,
    /// Waglan Island
    WAG,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum TempStation {
    /// Cheung Chau
    CCH,
    /// Clear Water Bay
    CWB,
    /// Hong Kong International Airport
    HKA,
    /// Hong Kong Observatory
    HKO,
    /// Hong Kong Park
    HKP,
    /// Wong Chuk Hang
    HKS,
    /// Happy Valley
    HPV,
    /// Tseung Kwan O
    JKB,
    /// Kowloon City
    KLT,
    /// King's Park
    KP,
    /// Kau Sai Chau
    KSC,
    /// Kwun Tong
    KTG,
    /// Lau Fau Shan
    LFS,
    /// Ngong Ping
    NGP,
    /// Peng Chau
    PEN,
    /// Tai Mei Tuk
    PLC,
    /// Kai Tak Runway Park
    SE1,
    /// Shek Kong
    SEK,
    /// Sha Tin
    SHA,
    /// Sai Kung
    SKG,
    /// Shau Kei Wan
    SKW,
    /// Sheung Shui
    SSH,
    /// Sham Shui Po
    SSP,
    /// Stanley
    STY,
    /// Tate's Cairn
    TC,
    /// Ta Kwu Ling
    TKL,
    /// Tai Mo Shan
    TMS,
    /// Tai Po
    TPO,
    /// Tuen Mun Children and Juvenile Home
    TU1,
    /// Tsuen Wan Shing Mun Valley
    TW,
    /// Tsuen Wan
    TWN,
    /// New Tsing Yi Station
    TY1,
    /// Pak Tam Chung (Tsak Yue Wu)
    TYW,
    /// The Peak
    VP1,
    /// Waglan Island
    WGL,
    /// Wetland Park
    WLP,
    /// Wong Tai Sin
    WTS,
    /// Yuen Long Park
    YLP,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum WeatherStation {
    /// Cheung Chau
    CCH,
    /// Chek Lap Kok
    CLK,
    /// Ping Chau
    EPC,
    /// Hong Kong Observatory
    HKO,
    /// Hong Kong Park
    HKP,
    /// Wong Chuk Hang
    HKS,
    /// Happy Valley
    HPV,
    /// Tseung Kwan O
    JKB,
    /// Kat O
    KAT,
    /// Kowloon City
    KLT,
    /// King's Park
    KP,
    /// Kwun Tong
    KTG,
    /// Lau Fau Shan
    LFS,
    /// Tai Mei Tuk
    PLC,
    /// Kai Tak Runway Park
    SE1,
    /// Shek Kong
    SEK,
    /// Sha Tin
    SHA,
    /// Sai Kung
    SKG,
    /// Shau Kei Wan
    SKW,
    /// Sham Shui Po
    SSP,
    /// Sha Tau Kok
    STK,
    /// Stanley
    STY,
    /// Sai Wan Ho
    SWH,
    /// Tap Mun
    TAP,
    /// Tsim Bei Tsui
    TBT,
    /// Ta Kwu Ling
    TKL,
    /// Tai Po
    TPO,
    /// Tuen Mun
    TUN,
    /// Tsuen Wan Shing Mun Valley
    TW,
    /// Tsuen Wan Ho Koon
    TWN,
    /// Tsing Yi
    TY1,
    /// Wong Tai Sin
    WTS,
    /// Yuen Long Park
    YLP,
    /// Yuen Ng Fan
    YNF,
}
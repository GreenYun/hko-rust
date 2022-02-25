// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use serde::Deserialize;

/// `value` with its `unit`.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct ValUnit {
    pub value: f32,
    pub unit: String,
}

/// `value` with its `unit` in specified `place`.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PlaceValUnit {
    pub place: String,
    pub value: f32,
    pub unit: String,
}

/// One or more slice of messages.
///
/// [`String`](Message::String)`(`[`String`]`)` might be empty.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    String(String),
    List(Vec<String>),
}

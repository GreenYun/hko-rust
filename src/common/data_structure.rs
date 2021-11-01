// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::Deserialize;

/// `value` with `unit`.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct ValUnit {
    pub value: f32,
    pub unit: String,
}

/// `value` with `unit` in specified `place`.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PlaceValUnit {
    pub place: String,
    pub value: f32,
    pub unit: String,
}

/// One or more slice of messages.
///
/// [`String`](Message::String)`(`[`String`]`)` might be empty.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    String(String),
    List(Vec<String>),
}

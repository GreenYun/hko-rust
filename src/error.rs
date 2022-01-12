// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Error types.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

/// The error type returned when parsing a response.
#[derive(Clone, Debug)]
pub enum DataError {
    /// The response is empty.
    EarlyEOF,

    /// The response is not in the expected format.
    SourceFormat(String),
}

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DataError::SourceFormat(s) => f.write_fmt(format_args!("Source format error: {}", s)),
            DataError::EarlyEOF => "early EOF found".fmt(f),
        }
    }
}

impl Error for DataError {}

/// The error type returned when trying to convert invalid string to PSR type.
#[derive(Clone, Copy, Debug)]
pub struct InvalidPSRError;

impl Display for InvalidPSRError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        "PSR value is not valid".fmt(f)
    }
}

impl Error for InvalidPSRError {}

/// The error type returned when encountering illegal parameters in API request.
#[derive(Clone, Debug)]
pub struct APIRequestError(pub String);

impl Display for APIRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self(s) => f.write_fmt(format_args!("illegal parameter: {}", s)),
        }
    }
}

impl Error for APIRequestError {}

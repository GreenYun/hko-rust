// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{marker::PhantomData, ptr::NonNull, vec};

use serde::Deserialize;

/// An iterator over the elements of a `Message`.
///
/// This struct is created by the [`iter`](Message::iter) method on [`Message`].
pub struct Iter<'a> {
    ptr: NonNull<String>,
    end: *const String,
    _marker: PhantomData<&'a String>,
}

impl<'a> Iter<'a> {
    #[inline]
    fn post_inc_start(&mut self) -> *const String {
        let old = self.ptr.as_ptr();

        self.ptr = unsafe { NonNull::new_unchecked(self.ptr.as_ptr().add(1)) };

        old
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.ptr.as_ptr() as *const String == self.end {
                None
            } else {
                Some(&*self.post_inc_start())
            }
        }
    }
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

impl Message {
    /// Returns true if the result is [`String`](Message::String).
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns true if the result is [`List`](Message::List).
    pub const fn is_list(&self) -> bool {
        !self.is_string()
    }

    /// Converts from `Message` to [`Option`]`<`[`String`]`>`.
    ///
    /// Converts `self` into an [`Option`]`<`[`String`]`>`, consuming `self`, and discarding the list, if any.
    pub fn string(self) -> Option<String> {
        match self {
            Self::String(x) => Some(x),
            #[allow(unused_variables)]
            Self::List(x) => None,
        }
    }

    /// Converts from `Message` to [`Option`]`<`[`Vec`]`<`[`String`]`>>`.
    ///
    /// Converts `self` into an [`Option`]`<`[`Vec`]`<`[`String`]`>>`, consuming `self`, and discarding the string, if any.
    pub fn list(self) -> Option<Vec<String>> {
        match self {
            #[allow(unused_variables)]
            Self::String(x) => None,
            Self::List(x) => Some(x),
        }
    }

    /// Returns an iterator over the possibly contained value.
    pub fn iter(&self) -> Iter {
        let (ptr, len) = match self {
            Self::String(x) => (x as *const String, 1),
            Self::List(x) => (x.as_ptr(), x.len()),
        };

        unsafe {
            Iter {
                ptr: NonNull::new_unchecked(ptr as *mut String),
                end: ptr.add(len),
                _marker: PhantomData,
            }
        }
    }
}

impl IntoIterator for Message {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::String(x) => vec![x],
            Self::List(x) => x,
        }
        .into_iter()
    }
}

impl<'a> IntoIterator for &'a Message {
    type Item = &'a String;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

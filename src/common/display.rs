// Copyright (c) 2021 - 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use crate::common::Lang;

/// Trait to display the name and the description of values of an enumeration.
pub trait EnumNameDesc: Debug {
    /// Gets the name of the item.
    #[allow(unused_variables)]
    fn name(&self, lang: Lang) -> String {
        format!("{:?}", self)
    }

    /// Gets the description of the item.
    #[allow(unused_variables)]
    fn desc(&self, lang: Lang) -> String {
        format!("{:?}", self)
    }
}

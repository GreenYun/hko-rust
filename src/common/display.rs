// Copyright (c) 2022 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::Debug;

use crate::common::Lang;

pub trait EnumNameDesc: Debug {
    #[allow(unused_variables)]
    fn name(&self, lang: Lang) -> String {
        format!("{:?}", self)
    }

    #[allow(unused_variables)]
    fn desc(&self, lang: Lang) -> String {
        format!("{:?}", self)
    }
}

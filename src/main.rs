// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use hko::{common::Lang, fetch, psr_icon_uri, weather::current::Current};

#[allow(unused_must_use)]
#[tokio::main]
async fn main() {
    let _c: Current = fetch(Lang::en).await.unwrap();
    psr_icon_uri!(hko::weather::PSR::High);
}

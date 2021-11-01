// Copyright (c) 2021 GreenYun Organization
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[cfg(test)]
mod tests {
    use crate::weather::PSR;

    #[test]
    fn test_name() {
        let psr = PSR::High;
        println!("{:b}", psr);
        println!("{:e}", psr);
        println!("{:E}", psr);
        println!("{:o}", psr);
        println!("{:p}", psr);
        println!("{:x}", psr);
    }
}

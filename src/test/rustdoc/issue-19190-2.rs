// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;

pub struct Bar;

impl Deref for Bar {
    type Target = i32;
    fn deref(&self) -> &i32 { loop {} }
}

// @has issue_19190_2/struct.Bar.html
// @has - '//*[@id="method.count_ones"]' 'fn count_ones(self) -> u32'
// @!has - '//*[@id="method.min_value"]' 'fn min_value() -> i32'

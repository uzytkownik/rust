// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: --test

#![feature(rand, std_panic)]

use std::env::*;
use std::__rand as rand;
use std::__rand::Rng;
use std::iter::repeat;
use std::ffi::{OsString, OsStr};


fn make_rand_name() -> OsString {
    let mut rng = rand::thread_rng();
    let n = format!("TEST{}", rng.gen_ascii_chars().take(10)
                                 .collect::<String>());
    let n = OsString::from(n);
    assert!(var_os(&n).is_none());
    n
}

fn eq(a: Option<OsString>, b: Option<&str>) {
    assert_eq!(a.as_ref().map(|s| &**s), b.map(OsStr::new).map(|s| &*s));
}

#[test]
fn test_set_var() {
    let n = make_rand_name();
    set_var(&n, "VALUE");
    eq(var_os(&n), Some("VALUE"));
}

#[test]
fn test_remove_var() {
    let n = make_rand_name();
    set_var(&n, "VALUE");
    remove_var(&n);
    eq(var_os(&n), None);
}

#[test]
fn test_set_var_overwrite() {
    let n = make_rand_name();
    set_var(&n, "1");
    set_var(&n, "2");
    eq(var_os(&n), Some("2"));
    set_var(&n, "");
    eq(var_os(&n), Some(""));
}

#[test]
#[cfg_attr(target_os = "emscripten", ignore)]
fn test_var_big() {
    let mut s = "".to_string();
    let mut i = 0;
    while i < 100 {
        s.push_str("aaaaaaaaaa");
        i += 1;
    }
    let n = make_rand_name();
    set_var(&n, &s);
    eq(var_os(&n), Some(&s));
}

#[test]
#[cfg_attr(target_os = "emscripten", ignore)]
fn test_env_set_get_huge() {
    let n = make_rand_name();
    let s = repeat("x").take(10000).collect::<String>();
    set_var(&n, &s);
    eq(var_os(&n), Some(&s));
    remove_var(&n);
    eq(var_os(&n), None);
}

#[test]
fn test_env_set_var() {
    let n = make_rand_name();

    let mut e = vars_os();
    set_var(&n, "VALUE");
    assert!(!e.any(|(k, v)| {
        &*k == &*n && &*v == "VALUE"
    }));

    assert!(vars_os().any(|(k, v)| {
        &*k == &*n && &*v == "VALUE"
    }));
}

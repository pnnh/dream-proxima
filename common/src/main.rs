use common;

extern crate libc;

use std::ffi::CStr;
use std::str;

use handlebars::Handlebars;
use libc::c_char;
use serde_json::json;

fn main() {
    let result = common::random_string(16, true,
                                       true, false, true);

    println!("value: {}", result);
    println!("value: haha");
}

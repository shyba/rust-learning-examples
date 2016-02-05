extern crate iron;

use iron::prelude::*;
use iron::status;
use std::ffi::CStr;
extern crate libc;
use std::str;

#[no_mangle]
pub extern fn listen_with_callback(callback: fn()-> *const libc::c_char) {
    Iron::new(move |_: &mut Request| {
        let str_ptr = callback();
        let c_str = unsafe {
            assert!(!str_ptr.is_null());
            CStr::from_ptr(str_ptr)
        };
        let r_str = str::from_utf8(c_str.to_bytes()).unwrap();
        Ok(Response::with((status::Ok, r_str)))
    }).http("localhost:3000").unwrap();
}

#[test]
fn it_works() {
}

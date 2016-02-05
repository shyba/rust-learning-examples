extern crate iron;

use iron::prelude::*;
use iron::status;
use std::ffi::CStr;
extern crate libc;
use std::str;
use std::io::Read;
use std::ffi::CString;

#[no_mangle]
pub extern fn listen_with_callback(callback: fn(*const libc::c_char)-> *const libc::c_char) {
    Iron::new(move |req: &mut Request| {
        let mut buffer = String::new();
        req.body.read_to_string(&mut buffer).ok().expect("error reading body!");
        let callback_string = CString::new(buffer).unwrap().into_raw();
        let str_ptr = callback(callback_string);
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

use std::thread;
use std::ffi::CString;
extern crate libc;

#[no_mangle]
pub extern fn process(callback: fn(i32) -> (i32)) -> *const libc::c_char {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(move || {
            let mut x = 0;
            for _ in 0..5_000_000 {
                x += 1
            }
            callback(x)
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
                 h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
    let returned_string = CString::new("Done!").unwrap();
    returned_string.into_raw()
}

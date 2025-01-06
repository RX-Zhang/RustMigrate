

use std::mem;
use std::boxed::Box;
use std::os::raw::c_int;
use libc::c_void;

fn my_function(x: Box<i32>, y: Box<i32>) -> Box<i32> {
    let x_raw = Box::into_raw(x);
    let y_raw = Box::into_raw(y);
    let result = unsafe { Box::from_raw(Box::leak(Box::new(unsafe { *x_raw }.wrapping_add(unsafe { *y_raw }))) )};
    mem::forget(x_raw);
    mem::forget(y_raw);
    result
}


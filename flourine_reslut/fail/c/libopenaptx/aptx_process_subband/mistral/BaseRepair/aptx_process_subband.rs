

extern crate libc;

use libc::{c_int, int32_t, int16_t, int64_t, int64_t as i64, uint32_t, uint64_t, uint64_t as u64};
use std::mem;

const DIFFSIGN: fn(c_int) -> c_int = i32::signum;

fn c_diff_sign(x: c_int) -> c_int {
    DIFFSIGN(x) as c_int
}

fn c_wrapping_add(x: uint32_t, y: uint32_t) -> uint32_t {
    (x.wrapping_add(y)) & (u32::MAX as u32)
}

fn c_wrapping_sub(x: uint32_t, y: uint32_t) -> uint32_t {
    (x.wrapping_sub(y)) & (u32::MAX as u32)
}

fn c_wrapping_mul(x: uint32_t, y: uint32_t) -> uint32_t {
    (x.wrapping_mul(y)) & (u32::MAX as u32)
}

fn c_wrapping_div(x: uint32_t, y: uint32_t) -> uint32_t {
    if y == 0 {
        0
    } else {
        (x.wrapping_div(y)) & (u32::MAX as u32)
    }
}

fn c_wrapping_rem(x: uint32_t, y: uint32_t) -> uint32_t {
    if y == 0 {
        0
    } else {
        (x.wrapping_rem(y)) & (u32::MAX as u32)
    }
}


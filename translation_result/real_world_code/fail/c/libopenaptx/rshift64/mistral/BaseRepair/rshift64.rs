
use libc::int64_t;
use std::ops::{Shr, ShrAssign};

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 << (shift - 1)) as int64_t;
    let mask = ((1 << (shift + 1)) - 1) as int64_t;
    let shifted = (value.wrapping_add(rounding)).wrapping_shr(shift);
    shifted.wrapping_sub((value & mask).wrapping_shr(shift))
}

fn shr64(value: Box<int64_t>, shift: u32) -> Box<int64_t> {
    Box::new(rshift64(*value, shift))
}

fn shr_assign64(value: &mut Box<int64_t>, shift: u32) {
    **value = rshift64(**value, shift);
}

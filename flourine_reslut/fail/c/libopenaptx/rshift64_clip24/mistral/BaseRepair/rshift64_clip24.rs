

use libc::int32_t;
use libc::int64_t;
use libc::uint32_t;
use libc::uint64_t;
use std::ops::Shr;
use std::boxed::Box;

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
if ((a as uint32_t).wrapping_add((1 << p) as uint32_t) & !((2 << p) as u32 - 1)) != 0 {
return (a >> 31) as int32_t ^ ((1 << p) - 1);
} else {
return a;
}
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
let rounding = (1 as int64_t) << (shift - 1);
let mask = ((1 as int64_t) << (shift + 1)) - 1;
return ((value.wrapping_add(rounding)) >> shift).wrapping_sub((((value & mask) == rounding) as i64));
}

fn rshift64_clip24(value: int64_t, shift: u32) -> int32_t {
return clip_intp2(value as int32_t, 23);
}




use libc::int32_t;
use libc::uint32_t;

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
if ((a as u32).wrapping_add((1 as u32) << p) & !((2 as u32) << p - 1)) != 0 {
return (a >> 31) ^ ((1 as int32_t) << p - 1);
} else {
return a;
}
}

fn rshift32(value: int32_t, shift: u32) -> int32_t {
let rounding: int32_t = (1 as int32_t) << (shift - 1);
let mask: int32_t = ((1 as int32_t) << (shift + 1)) - 1;
let shifted = (value.wrapping_add(rounding)) >> shift;
let masked = value & mask;
let masked_rounding = masked == rounding as int32_t;
let result = shifted.wrapping_sub(masked_rounding as int32_t);
return result;
}

fn rshift32_clip24(value: int32_t, shift: u32) -> int32_t {
return clip_intp2(rshift32(value, shift) as int32_t, 23);
}


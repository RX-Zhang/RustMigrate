

use libc::int32_t;
use libc::uint32_t;

fn clip_intp2(mut a: int32_t, p: u32) -> int32_t {
let temp: uint32_t = (a as uint32_t).wrapping_add((1 << p) as uint32_t) & !((2 << p) - 1 as uint32_t);
if temp != 0 {
a = ((a >> 31) as int32_t) ^ ((1 << p) - 1);
}
a
}


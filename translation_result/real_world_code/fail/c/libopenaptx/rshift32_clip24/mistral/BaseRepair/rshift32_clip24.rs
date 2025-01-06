
use libc::uint32_t;
use std::i32;
use std::ops::BitOr;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask: i32 = (1 << (p % 32)) - 1;
    let shifted_a = (a as u32).wrapping_shr((32 - p) % 32) as i32;
    let clipped_a = shifted_a & mask;
    clipped_a as i32
}

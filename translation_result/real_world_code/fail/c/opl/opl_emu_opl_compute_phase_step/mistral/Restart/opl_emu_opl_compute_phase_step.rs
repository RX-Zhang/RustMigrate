
use libc::uint32_t;
use std::i32;
use std::ops::{Shr, Sub, Add, Mul};

fn opl_emu_bitfield(value: uint32_t, start: i32, length: i32) -> uint32_t {
    assert!(start >= 0 && length >= 0 && start + length <= 32,
        "Invalid bitfield range: start={}, length={}", start, length);
    let mask: uint32_t = (1 << length) - 1;
    (value >> start) & mask
}

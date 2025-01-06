
use libc::uint32_t;
use std::ops::BitAnd;
use std::ops::Shr;
use std::mem;
use std::usize;

pub fn rotl32(num: uint32_t, r: uint32_t) -> uint32_t {
 let shift_amount = (r % 32) as usize;
 let mask = (1 << shift_amount) - 1;
 let rotated_part = (num & mask).wrapping_shr((mem::size_of::<uint32_t>() * 8 - shift_amount) as u32);
 let remaining_part = num << shift_amount;
 return (rotated_part | remaining_part) & 0xFFFFFFFF;
}

pub fn rotr32(num: uint32_t, r: uint32_t) -> uint32_t {
 let shift_amount = (r % 32) as usize;
 let mask = (1 << shift_amount) - 1;
 let rotated_part = num.wrapping_shr((mem::size_of::<uint32_t>() * 8 - shift_amount) as u32);
 let remaining_part = num << shift_amount;
 return (rotated_part | remaining_part) & 0xFFFFFFFF;
}

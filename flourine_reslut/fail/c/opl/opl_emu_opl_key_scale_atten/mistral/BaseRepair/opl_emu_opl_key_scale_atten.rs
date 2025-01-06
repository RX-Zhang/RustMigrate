

use libc::{int32_t, c_uint, uint32_t};
use std::cmp::max;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

fn opl_emu_opl_key_scale_atten(block: uint32_t, fnum_4msb: uint32_t) -> uint32_t {
let result = (FNUM_TO_ATTEN[fnum_4msb as usize] as int32_t).wrapping_sub(8 * (block.wrapping_sub(7) as int32_t));
max(0, result) as c_uint
}


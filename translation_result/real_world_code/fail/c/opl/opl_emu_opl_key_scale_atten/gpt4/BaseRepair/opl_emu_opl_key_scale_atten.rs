
use std::cmp;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

pub fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_index = (fnum_4msb & 0xF) as usize; // Ensure index is within bounds
    let result: i32 = FNUM_TO_ATTEN[fnum_index] as i32 - 8 * ((block ^ 7) as i32);
    cmp::max(0, result) as u32
}

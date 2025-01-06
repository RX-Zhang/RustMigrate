
use std::cmp;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

pub fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let result = FNUM_TO_ATTEN[fnum_4msb as usize] as i32
        - (8 * ((block as i32).wrapping_sub(7) as u32)) as i32;
    cmp::max(0, result) as u32
}

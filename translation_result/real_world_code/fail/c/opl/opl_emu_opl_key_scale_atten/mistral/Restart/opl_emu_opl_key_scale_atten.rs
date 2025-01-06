
use std::u32;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let shifted_block = (block as i32) << 3;
    let raw_result = (FNUM_TO_ATTEN[fnum_4msb as usize] as i32) - shifted_block;
    let result = raw_result.max(0) as u32;
    u32::wrapping_add(u32::wrapping_neg(shifted_block as u32), result)
}

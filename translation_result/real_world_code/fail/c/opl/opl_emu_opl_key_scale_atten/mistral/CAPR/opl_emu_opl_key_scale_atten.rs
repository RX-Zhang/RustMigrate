
use std::u32;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let result = u32::wrapping_sub(
        FNUM_TO_ATTEN[fnum_4msb as usize] as u32,
        u32::wrapping_mul(8, block.wrapping_sub(7)),
    );
    result.saturating_sub(1)
}

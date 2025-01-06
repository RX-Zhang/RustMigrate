

use std::u32;

const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    if block > 7 || fnum_4msb > 15 {
        return 0;
    }
    let index = (block as u8).wrapping_add((((fnum_4msb & 0x0F) as u8) << 3)) as usize;
    FNUM_TO_ATTEN[index] as u32
}


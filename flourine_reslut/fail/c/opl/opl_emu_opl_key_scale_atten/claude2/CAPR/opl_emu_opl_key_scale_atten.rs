
use std::cmp::Ordering;

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    // this table uses the top 4 bits of FNUM and are the maximal values
    // (for when block == 7). Values for other blocks can be computed by
    // subtracting 8 for each block below 7.
    const FNUM_TO_ATTEN: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];
    
    let result = FNUM_TO_ATTEN[fnum_4msb as usize] as i32 - 8 * (block.wrapping_sub(7) as i32);
    
    match result.cmp(&0) {
        Ordering::Greater => result as u32,
        Ordering::Less => 0,
        Ordering::Equal => 0
    }
}

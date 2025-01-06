
use std::u8;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let masked_regbyte = *regbyte & 0x0f as u8;
    let shifted_masked_regbyte = (masked_regbyte as i32) << (8 - (4 as u8));
    *regbyte = (shifted_masked_regbyte.wrapping_add(*regbyte as i32) & 0xff as i32) as u8;
    *regbyte = (*regbyte as i32 + volume) as u8;
}

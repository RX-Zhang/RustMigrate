
use std::u8;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let masked_regbyte = !(*regbyte & 0x3F) & 0xFF; // Mask and keep the value within u8 range
    let level: u8 = ((masked_regbyte as u16 * volume as u16 / 127) as u8).wrapping_shr(6) & 0x3F; // Calculate level with wrapping operations and bitwise AND

    *regbyte = (*regbyte & 0xC0) | level; // Update the regbyte value
}

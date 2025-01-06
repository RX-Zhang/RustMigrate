
fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = !(*regbyte);
    level &= 0x3F;
    level = ((level as i32 * volume) / 127) as u8;
    if level > 0x3F {
        level = 0x3F;
    }
    if level < 0 {
        level = 0;
    }
    level = !level;
    level &= 0x3F;
    *regbyte &= 0xC0;
    *regbyte |= level;
}


fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level: i32;

    // invert bits and strip out the KSL header
    level = !(*regbyte as i32);
    level &= 0x3f;

    // adjust volume
    level = (level * volume) / 127;

    // boundaries check
    if level > 0x3f {
        level = 0x3f;
    }
    if level < 0 {
        level = 0;
    }

    // invert the bits, as expected by the OPL registers
    level = !level;
    level &= 0x3f;

    // final result computation
    *regbyte &= 0xC0; // zero out all attentuation bits
    *regbyte |= level as u8; // fill in the new attentuation value
}


use std::ops::BitXor;

fn calc_vol(regbyte: &mut Box<u8>, volume: i32) {
    let mut level: i32 = (!**regbyte).wrapping_add(0xC0) as i32;
    level &= 0x3f;

    level = (level * volume) / 127;

    if level > 0x3f {
        level = 0x3f;
    } else if level < 0 {
        level = 0;
    }

    level = (!level).wrapping_add(0xC0) as i32;
    level &= 0x3f;

    **regbyte &= 0xC0;
    **regbyte |= level as u8;
}

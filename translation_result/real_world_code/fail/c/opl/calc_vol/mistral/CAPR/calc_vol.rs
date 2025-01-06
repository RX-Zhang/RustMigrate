
use std::u8;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let level = match u8::wrapping_neg((*regbyte).into()) & 0x3F {
        0 => return,
        level => level,
    };

    let level = (level as i32 * volume / 127) as u8;

    if level > 0x3F {
        *regbyte &= 0xC0;
        *regbyte |= 0x3F;
    } else {
        *regbyte &= 0xC0;
        *regbyte |= level;
    }
}

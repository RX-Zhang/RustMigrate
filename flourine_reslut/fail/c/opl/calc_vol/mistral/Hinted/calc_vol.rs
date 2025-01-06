

use std::mem;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level: u8;
    /* invert bits and strip out the KSL header */
    let mut regbyte_copy = *regbyte;
    level = !regbyte_copy & 0x3f;

    /* adjust volume */
    let level_i32 = (level as i32) * (volume as i32);
    let level_u8 = (level_i32 / 127) as u8;
    if level_i32 < 0 || level_u8 as i32 != level_i32 / 127 {
        level = 0x3f;
    } else {
        level = level_u8;
    }

    *regbyte = level;
}


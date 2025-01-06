

use std::boxed::Box;
use std::mem;

static mut REGBYTE: u8 = 0;

fn calc_vol(volume: i32) {
let mut level_box = Box::new(0u8);

unsafe {
let level = !*level_box;
*level_box = (level.wrapping_add(128) & 0x3f).wrapping_neg();
}

let level = *level_box as i32 * volume as i32 / 127;
let level = level.clamp(0, 0x3f) as u8;

unsafe {
REGBYTE = (REGBYTE & 0xC0) | level;
}
}


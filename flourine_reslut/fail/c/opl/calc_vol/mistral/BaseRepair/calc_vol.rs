

use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};

static REGBYTE: AtomicUsize = AtomicUsize::new(0);

fn calc_vol(volume: i32) {
    let level = {
        let level = !REGBYTE.load(Ordering::Relaxed);
        level & 0x3f
    };

    let level = {
        let level = level as i32 * volume as i32 / 127;
        level.clamp(0, 0x3f) as u8
    };

    REGBYTE.store(level as usize, Ordering::Relaxed);
}


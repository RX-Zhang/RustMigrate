

use std::cmp::Ordering;

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten = Box::new([0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56]);

    let result = *fnum_to_atten.get((fnum_4msb & 0xf) as usize).unwrap()
        - 8 * block.wrapping_sub(7);
    
    match result.cmp(&0) {
        Ordering::Greater => result,
        Ordering::Less => 0,
        Ordering::Equal => 0
    }
}


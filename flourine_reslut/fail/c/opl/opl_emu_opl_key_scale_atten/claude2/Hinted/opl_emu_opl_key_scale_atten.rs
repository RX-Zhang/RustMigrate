
use std::cmp::Ordering;

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten: Box<[u8]> = Box::new([0,24,32,37,40,43,45,47,48,50,51,52,53,54,55,56]);
    
    let block = match block.cmp(&7) {
        Ordering::Less => 7 - block,
        Ordering::Greater => return 0,
        Ordering::Equal => 0,
    };
    
    let fnum_atten = *fnum_to_atten.get(fnum_4msb as usize).unwrap_or(&0);
    let result = fnum_atten.wrapping_sub((8 * block) as u8) as i32;
    
    if result < 0 {
        0
    } else {
        result as u32
    }
}


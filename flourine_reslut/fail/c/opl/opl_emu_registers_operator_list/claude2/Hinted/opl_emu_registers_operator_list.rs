
use std::convert::TryInto;

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let o1 = o1 as u32;
    let o2 = o2 as u32;
    let o3: u8 = o3;
    let o4: u8 = o4;
    
    o1 | o2.wrapping_shl(8) | (o3 as u32).wrapping_shl(16) | (o4 as u32).wrapping_shl(24) 
}

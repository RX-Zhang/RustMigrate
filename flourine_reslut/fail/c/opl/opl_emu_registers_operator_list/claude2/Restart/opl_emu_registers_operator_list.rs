
use std::convert::TryInto;

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let o2 = u32::from(o2);
    let o3 = u32::from(o3); 
    let o4 = u32::from(o4);
    
    let res = u32::from(o1);
    let res = res.wrapping_add(o2 << 8);
    let res = res.wrapping_add(o3 << 16);
    let res = res.wrapping_add(o4 << 24);
    res
}


use std::boxed::Box;
use std::mem;
use std::usize;

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let o1_shifted = (o1 as usize) << 24;
    let o2_shifted = (o2 as usize) << 16;
    let o3_shifted = (o3 as usize) << 8;

    let result = o1_shifted | o2_shifted | o3_shifted | (o4 as usize);

    result as u32
}


use std::boxed::Box;
use std::mem::size_of;

pub const MASK: u32 = 0xff;

pub fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let o1_boxed: Box<u32> = Box::new(o1 as u32 & MASK);
    let o2_boxed: Box<u32> = Box::new(o2 as u32 & MASK);
    let o3_boxed: Box<u32> = Box::new(o3 as u32 & MASK);
    let o4_boxed: Box<u32> = Box::new(o4 as u32 & MASK);

    let shifted_o2 = *o2_boxed << (size_of::<u8>() * 8);
    let shifted_o3 = *o3_boxed << (size_of::<u8>() * 16);
    let shifted_o4 = *o4_boxed << (size_of::<u8>() * 24);

    *o1_boxed | shifted_o2 | shifted_o3 | shifted_o4
}

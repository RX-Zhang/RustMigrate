
use std::u32;

fn opl_emu_registers_operator_list(o: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    return (o as u32)
        | (u32::from(o2) << 8)
        | (u32::from(o3) << 16)
        | (u32::from(o4) << 24);
}

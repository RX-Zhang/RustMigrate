
use std::u32;

fn opl_emu_s_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    (o1 as u32).wrapping_add(
        (u32::from(o2) << 8).wrapping_add(
            (u32::from(o3) << 16).wrapping_add(
                (u32::from(o4) << 24))))
}

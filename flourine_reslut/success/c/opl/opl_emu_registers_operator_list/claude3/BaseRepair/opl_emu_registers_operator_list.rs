
fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let o1 = o1 as u32;
    let o2 = (o2 as u32) << 8;
    let o3 = (o3 as u32) << 16;
    let o4 = (o4 as u32) << 24;

    o1.wrapping_add(o2)
        .wrapping_add(o3)
        .wrapping_add(o4)
}

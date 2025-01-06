
fn opl_emu_registers_byte(regs: &Box<[u8]>, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs[(offset + extra_offset) as usize].into(), start as i32, count as i32)
}

fn opl_emu_bitfield(val: u32, start: i32, count: i32) -> u32 {
    (val >> start) & ((1 << count) - 1)
}

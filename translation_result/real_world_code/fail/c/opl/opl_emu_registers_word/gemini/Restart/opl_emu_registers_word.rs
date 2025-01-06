
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shifted = (value >> start as u32) & ((1 << length) - 1);
    shifted
}

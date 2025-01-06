
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = ((1 << length) - 1) << start;
    (value & !mask) | ((value >> start) & ((1 << length) - 1)) << start
}


fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    if start < 0 || length <= 0 {
        return 0;
    }
    let start = start as u32;
    let length = length as u32;
    (value >> start) & ((1 << length) - 1)
}

fn main() {
    println!("{}", opl_emu_bitfield(4261416448, -184518411, -65536)); // Expected Output: 0
    println!("{}", opl_emu_bitfield(268499200, 64511, -134279156)); // Expected Output: 0
    println!("{}", opl_emu_bitfield(922694143, -536922579, 825308983)); // Expected Output: 112633
    println!("{}", opl_emu_bitfield(4294967295, 16714495, 184025087)); // Expected Output: 1
}

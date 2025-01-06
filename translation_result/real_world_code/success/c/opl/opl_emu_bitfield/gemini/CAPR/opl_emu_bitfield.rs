
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    // Ensure start and length are within valid ranges
    let start = start as u32;
    let length = length as u32;

    // Perform the bitfield extraction
    (value >> (start % 32)) & ((1 << (length % 32)) - 1)
}

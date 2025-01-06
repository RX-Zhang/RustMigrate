
pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mut start = start as u32 % 32;
    let mut length = length as u32 % 32;

    if length == 0 {
        return 0;
    }

    if length > 32 {
        length = 32;
    }

    if start + length > 32 {
        length = 32 - start;
    }

    (value >> start) & ((1 << length) - 1)
}

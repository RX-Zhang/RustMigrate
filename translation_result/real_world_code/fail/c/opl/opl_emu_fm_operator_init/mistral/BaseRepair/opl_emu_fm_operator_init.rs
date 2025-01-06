
fn wrapping_shr(a: u32, b: u8) -> u32 {
    a >> (b as u32)
}

fn bitwise_and(a: u8, b: u8) -> u8 {
    a & b
}

fn bitwise_or(a: u8, b: u8) -> u8 {
    a | b
}

fn bitwise_not(a: u8) -> u8 {
    !a
}

fn create_mask(width: u8) -> u16 {
    (1 << (width as u16)) - 1
}

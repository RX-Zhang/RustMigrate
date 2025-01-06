
fn create_mask(length: u32) -> u32 {
    let mask = if length == 32 {
        u32::MAX
    } else {
        ((1u32 << length) - 1)
    };
    mask
}

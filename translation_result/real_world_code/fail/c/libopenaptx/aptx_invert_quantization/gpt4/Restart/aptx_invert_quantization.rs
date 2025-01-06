
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    let shifted = value.wrapping_shr(shift);
    if shifted < -(1 << 24) {
        -(1 << 24)
    } else if shifted >= (1 << 24) {
        (1 << 24) - 1
    } else {
        shifted as i32
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    value.wrapping_shr(shift)
}

fn clip(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

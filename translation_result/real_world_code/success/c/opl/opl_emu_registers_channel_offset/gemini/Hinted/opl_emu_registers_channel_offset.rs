
// Including necessary standard library modules
use std::num::Wrapping;

// Translated function from C to Rust
fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    // Wrapping operations to handle the arithmetic
    let wrapping_addition = Wrapping(chnum % 9) + Wrapping(0x100) * Wrapping(chnum / 9);
    // Returning back the u32 value
    wrapping_addition.0
}

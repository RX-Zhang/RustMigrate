
use std::convert::TryInto;

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    (chnum % 9_u32).wrapping_add(
        (chnum.wrapping_div(9_u32)).wrapping_mul(0x100_u32)
    )
}

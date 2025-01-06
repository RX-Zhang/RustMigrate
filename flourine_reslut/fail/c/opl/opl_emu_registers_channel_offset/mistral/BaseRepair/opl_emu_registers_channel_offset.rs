

use libc::uint32_t;

fn opl_emu_registers_channel_offset(chnum: uint32_t) -> uint32_t {
    let chnum = chnum % 256u32;
    (chnum % 9) + (100 * (chnum / 9)).wrapping_add(0x100)
}


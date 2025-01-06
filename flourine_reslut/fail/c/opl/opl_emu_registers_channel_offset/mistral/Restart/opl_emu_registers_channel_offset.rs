

use libc::uint32_t;

fn opl_emu_registers_channel_offset(chnum: uint32_t) -> uint32_t {
let chnum = chnum % 16u32;
chnum.wrapping_add(100 * (chnum / 9))
}


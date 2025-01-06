
use std::convert::TryInto;

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let opnum = opnum.wrapping_rem(18);
    let offset = (opnum + opnum.wrapping_mul(2) / 6).wrapping_add(0x100 * (opnum / 18));
    offset
}

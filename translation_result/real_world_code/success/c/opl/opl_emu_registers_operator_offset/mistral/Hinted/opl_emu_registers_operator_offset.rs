
use std::u32;

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let mut rem = opnum % 18;
    let quot = (opnum / 18) as i32;
    rem = (rem + 2 * (rem / 6)) as u32;
    rem + u32::wrapping_mul(0x100u32, quot as u32)
}

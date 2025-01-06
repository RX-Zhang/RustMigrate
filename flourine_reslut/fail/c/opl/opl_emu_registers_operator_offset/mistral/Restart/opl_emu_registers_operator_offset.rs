
use std::mem;

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let mut result = opnum % 18;
    result += 2 * (result / 6);
    result += 0x100 * (opnum / 18);
    result
}

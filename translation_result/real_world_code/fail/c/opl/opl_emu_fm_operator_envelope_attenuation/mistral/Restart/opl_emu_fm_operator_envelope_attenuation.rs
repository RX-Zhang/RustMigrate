
use std::ops::Index;

#[repr(C)]
struct Register {
    value: u16,
}

struct OplEmus {
    reg: Box<[Register]>,
}

fn opl_emu_registers_byte(
    regs: &OplEmus,
    offset: u32,
    start: u32,
) -> u8 {
    let reg = &regs.reg[start as usize];
    let val = (reg.value >> (offset % 16)) as u8;
    val
}

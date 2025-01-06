
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let rem = opnum % 18;
    let div = rem / 6;
    rem + 2 * div + 0x100 * (opnum / 18)
}

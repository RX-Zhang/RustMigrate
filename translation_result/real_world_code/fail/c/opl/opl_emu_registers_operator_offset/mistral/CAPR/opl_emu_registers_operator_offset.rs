
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let rem6 = opnum % 6;
    let quot18 = (opnum + 5) / 18; // Round up by adding 5 and dividing by 18
    let shifted = ((rem6 as i32) + 2 * (rem6 / 6) as i32) as u32;
    shifted + quot18.wrapping_mul(0x100)
}

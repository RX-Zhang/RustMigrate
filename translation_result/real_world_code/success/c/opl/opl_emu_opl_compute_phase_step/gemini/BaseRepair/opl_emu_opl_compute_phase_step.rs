
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> (start as u32)) & ((1u32 << (length as u32)) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let mut fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;
    fnum = fnum.wrapping_add((((lfo_raw_pm as i64) * (opl_emu_bitfield(block_freq, 7, 3) as i64)) >> 1) as u32);
    fnum &= 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum << block).wrapping_shr(2);
    phase_step.wrapping_mul(multiple).wrapping_shr(1)
}

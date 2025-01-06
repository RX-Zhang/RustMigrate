
fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    value.wrapping_shr(start as u32) & ((1 << length) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let fnum = opl_emu_bitfield(block_freq, 0, 10).wrapping_shl(2);
    let fnum = fnum.wrapping_add(((lfo_raw_pm as u32 * opl_emu_bitfield(block_freq, 7, 3)) >> 1) & 0xfff);
    let block = opl_emu_bitfield(block_freq, 10, 3) as u32;
    let phase_step = (fnum << block) >> 2;
    (phase_step.wrapping_mul(multiple)) >> 1
}

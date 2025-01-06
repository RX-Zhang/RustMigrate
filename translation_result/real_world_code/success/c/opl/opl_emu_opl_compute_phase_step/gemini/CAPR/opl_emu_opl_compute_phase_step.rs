
fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value.wrapping_shr(start % 32)) & ((1u32.wrapping_shl(length % 32)).wrapping_sub(1))
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let mut fnum = opl_emu_bitfield(block_freq, 0, 10).wrapping_shl(2);
    fnum = fnum.wrapping_add(((lfo_raw_pm as u32).wrapping_mul(opl_emu_bitfield(block_freq, 7, 3))).wrapping_shr(1));
    fnum &= 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum.wrapping_shl(block % 32)).wrapping_shr(2);
    (phase_step.wrapping_mul(multiple)).wrapping_shr(1)
}

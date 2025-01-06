
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;
    let fnum = fnum.wrapping_add((lfo_raw_pm.wrapping_mul(opl_emu_bitfield(block_freq, 7, 3) as i32)) as u32 >> 1) & 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum << block) >> 2;
    (phase_step.wrapping_mul(multiple)) >> 1
}

fn main() {
    println!("{}", opl_emu_opl_compute_phase_step(981942538, 12033, 3080506));
    println!("{}", opl_emu_opl_compute_phase_step(167781377, 393472, 8388649));
    println!("{}", opl_emu_opl_compute_phase_step(859125028, 842545463, 607466032));
    println!("{}", opl_emu_opl_compute_phase_step(4294964794, 4294911760, -2603));
}

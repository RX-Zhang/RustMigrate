
use std::ops::{Add, Shl, Shr};

#[derive(Clone, Copy)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: usize,
    start: usize,
    count: usize,
    extra_offset: usize,
) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start, count)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr = (*noise_lfsr).wrapping_shl(1);
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 8, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 1, 1);

    let mut am_counter = *lfo_am_counter;
    *lfo_am_counter = am_counter.wrapping_add(1);
    if am_counter >= 210 * 64 - 1 {
        *lfo_am_counter = 0;
    }

    let shift = 9 - 2 * am_depth;

    *lfo_am = if am_counter < 105 * 64 {
        (am_counter >> shift) as u8
    } else {
        ((210 * 64 + 63 - am_counter) >> shift) as u8
    };

    let mut pm_counter = *lfo_pm_counter;
    *lfo_pm_counter = pm_counter.wrapping_add(1);

    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_value = pm_scale[opl_emu_bitfield(pm_counter.into(), 10, 3) as usize] >> (pm_depth ^ 1);
    pm_value as i32
}

fn opl_emu_registers_lfo_pm_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 6, 1, 0)
}

fn opl_emu_registers_lfo_am_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 7, 1, 0)
}

fn opl_emu_registers_clock_noise_and_lfo(regs: &mut OplEmuRegisters) -> i32 {
    let lfo_am_depth = opl_emu_registers_lfo_am_depth(regs);
    let lfo_pm_depth = opl_emu_registers_lfo_pm_depth(regs);
    opl_emu_opl_clock_noise_and_lfo(
        &mut regs.m_noise_lfsr,
        &mut regs.m_lfo_am_counter,
        &mut regs.m_lfo_pm_counter,
        &mut regs.m_lfo_am,
        lfo_am_depth,
        lfo_pm_depth,
    )
}

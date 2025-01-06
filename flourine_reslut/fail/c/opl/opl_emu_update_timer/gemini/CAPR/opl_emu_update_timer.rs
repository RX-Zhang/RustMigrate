

use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
}

struct OplEmu {
    env_counter: u32,
    status: u8,
    timer_running: [u8; 2],
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: [OplEmuFmChannel; 18],
    operator: [OplEmuFmOperator; 36],
}

fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.regdata[offset as usize + extra_offset as usize] as u32, start as usize, count as usize)
}

fn opl_emu_registers_timer_b_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8, 0)
}

fn opl_emu_registers_timer_a_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x02, 0, 8, 0) * 4
}

fn opl_emu_update_timer(emu: &mut OplEmu, tnum: u32, enable: bool) {
    if enable && emu.timer_running[tnum as usize] == 0 {
        let period = if tnum == 0 {
            1024 - opl_emu_registers_timer_a_value(&emu.regs)
        } else {
            16 * (256 - opl_emu_registers_timer_b_value(&emu.regs))
        };
        emu.timer_running[tnum as usize] = 1;
    } else if !enable {
        emu.timer_running[tnum as usize] = 0;
    }
}


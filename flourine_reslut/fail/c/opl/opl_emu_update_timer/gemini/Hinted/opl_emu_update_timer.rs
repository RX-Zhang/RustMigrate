
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
pub enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Copy, Clone)]
pub struct OplEmuRegisters {
    pub lfo_am_counter: u16,
    pub lfo_pm_counter: u16,
    pub noise_lfsr: u32,
    pub lfo_am: u8,
    pub regdata: [u8; 0x200],
    pub waveform: [[u16; 0x400]; 8],
}

#[derive(Copy, Clone)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Copy, Clone)]
pub struct OplEmuFmOperator {
    pub choffs: u32,
    pub opoffs: u32,
    pub phase: u32,
    pub env_attenuation: u16,
    pub env_state: OplEmuEnvelopeState,
    pub key_state: u8,
    pub keyon_live: u8,
    pub cache: OplEmuOpdataCache,
}

#[derive(Copy, Clone)]
pub struct OplEmuFmChannel {
    pub choffs: u32,
    pub feedback: [i16; 2],
    pub feedback_in: i16,
}

#[derive(Copy, Clone)]
pub struct OplEmu {
    pub env_counter: u32,
    pub status: u8,
    pub timer_running: [u8; 2],
    pub active_channels: u32,
    pub modified_channels: u32,
    pub prepare_count: u32,
    pub regs: OplEmuRegisters,
    pub channel: [OplEmuFmChannel; 18],
    pub operator: [OplEmuFmOperator; 36],
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

pub fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

pub fn opl_emu_registers_timer_b_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8, 0)
}

pub fn opl_emu_registers_timer_a_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x02, 0, 8, 0) * 4
}

pub fn opl_emu_update_timer(emu: &mut OplEmu, tnum: u32, enable: u32) {
    if enable != 0 && emu.timer_running[tnum as usize] == 0 {
        let period = if tnum == 0 {
            1024 - opl_emu_registers_timer_a_value(&emu.regs)
        } else {
            16 * (256 - opl_emu_registers_timer_b_value(&emu.regs))
        };
        emu.timer_running[tnum as usize] = 1;
    } else if enable == 0 {
        emu.timer_running[tnum as usize] = 0;
    }
}

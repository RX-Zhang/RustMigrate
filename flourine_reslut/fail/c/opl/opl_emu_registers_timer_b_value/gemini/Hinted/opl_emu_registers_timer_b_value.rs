
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters<const OPL_EMU_REGISTERS_WAVEFORMS: usize, const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize, const OPL_EMU_REGISTERS_REGISTERS: usize> {
    // internal state
    pub m_lfo_am_counter: u16,            // LFO AM counter
    pub m_lfo_pm_counter: u16,            // LFO PM counter
    pub m_noise_lfsr: u32,                // noise LFSR state
    pub m_lfo_am: u8,                     // current LFO AM value
    pub m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,         // register data
    pub m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>, // waveforms
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start as u32) & ((1 << length) - 1)
}

pub fn opl_emu_registers_byte<const OPL_EMU_REGISTERS_WAVEFORMS: usize, const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize, const OPL_EMU_REGISTERS_REGISTERS: usize>(regs: &mut OplEmuRegisters<OPL_EMU_REGISTERS_WAVEFORMS, OPL_EMU_REGISTERS_WAVEFORM_LENGTH, OPL_EMU_REGISTERS_REGISTERS>, offset: u32, start: u32, count: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize] as u32, start as i32, count as i32)
}

pub fn opl_emu_registers_timer_b_value<const OPL_EMU_REGISTERS_WAVEFORMS: usize, const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize, const OPL_EMU_REGISTERS_REGISTERS: usize>(regs: &mut OplEmuRegisters<OPL_EMU_REGISTERS_WAVEFORMS, OPL_EMU_REGISTERS_WAVEFORM_LENGTH, OPL_EMU_REGISTERS_REGISTERS>) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8)
}


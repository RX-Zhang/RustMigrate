
use std::mem::size_of;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    pub m_waveform: Box<[[u8; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

pub fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

pub fn opl_emu_registers_byte(
    regs: &mut OplEmuRegisters,
    offset: usize,
    start: usize,
    count: usize,
    extra_offset: usize,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[offset + extra_offset] as u32,
        start,
        count,
    )
}

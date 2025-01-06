
use std::mem;
use std::option;
use std::usize;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u32,
    m_lfo_pm_counter: u32,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
    let value = regs.m_regdata[offset + extra_offset] as u32;
    opl_emu_bitfield(value, start as u32, count as u32)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> bool {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0) != 0
}

fn opl_emu_registers_ch_output_1(regs: &OplEmuRegisters, choffs: usize) -> bool {
    if opl_emu_registers_newflag(regs) {
        (regs.m_regdata[0xc0 + choffs] as u32 & (1 << 5)) != 0
    } else {
        true
    }
}

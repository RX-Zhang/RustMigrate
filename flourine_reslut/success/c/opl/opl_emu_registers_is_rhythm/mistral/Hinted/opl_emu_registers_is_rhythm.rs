
use std::mem;
use std::usize;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS], // waveforms
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
    opl_emu_bitfield(
        u32::from(regs.m_regdata[offset + extra_offset]),
        start,
        count,
    )
}

fn opl_emu_registers_rhythm_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 5, 1, 0)
}

fn opl_emu_registers_is_rhythm(regs: &OplEmuRegisters, choffs: usize) -> bool {
    opl_emu_registers_rhythm_enable(regs) > 0 && (choffs >= 6 && choffs <= 8)
}

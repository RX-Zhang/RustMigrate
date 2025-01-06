
use std::ops::{BitAnd, BitOr, Shl, Shr};

#[derive(Debug)]
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

fn opl_emu_registers_op_sustain_level(regs: &OplEmuRegisters, opoffs: usize) -> u32 {
    opl_emu_registers_byte(regs, 0x80, 4, 4, opoffs)
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 20744,
        m_lfo_pm_counter: 146,
        m_noise_lfsr: 0,
        m_lfo_am: 127,
        m_regdata: [0; 0x200],
        m_waveform: [[0; 0x400]; 8],
    };
    let offset = 172;
    let result = opl_emu_registers_op_sustain_level(&regs, offset);
    println!("result: {}", result);
}


use std::ops::{BitAnd, BitOr, Shl};

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

fn opl_emu_registers_word(
    regs: &OplEmuRegisters,
    offset1: usize,
    start1: usize,
    count1: usize,
    offset2: usize,
    start2: usize,
    count2: usize,
    extra_offset: usize,
) -> u32 {
    (opl_emu_registers_byte(regs, offset1, start1, count1, extra_offset) << count2)
        | opl_emu_registers_byte(regs, offset2, start2, count2, extra_offset)
}

fn opl_emu_registers_ch_block_freq(regs: &OplEmuRegisters, choffs: usize) -> u32 {
    opl_emu_registers_word(regs, 0xb0, 0, 5, 0xa0, 0, 8, choffs)
}

fn main() {
    let regs = OplEmuRegisters {
        m_lfo_am_counter: 27127,
        m_lfo_pm_counter: 31479,
        m_noise_lfsr: 721420137,
        m_lfo_am: 255,
        m_regdata: [255; 0x200],
        m_waveform: [[65535; 0x400]; 8],
    };

    let offset: u32 = 3924421097;
    let result = opl_emu_registers_ch_block_freq(&regs, offset as usize);
    println!("{}", result);
}

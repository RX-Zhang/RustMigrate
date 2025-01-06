
use std::convert::TryInto;

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

fn opl_emu_registers_ch_feedback(regs: &OplEmuRegisters, choffs: usize) -> u32 {
    opl_emu_registers_byte(regs, 0xc0, 1, 3, choffs)
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 2570,
        m_lfo_pm_counter: 64764,
        m_noise_lfsr: 4244438268,
        m_lfo_am: 252,
        m_regdata: [252; 0x200],
        m_waveform: [[64764; 0x400]; 8],
    };

    let offset: usize = 0;
    let start: usize = 18;
    let count: usize = 32;
    let extra_offset: usize = 0;

    let result = opl_emu_registers_byte(&regs, offset, start, count, extra_offset);
    println!("result: {}", result);
}

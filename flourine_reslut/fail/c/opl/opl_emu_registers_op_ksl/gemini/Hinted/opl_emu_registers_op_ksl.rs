
use std::ops::BitAnd;

#[derive(Debug)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 512],
    m_waveform: [[u16; 1024]; 8],
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

fn opl_emu_registers_op_ksl(regs: &OplEmuRegisters, opoffs: usize) -> u32 {
    let temp = opl_emu_registers_byte(regs, 0x40, 6, 2, opoffs);
    opl_emu_bitfield(temp, 1, 1) | (opl_emu_bitfield(temp, 0, 1) << 1)
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 2624,
        m_lfo_pm_counter: 2624,
        m_noise_lfsr: 167772517,
        m_lfo_am: 0,
        m_regdata: [0; 512],
        m_waveform: [[0; 1024]; 8],
    };

    let offset = 14902075604643794638i128;
    let result = opl_emu_registers_op_ksl(&regs, offset as usize);
    println!("Result: {}", result);
}

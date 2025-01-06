
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Shl;
use std::ops::Shr;

#[derive(Debug)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Vec<u8>,
    m_waveform: Vec<Vec<u16>>,
}

impl OplEmuRegisters {
    fn op_lfo_pm_enable(&mut self, opoffs: u32) -> u32 {
        opl_emu_registers_byte(self, 0x20, 6, 1, opoffs)
    }
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &mut OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 65535,
        m_lfo_pm_counter: 63487,
        m_noise_lfsr: 3238002687,
        m_lfo_am: 53,
        m_regdata: vec![53; 508],
        m_waveform: vec![
            vec![0; 1020],
            vec![18761; 1020],
            vec![0; 1020],
            vec![0; 1020],
            vec![0; 1020],
            vec![0; 1020],
            vec![0; 1020],
            vec![0; 1020],
        ],
    };
    let opoffs: u32 = 0;
    let result = regs.op_lfo_pm_enable(opoffs);
    println!("{}", result);
}

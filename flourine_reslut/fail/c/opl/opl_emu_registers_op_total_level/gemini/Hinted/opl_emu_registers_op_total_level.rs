
use std::ops::{BitAnd, BitOr, Shl, Shr};

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
    fn bitfield(&self, value: u32, start: usize, length: usize) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn byte(&self, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
        self.bitfield(self.m_regdata[offset + extra_offset] as u32, start, count)
    }

    fn op_total_level(&self, opoffs: usize) -> u32 {
        self.byte(0x40, 0, 6, opoffs)
    }
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 65290,
        m_lfo_pm_counter: 65535,
        m_noise_lfsr: 4294933247,
        m_lfo_am: 255,
        m_regdata: vec![255; 512],
        m_waveform: vec![
            vec![0, 65535, 65535, 65535, 65535, 1020],
            vec![65535, 65535, 65535, 65535, 65535, 1020],
            vec![65535, 65535, 65535, 65535, 65535, 1020],
            vec![56157, 65535, 65535, 65535, 65535, 1020],
            vec![65535, 65535, 65535, 0, 0, 1020],
            vec![65535, 65535, 65535, 65535, 65535, 1020],
            vec![65535, 65535, 65535, 65535, 65535, 1020],
            vec![65535, 65535, 65535, 65535, 65535, 1020],
        ],
    };

    let offset = 253;
    let result = regs.op_total_level(offset);
    println!("Result: {}", result);
}



use std::ops::{BitAnd, BitOr, Shl, Shr};

#[derive(Debug)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 512],
    m_waveform: [[u16; 1024]; 8],
}

impl OplEmuRegisters {
    fn opl_emu_bitfield(&self, value: u32, start: u32, length: u32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_registers_byte(
        &self,
        offset: u32,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        self.opl_emu_bitfield(self.m_regdata[offset as usize + extra_offset as usize] as u32, start, count)
    }

    fn opl_emu_registers_op_eg_sustain(&self, opoffs: u32) -> u32 {
        self.opl_emu_registers_byte(0x20, 5, 1, opoffs)
    }
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 40606,
        m_lfo_pm_counter: 40606,
        m_noise_lfsr: 178167454,
        m_lfo_am: 236,
        m_regdata: [255; 512],
        m_waveform: [[65535; 1024]; 8],
    };

    let offset = 1583227998;
    let result = regs.opl_emu_registers_op_eg_sustain(offset);
    println!("Result: {}", result);
}


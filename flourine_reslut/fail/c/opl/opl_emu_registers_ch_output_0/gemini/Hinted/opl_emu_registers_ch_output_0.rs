
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

    fn opl_emu_registers_newflag(&self) -> u32 {
        self.opl_emu_registers_byte(0x105, 0, 1, 0)
    }

    fn opl_emu_registers_ch_output_0(&self, choffs: u32) -> u32 {
        if self.opl_emu_registers_newflag() != 0 {
            self.opl_emu_registers_byte(0xc0 + choffs, 4, 1, 0)
        } else {
            1
        }
    }
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 122,
        m_lfo_pm_counter: 31232,
        m_noise_lfsr: 4278190080,
        m_lfo_am: 37,
        m_regdata: [0; 0x200],
        m_waveform: [[0; 0x400]; 8],
    };

    let offset = 2021160960;

    println!("{}", regs.opl_emu_registers_ch_output_0(offset));
}

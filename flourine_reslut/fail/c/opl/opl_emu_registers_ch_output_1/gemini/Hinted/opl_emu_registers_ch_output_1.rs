
use std::ops::BitAnd;

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
        self.opl_emu_bitfield(
            self.m_regdata[offset as usize + extra_offset as usize] as u32,
            start,
            count,
        )
    }

    fn opl_emu_registers_newflag(&self) -> u32 {
        self.opl_emu_registers_byte(0x105, 0, 1, 0)
    }

    fn opl_emu_registers_ch_output_1(&self, choffs: u32) -> u32 {
        if self.opl_emu_registers_newflag() != 0 {
            self.opl_emu_registers_byte(0xc0 + choffs, 5, 1, 0)
        } else {
            1
        }
    }
}

fn main() {
    let regs = OplEmuRegisters {
        m_lfo_am_counter: 11,
        m_lfo_pm_counter: 0,
        m_noise_lfsr: 0,
        m_lfo_am: 0,
        m_regdata: vec![0; 512],
        m_waveform: vec![vec![0; 1024]; 8],
    };

    let offset = 3722304768;
    let result = regs.opl_emu_registers_ch_output_1(offset);
    println!("{}", result);
}

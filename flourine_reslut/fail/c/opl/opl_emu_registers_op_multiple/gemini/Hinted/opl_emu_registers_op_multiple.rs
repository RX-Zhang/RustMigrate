
use std::ops::BitAnd;

#[derive(Debug)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8]>,
    m_waveform: Box<[Box<[u16]>]>,
}

impl OplEmuRegisters {
    fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
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
            start as i32,
            count as i32,
        )
    }

    fn opl_emu_registers_op_multiple(&self, opoffs: u32) -> u32 {
        self.opl_emu_registers_byte(0x20, 0, 4, opoffs)
    }
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 10785,
        m_lfo_pm_counter: 514,
        m_noise_lfsr: 33686018,
        m_lfo_am: 2,
        m_regdata: Box::new([2; 512]),
        m_waveform: Box::new([
            Box::new([6939; 1024]),
            Box::new([60652; 1024]),
            Box::new([5140; 1024]),
            Box::new([6939; 1024]),
            Box::new([514; 1024]),
            Box::new([6954; 1024]),
            Box::new([60652; 1024]),
            Box::new([65535; 1024]),
            Box::new([5140; 1024]),
            Box::new([34695; 1024]),
        ]),
    };

    let opoffs = 3035753945046712320u64 as u32;
    let result = regs.opl_emu_registers_op_multiple(opoffs);
    println!("{}", result);
}


const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

impl OplEmuRegisters {
    fn new() -> Box<Self> {
        Box::new(Self {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
            m_waveform: [[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
        })
    }

    fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_registers_byte(&self, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
        Self::opl_emu_bitfield(self.m_regdata[(offset + extra_offset) as usize] as u32, start as i32, count as i32)
    }

    fn opl_emu_registers_word(&self, offset1: u32, start1: u32, count1: u32, offset2: u32, start2: u32, count2: u32, extra_offset: u32) -> u32 {
        (self.opl_emu_registers_byte(offset1, start1, count1, extra_offset) << count2) | self.opl_emu_registers_byte(offset2, start2, count2, extra_offset)
    }

    fn opl_emu_registers_ch_block_freq(&self, choffs: u32) -> u32 {
        self.opl_emu_registers_word(0xb0, 0, 5, 0xa0, 0, 8, choffs)
    }
}

fn main() {
    let regs = OplEmuRegisters::new();
    println!("{}", regs.opl_emu_registers_ch_block_freq(0));
}

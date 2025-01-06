
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

    fn opl_emu_registers_lfo_pm_depth(&self) -> u32 {
        self.opl_emu_registers_byte(0xbd, 6, 1, 0)
    }

    fn opl_emu_registers_lfo_am_depth(&self) -> u32 {
        self.opl_emu_registers_byte(0xbd, 7, 1, 0)
    }

    fn opl_emu_opl_clock_noise_and_lfo(noise_lfsr: &mut u32, lfo_am_counter: &mut u16, lfo_pm_counter: &mut u16, lfo_am: &mut u8, am_depth: u32, pm_depth: u32) -> i32 {
        *noise_lfsr = noise_lfsr.wrapping_shl(1) | (Self::opl_emu_bitfield(*noise_lfsr, 23, 1) ^ Self::opl_emu_bitfield(*noise_lfsr, 9, 1) ^ Self::opl_emu_bitfield(*noise_lfsr, 8, 1) ^ Self::opl_emu_bitfield(*noise_lfsr, 1, 1));
        let am_counter = *lfo_am_counter as u32;
        *lfo_am_counter = lfo_am_counter.wrapping_add(1);
        if am_counter >= 210 * 64 - 1 {
            *lfo_am_counter = 0;
        }
        let shift = 9 - 2 * am_depth;
        *lfo_am = (((if am_counter < 105 * 64 { am_counter } else { 210 * 64 + 63 - am_counter }) >> shift) & 0xFF) as u8;
        let pm_counter = *lfo_pm_counter as u32;
        *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);
        let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
        pm_scale[Self::opl_emu_bitfield(pm_counter, 10, 3) as usize] as i32 >> (pm_depth ^ 1)
    }

    fn opl_emu_registers_clock_noise_and_lfo(&mut self) -> i32 {
        let am_depth = self.opl_emu_registers_lfo_am_depth();
        let pm_depth = self.opl_emu_registers_lfo_pm_depth();
        Self::opl_emu_opl_clock_noise_and_lfo(&mut self.m_noise_lfsr, &mut self.m_lfo_am_counter, &mut self.m_lfo_pm_counter, &mut self.m_lfo_am, am_depth, pm_depth)
    }
}

fn main() {
    let mut regs = OplEmuRegisters::new();
    let _ = regs.opl_emu_registers_clock_noise_and_lfo();
}

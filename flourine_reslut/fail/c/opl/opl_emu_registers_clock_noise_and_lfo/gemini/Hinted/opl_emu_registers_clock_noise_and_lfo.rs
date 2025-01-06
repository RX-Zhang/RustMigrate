

#[derive(Debug)]
pub struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: Vec<u8>,         // register data
    m_waveform: Vec<Vec<u16>>, // waveforms
}

impl OplEmuRegisters {
    pub fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    pub fn opl_emu_registers_byte(
        &self,
        offset: usize,
        start: usize,
        count: usize,
        extra_offset: usize,
    ) -> u32 {
        Self::opl_emu_bitfield(self.m_regdata[offset + extra_offset] as u32, start, count)
    }

    pub fn opl_emu_opl_clock_noise_and_lfo(
        &mut self,
        am_depth: u32,
        pm_depth: u32,
    ) -> i32 {
        // OPL has a 23-bit noise generator for the rhythm section, running at
        // a constant rate, used only for percussion input
        self.m_noise_lfsr <<= 1;
        self.m_noise_lfsr |= Self::opl_emu_bitfield(self.m_noise_lfsr, 23, 1)
            ^ Self::opl_emu_bitfield(self.m_noise_lfsr, 9, 1)
            ^ Self::opl_emu_bitfield(self.m_noise_lfsr, 8, 1)
            ^ Self::opl_emu_bitfield(self.m_noise_lfsr, 1, 1);

        // OPL has two fixed-frequency LFOs, one for AM, one for PM

        // the AM LFO has 210*64 steps; at a nominal 50kHz output,
        // this equates to a period of 50000/(210*64) = 3.72Hz
        let mut am_counter = self.m_lfo_am_counter;
        am_counter = am_counter.wrapping_add(1);
        if am_counter >= 210 * 64 - 1 {
            self.m_lfo_am_counter = 0;
        } else {
            self.m_lfo_am_counter = am_counter;
        }

        // low 8 bits are fractional; depth 0 is divided by 2, while depth 1 is times 2
        let shift = 9 - 2 * am_depth;

        // AM value is the upper bits of the value, inverted across the midpoint
        // to produce a triangle
        self.m_lfo_am = if am_counter < 105 * 64 {
            (am_counter >> shift) as u8
        } else {
            ((210 * 64 + 63 - am_counter) >> shift) as u8
        };

        // the PM LFO has 8192 steps, or a nominal period of 6.1Hz
        let mut pm_counter = self.m_lfo_pm_counter;
        pm_counter = pm_counter.wrapping_add(1);

        // PM LFO is broken into 8 chunks, each lasting 1024 steps; the PM value
        // depends on the upper bits of FNUM, so this value is a fraction and
        // sign to apply to that value, as a 1.3 value
        let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
        let pm_scale_index = Self::opl_emu_bitfield(pm_counter as u32, 10, 3) as u32;
        let pm_scale_value = pm_scale[pm_scale_index as usize];
        pm_scale_value >> (pm_depth ^ 1)
    }

    pub fn opl_emu_registers_lfo_pm_depth(&self) -> u32 {
        self.opl_emu_registers_byte(0xbd, 6, 1, 0)
    }

    pub fn opl_emu_registers_lfo_am_depth(&self) -> u32 {
        self.opl_emu_registers_byte(0xbd, 7, 1, 0)
    }

    pub fn opl_emu_registers_clock_noise_and_lfo(&mut self) -> i32 {
        self.opl_emu_opl_clock_noise_and_lfo(
            self.opl_emu_registers_lfo_am_depth(),
            self.opl_emu_registers_lfo_pm_depth(),
        )
    }
}


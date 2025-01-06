
use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u32,
    m_lfo_pm_counter: u32,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

impl OplEmuRegisters {
    fn opl_emu_bitfield(&self, value: u32, start: u32, length: u32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_registers_byte(
        &self,
        offset: usize,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        self.opl_emu_bitfield(
            u32::from(self.m_regdata[offset + extra_offset as usize]),
            start,
            count,
        )
    }

    fn opl_emu_registers_newflag(&self) -> bool {
        self.opl_emu_registers_byte(105, 0, 1, 0) != 0
    }

    fn opl_emu_registers_ch_output_2(&self, choffs: usize) -> u32 {
        if self.opl_emu_registers_newflag() {
            self.opl_emu_registers_byte(128 + choffs, 6, 1, 0)
        } else {
            0
        }
    }
}

fn main() {
    // Example usage
    let regs = OplEmuRegisters {
        m_lfo_am_counter: u32::MAX,
        m_lfo_pm_counter: 4009754623,
        m_noise_lfsr: 4003393262,
        m_lfo_am: 238,
        m_regdata: [
            238;
            OPL_EMU_REGISTERS_REGISTERS
        ],
        m_waveform: [
            [
                16639;
                OPL_EMU_REGISTERS_WAVEFORM_LENGTH
            ];
            OPL_EMU_REGISTERS_WAVEFORMS
        ],
    };

    let choffs = 0;
    let result = regs.opl_emu_registers_ch_output_2(choffs);
    println!("{}", result);
}

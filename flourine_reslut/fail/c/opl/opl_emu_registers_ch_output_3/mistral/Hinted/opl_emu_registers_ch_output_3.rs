

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
    m_regdata: [u32; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

impl OplEmuRegisters {
    fn opl_emu_bitfield(&self, value: u32, start: u32, length: u32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_registers_byte(&self, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
        let real_offset = offset as usize + extra_offset as usize;
        let reg_value = self.m_regdata[real_offset];
        self.opl_emu_bitfield(reg_value, start, count)
    }

    fn opl_emu_registers_newflag(&self) -> bool {
        self.opl_emu_registers_byte(0x105, 0, 1, 0) != 0
    }

    fn opl_emu_registers_ch_output_3(&self, choffs: u32) -> u32 {
        if self.opl_emu_registers_newflag() {
            self.opl_emu_registers_byte(0xc0 + choffs, 7, 1, 0)
        } else {
            0
        }
    }
}

fn main() {
    // Example usage
    let regs = OplEmuRegisters {
        m_lfo_am_counter: 4292355765,
        m_lfo_pm_counter: 65535,
        m_noise_lfsr: 4294967295,
        m_lfo_am: 255,
        m_regdata: [
            255;
            OPL_EMU_REGISTERS_REGISTERS
        ],
        m_waveform: [
            [
                25700;
                OPL_EMU_REGISTERS_WAVEFORM_LENGTH
            ];
            OPL_EMU_REGISTERS_WAVEFORMS
        ],
    };

    let choffs = 0;
    let result = regs.opl_emu_registers_ch_output_3(choffs);
    println!("Result: {}", result);
}

